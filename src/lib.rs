#![allow(unused)]

mod board;
mod player;
mod properties;
mod utils;

use board::{BoardCell, BOARD, BOARD_SIZE};
use player::Player;
use properties::{Property, PropertyColor, PropertyState, PROPERTIES};
use rand::Rng;
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

    /// Starts and runs/plays the game.
    fn run(&mut self) {
        self.determine_player_order();

        // Main game loop
        let mut game_over = false;
        while game_over == false {
            let mut auction: Option<&mut Property> = None;
            for curr_player in 0..self.players.len() {
                // Roll die to determine next location of player
                self.players[curr_player].last_dice = Self::roll_dice();

                // Move to rolled location
                let new_position = Self::board_index_from_dice(
                    self.players[curr_player].position,
                    &self.players[curr_player].last_dice,
                );
                self.players[curr_player].position = new_position;

                // Determine type of location and take possible actions
                match self.board[new_position] {
                    BoardCell::Go => {
                        self.players[curr_player].money += Money(200);
                    }

                    BoardCell::Property(prop_idx) => {
                        let property = &mut self.properties[prop_idx];

                        match property.state {
                            PropertyState::NotBought => {
                                let player = &mut self.players[curr_player];

                                // Buy the property if player has enough money
                                if player.money >= property.price {
                                    player.properties.push(prop_idx);
                                    player.money -= property.price;
                                    property.state = PropertyState::Bought(curr_player);
                                } else {
                                    // TODO: Handle if not enough money (auction)
                                }
                            }

                            PropertyState::Bought(owner) => {
                                // If the player owns this property, do nothing
                                if owner != curr_player {
                                    let owner = unsafe {
                                        let ptr = self.players.as_mut_ptr();
                                        ptr.add(owner).as_mut().expect("Failed to access owner")
                                    };
                                    let rent = property.rent[property.rent_state as usize];

                                    // Pay rent to owning player
                                    if self.players[curr_player].money >= rent {
                                        owner.money += rent;
                                        self.players[curr_player].money -= rent;
                                    } else {
                                        // TODO: Handle if not enough money (auction)/sell/trade/mortgage properties
                                    }
                                }
                            }

                            // Do nothing if property is mortgaged
                            PropertyState::Mortgaged => {}
                        }
                    }

                    BoardCell::CommunityChest => todo!(), // TODO: RNG from set list
                    BoardCell::Chance => todo!(),         // TODO: RNG from set list

                    BoardCell::VisitingJail => {}
                    BoardCell::Jail => {
                        // TODO:
                        //  - Player will have to roll double or pay 50 to get out of jail
                        //  - Add jail counter to each player, along with an "in_jail" marker
                        //      - Check that before changing the player's position above
                    }
                    BoardCell::GoToJail => {
                        // TODO: Player position set to jail position
                    }

                    BoardCell::FreeParking(money) => {
                        let player = &mut self.players[curr_player];

                        // Player receives the money stored in free parking
                        if money.0 != 0 {
                            player.money += money;
                        }
                    }

                    BoardCell::IncomeTax(money) => {
                        // TODO: Move player's money to free parking
                        //  - Handle when player has no money!
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
