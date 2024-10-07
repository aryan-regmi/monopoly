#![allow(unused)]

mod board;
mod player;
mod properties;
mod utils;

use board::{BoardCell, BOARD, BOARD_SIZE};
use player::Player;
use properties::{Property, PropertyColor, PropertyState, RentType, PROPERTIES};
use rand::Rng;
use tracing::{info, instrument, level_filters::LevelFilter, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer, Registry,
};
use utils::{Money, D6};

/// Total number of houses in the game.
const NUM_HOUSES: usize = 32;

/// Total number of hotels in the game.
const NUM_HOTLES: usize = 12;

/// Total number of properties in the game.
const NUM_PROPERTIES: usize = 28;

/// Contains the game state and logic.
#[derive(Debug)]
struct Game<'a> {
    /// All the properties in the game.
    properties: [Property<'a>; NUM_PROPERTIES],

    /// All players in the game.
    players: Vec<Player>,

    /// Represents the game board.
    ///
    /// 41 cells for each position in the game.
    board: [BoardCell; BOARD_SIZE],
}

impl<'a> Game<'a> {
    /// Initializes a new game.
    fn new(players: Vec<Player>) -> Self {
        Self {
            players,
            properties: PROPERTIES,
            board: BOARD,
        }
    }

    #[instrument]
    /// Starts and runs/plays the game.
    fn run(&mut self) {
        info!("Rolling for player order..");
        self.determine_player_order();

        // Main game loop
        info!("Starting game...");
        let mut game_over = false;
        while game_over == false {
            for curr_player in 0..self.players.len() {
                // Roll die to determine next location of player
                self.get_player_mut(curr_player).last_dice = Self::roll_dice();
                info!(
                    "Player {} rolled {:?}",
                    curr_player + 1,
                    self.get_player(curr_player).last_dice
                );

                // Move to rolled location
                let new_position = Self::board_index_from_dice(
                    self.get_player(curr_player).position,
                    &self.get_player(curr_player).last_dice,
                );
                self.get_player_mut(curr_player).position = new_position;

                // Determine type of location and take possible actions
                match self.board[new_position] {
                    BoardCell::Go => {
                        self.get_player_mut(curr_player).money += Money(200);
                        info!("Player {} landed on Go", curr_player + 1);
                    }

                    BoardCell::Property(prop_idx) => {
                        let property = &mut self.properties[prop_idx];
                        info!("Player {} landed on {}", curr_player + 1, property.name);

                        match property.state {
                            PropertyState::NotBought => {
                                info!("{} is on sale for ${}", property.name, property.price.0);
                                let player = &mut self.players[curr_player];

                                // Buy the property if player has enough money
                                if player.money >= property.price {
                                    player.properties.push(prop_idx);
                                    player.money -= property.price;
                                    property.state = PropertyState::Bought(curr_player);
                                    info!(
                                        "Player {} bought {} for ${}",
                                        curr_player + 1,
                                        property.name,
                                        property.price.0
                                    );
                                } else {
                                    // Run auction if the player can't afford the property
                                    let name = property.name;
                                    info!("Player {} doesn't have enough money to buy {}; the bank will now auction it!",
                                        curr_player + 1,
                                        name);
                                    let (player, money) = self.run_auction(prop_idx);
                                    let player = player.expect("Invalid player");
                                    self.get_player_mut(player).money -= money;
                                    self.get_player_mut(player).properties.push(prop_idx);
                                    self.properties[prop_idx].state = PropertyState::Bought(player);
                                    info!(
                                        "Player {} won the auction and bought {} for ${}",
                                        player + 1,
                                        name,
                                        money.0
                                    );
                                }
                            }

                            PropertyState::Bought(owner) => {
                                // If the player owns this property, do nothing
                                if owner != curr_player {
                                    let owner_idx = owner;
                                    info!(
                                        "Player {} owns {}: player {} must pay them rent",
                                        owner + 1,
                                        property.name,
                                        curr_player + 1
                                    );

                                    let owner = unsafe {
                                        let ptr = self.players.as_mut_ptr();
                                        ptr.add(owner).as_mut().expect("Failed to access owner")
                                    };
                                    let rent = property.rent[property.rent_state as usize];

                                    // Pay rent to owning player
                                    if self.players[curr_player].money >= rent {
                                        owner.money += rent;
                                        self.get_player_mut(curr_player).money -= rent;
                                        info!(
                                            "Player {} paid player {} ${} for rent",
                                            curr_player + 1,
                                            owner_idx + 1,
                                            rent.0
                                        );
                                    } else {
                                        info!(
                                            "Player {} doesn't have enough money to pay the rent.",
                                            curr_player + 1
                                        );
                                        let len = self.players[curr_player].properties.len();
                                        if self.players[curr_player].properties.len() != 0 {
                                            loop {
                                                for prop_idx in 0..len {
                                                    // Sell a house if there is one
                                                    let has_houses = self.properties[prop_idx]
                                                        .rent_state
                                                        .has_houses();
                                                    let is_mortgaged = self.properties[prop_idx]
                                                        .state
                                                        == PropertyState::Mortgaged;
                                                    if has_houses && !is_mortgaged {
                                                        let building_sell_price = self.properties
                                                            [prop_idx]
                                                            .building_cost
                                                            .0
                                                            / 2;
                                                        info!("Player {} sold a building on {} for {}",
                                                            curr_player + 1,
                                                            self.properties[prop_idx].name,
                                                            building_sell_price
                                                        );
                                                        self.properties[prop_idx]
                                                            .rent_state
                                                            .downgrade();
                                                        self.players[curr_player].money +=
                                                            Money(building_sell_price);
                                                    } else if !is_mortgaged {
                                                        // Mortgage property if no houses
                                                        let mortgage =
                                                            self.properties[prop_idx].mortgage;
                                                        self.properties[prop_idx].state =
                                                            PropertyState::Mortgaged;
                                                        self.players[curr_player].money += mortgage;
                                                        info!(
                                                            "Player {} mortgaged {} for {}",
                                                            curr_player + 1,
                                                            self.properties[prop_idx].name,
                                                            mortgage.0
                                                        );
                                                    } else {
                                                        continue;
                                                    }
                                                }

                                                // Stop if all buildings have been sold and properties have been mortgaged
                                                if let Some(_) = self.players[curr_player]
                                                    .properties
                                                    .iter()
                                                    .find(|p| {
                                                        self.properties[**p].rent_state.has_houses()
                                                    })
                                                {
                                                    continue;
                                                } else {
                                                    break;
                                                }
                                            }
                                        }
                                    }

                                    if self.players[curr_player].money >= rent {
                                        owner.money += rent;
                                        self.get_player_mut(curr_player).money -= rent;
                                        info!(
                                            "Player {} paid player {} ${} for rent",
                                            curr_player + 1,
                                            owner_idx + 1,
                                            rent.0
                                        );
                                    } else {
                                        // TODO: Declare bankrupcy!
                                        //   - All buildings must be sold to the bank
                                        //   - If bankrupt to player: give them all assests
                                        //   - If bankrupt to bank: auction all their assests
                                        let bankrupt_player = false;
                                        let bankrupter = if bankrupt_player {
                                            format!("Player {}", owner_idx + 1)
                                        } else {
                                            "the bank".into()
                                        };
                                        info!(
                                            "Player {} declared bankrupcy! All of their property will go to {}",
                                            curr_player + 1,
                                            bankrupter
                                        )
                                    }
                                }
                            }

                            // Do nothing if property is mortgaged
                            PropertyState::Mortgaged => {
                                info!("{} is mortgaged: no rent is due", property.name);
                            }
                        }
                    }

                    // TODO: RNG from set list
                    BoardCell::CommunityChest => {
                        info!("Player {} landed on Community Chest", curr_player + 1);
                        // TODO: Log CommunityChest card description
                    }
                    // TODO: RNG from set list
                    BoardCell::Chance => {
                        info!("Player {} landed on Chance", curr_player + 1);
                        // TODO: Log Chance card description
                    }

                    BoardCell::VisitingJail => info!("Player {} is visiting jail", curr_player + 1),
                    BoardCell::Jail => {
                        // TODO:
                        //  - Player will have to roll double or pay 50 to get out of jail
                        //  - Add jail counter to each player, along with an "in_jail" marker
                        //      - Check that before changing the player's position above
                        info!("Player {} is in jail", curr_player + 1);
                    }
                    BoardCell::GoToJail => {
                        // TODO: Player position set to jail position
                        info!("Player {} is sent to jail; do not pass Go", curr_player + 1);
                    }

                    BoardCell::FreeParking(mut money) => {
                        let player = self.get_player_mut(curr_player);

                        // Player receives the money stored in free parking
                        if money.0 != 0 {
                            info!(
                                "Player {} landed on free parking and received ${}",
                                curr_player + 1,
                                money.0
                            );
                            player.money += money;
                            money = Money(0);
                        }
                    }

                    BoardCell::IncomeTax(tax) => {
                        let player = self.get_player_mut(curr_player);

                        if player.money >= tax {
                            info!(
                                "Player {} landed on income tax and must pay {} to the bank",
                                curr_player + 1,
                                tax.0
                            );
                            player.money -= tax;
                            // TODO: Move player's money to free parking
                            // - Set self.board[FREE_PARKING].money += tax
                            // info!("Free Parking now has ${}", self.board[FREE_PARKING].money);
                        } else {
                            // Handle when player has no money! (mortgage, sell houses, trade)
                            info!(
                                "Player {} doesn't have enough money to pay the tax.",
                                curr_player + 1
                            );
                            let len = self.players[curr_player].properties.len();
                            if self.players[curr_player].properties.len() != 0 {
                                loop {
                                    for prop_idx in 0..len {
                                        // Sell a house if there is one
                                        let has_houses =
                                            self.properties[prop_idx].rent_state.has_houses();
                                        let is_mortgaged = self.properties[prop_idx].state
                                            == PropertyState::Mortgaged;
                                        if has_houses && !is_mortgaged {
                                            let building_sell_price =
                                                self.properties[prop_idx].building_cost.0 / 2;
                                            info!(
                                                "Player {} sold a building on {} for {}",
                                                curr_player + 1,
                                                self.properties[prop_idx].name,
                                                building_sell_price
                                            );
                                            self.properties[prop_idx].rent_state.downgrade();
                                            self.players[curr_player].money +=
                                                Money(building_sell_price);
                                        } else if !is_mortgaged {
                                            // Mortgage property if no houses
                                            let mortgage = self.properties[prop_idx].mortgage;
                                            self.properties[prop_idx].state =
                                                PropertyState::Mortgaged;
                                            self.players[curr_player].money += mortgage;
                                            info!(
                                                "Player {} mortgaged {} for {}",
                                                curr_player + 1,
                                                self.properties[prop_idx].name,
                                                mortgage.0
                                            );
                                        } else {
                                            continue;
                                        }
                                    }

                                    // Stop if all buildings have been sold and properties have been mortgaged
                                    if let Some(_) = self.players[curr_player]
                                        .properties
                                        .iter()
                                        .find(|p| self.properties[**p].rent_state.has_houses())
                                    {
                                        continue;
                                    } else {
                                        break;
                                    }
                                }
                            }

                            if self.players[curr_player].money >= tax {
                                info!("Player {} paid income tax of ${}", curr_player + 1, tax.0);
                            } else {
                                // TODO: Declare bankrupcy!
                                //   - All buildings must be sold to the bank
                                //   - If bankrupt to player: give them all assests
                                //   - If bankrupt to bank: auction all their assests
                                info!(
                                            "Player {} declared bankrupcy! All of their property will go to the bank",
                                            curr_player + 1,
                                        )
                            }

                            // TODO: Trading
                        }
                    }
                }

                // TODO: Reroll if double was rolled
                //          - 3 doubles in a row = jail

                // TODO: Implement Trading
                //  - Add a likely-to-trade parameter for each player
                //      - Has a value for each other player
                //      - Depends on number of open properties (NotBought)
                //      - Depends on what groups each player owns
                //      - Depends on how much money the player has
            }

            // TODO: Handle auctions
            //      - Loop thru all players
            //      - Make bids for each
            //      - Give to the highest bidder @ the end
            //          - Each player has option to drop out of bidding

            // TODO: Check if all other players are bankrupt and end the game
        }
    }

    /// Runs an auction by making each player bid on the property (starting from $10).
    ///
    /// The last player remaning will pay the bank the amount bid in return for the property.
    fn run_auction(&mut self, property_idx: usize) -> (Option<usize>, Money) {
        let property = &mut self.properties[property_idx];
        let mut price = Money(10); // Start bid at $10
        let mut bid = (None, price);
        let mut stop = false;
        let num_players = self.players.len();
        let mut dropped = Vec::with_capacity(num_players);
        while stop == false {
            for (i, player) in self.players.iter_mut().enumerate() {
                if dropped.contains(&i) {
                    continue; // Ignore players who dropped out from the auction
                }

                // NOTE: Use a different critera (like ratio of cost to investment for this later?)
                if player.money + 10 >= price {
                    price += Money(10);
                    bid = (Some(i), price)
                } else {
                    dropped.push(i);
                }

                // End auction if only 1 player remaning
                if dropped.len() >= num_players - 1 {
                    stop = true;
                }
            }
        }
        return bid;
    }

    fn get_player(&self, idx: usize) -> &Player {
        &self.players[idx]
    }

    fn get_player_mut(&mut self, idx: usize) -> &mut Player {
        &mut self.players[idx]
    }

    fn get_property(&'a self, idx: usize) -> &Property {
        &self.properties[idx]
    }

    fn get_property_mut(&'a mut self, idx: usize) -> &mut Property {
        &mut self.properties[idx]
    }

    #[instrument]
    /// Determines the order of the players from highest to lowest initial rolls.
    fn determine_player_order(&mut self) {
        // Determine dice rolls
        for player in &mut self.players {
            player.last_dice = Self::roll_dice();
        }

        // Order `players` vector
        self.players.sort_by(|a, b| {
            let total1 = Self::total_dice(&a.last_dice);
            let total2 = Self::total_dice(&a.last_dice);
            total2.cmp(&total1)
        });
        info!("Roll order: {:?}", self.players);
    }

    /// Rolls two six-sided dice.
    fn roll_dice() -> (D6, D6) {
        let die1 = rand::thread_rng().gen_range((1..=6));
        let die2 = rand::thread_rng().gen_range((1..=6));
        (D6(die1), D6(die2))
    }

    /// Gets the total dice amount from a set of dice rolls.
    fn total_dice(dice: &(D6, D6)) -> u8 {
        dice.0 .0 + dice.1 .0
    }

    /// Check is a double was rolled (same value on both die).
    fn dice_is_double(dice: &(D6, D6)) -> bool {
        dice.0 .0 == dice.1 .0
    }

    /// Gets the index in the `board` array from a given dice roll.
    ///
    /// # Note
    /// `curr_idx` is the current position of the **player**.
    fn board_index_from_dice(curr_position: usize, dice: &(D6, D6)) -> usize {
        // Add dice total to curr_position, clamped by total number of cells in the board
        let dice_total = Self::total_dice(dice);
        let mut next_pos = curr_position + dice_total as usize;
        if next_pos >= BOARD_SIZE {
            next_pos = 0;
        }
        next_pos
    }
}
