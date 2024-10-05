#![allow(unused)]

use rand::Rng;

const PROPERTIES: [Property; 28] = [Property::NotBought(PropertyInner {
    color: todo!(),
    price: todo!(),
    mortgage: todo!(),
    building_cost: todo!(),
    rent: todo!(),
}); 28];

/// Total number of houses in the game.
const NUM_HOUSES: u8 = 32;

/// Total number of hotels in the game.
const NUM_HOTLES: u8 = 12;

/// Represents a six-sided die.
#[derive(Debug)]
struct D6(u8);

impl D6 {
    /// Creates a six-sided die.
    fn new(value: u8) -> Self {
        assert!(
            value <= 6 && value >= 1,
            "The six-sided die must have a value between 1 and 6."
        );
        Self(value)
    }
}

/// Represents money and prices.
#[derive(Debug, Clone, Copy)]
struct Money(usize);

/// A player that makes actions in the game.
#[derive(Debug)]
struct Player<'a> {
    /// Player's name/id.
    name: String,

    /// The properties owned by the player.
    properties: Vec<&'a Property>,

    /// The amount of money the player has.
    money: Money,

    /// The value of the last dice rolled by the player.
    last_dice: (D6, D6),
}

impl<'a> Player<'a> {
    fn new(name: String) -> Self {
        Self {
            name,
            properties: vec![],
            money: Money(1500),
            last_dice: (D6(1), D6(1)),
        }
    }
}

/// Contains the game state and logic.
#[derive(Debug)]
struct Game<'a> {
    /// All the properties in the game.
    properties: [Property; 28],

    /// All players in the game.
    players: Vec<Player<'a>>,
}

impl<'a> Game<'a> {
    /// Initializes a new game.
    fn new(players: Vec<Player<'a>>) -> Self {
        Self {
            players,
            properties: PROPERTIES,
        }
    }

    /// Starts the game.
    fn start(&mut self) {
        self.determine_player_order();

        // Main game loop
        let mut game_over = false;
        while game_over == false {
            for player in &mut self.players {
                // Roll die to determine next location of player
                player.last_dice = Self::roll_dice();

                // TODO: Take possible actions
                //  - Move to rolled location
                //  - Determine status of the property
                //      - Buy if `NotBought`
                //          - Auction if not enough money
                //      - Nothing if `Mortgaged`
                //      - Pay rent if `Bought`

                // TODO: Reroll if double was rolled
            }
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
}

/// A property that can be owned by players.
#[derive(Debug, Clone, Copy)]
enum Property {
    /// The initial state of all properties; all properties can be bought at the start of the
    /// game.
    NotBought(PropertyInner),

    /// The property is bought and owned by a player.
    Bought(PropertyInner),

    /// The property is mortgaged to the bank.
    ///
    /// The owning player can't collect rent on a mortgaged property.
    Mortgaged(PropertyInner),

    /// The property is up for auction and the players must bid on it; the highest bidder buys the
    /// property from the bank.
    Auctioned(PropertyInner),
}

/// Contains the various prices (cost, rent, mortgage) and the color of a property.
#[derive(Debug, Clone, Copy)]
struct PropertyInner {
    /// The color/group/type of the property (i.e. orange, utility, etc.).
    color: PropertyColor,

    /// The amount a player must pay to buy the property from the bank.
    price: Money,

    /// The amount received when the player mortgages the property.
    ///
    /// # Note
    /// The property must be "unimproved"; all buildings on all properties of the matching color
    /// must be sold to the bank first (at half price).
    mortgage: Money,

    /// Cost for each building on the property (house, hotel).
    building_cost: Money,

    /// The amount a player receives if another player land on this property.
    ///
    /// # Note
    /// The array is laid out in the following order:
    /// `Base, 1 House, 2 House, 3 House, 4 House, Hotel`
    rent: [Money; 6],
}

/// The various types/colors of properties.
#[derive(Debug, Clone, Copy)]
enum PropertyColor {
    Brown,
    LightBlue,
    Pink,
    Orange,
    Red,
    Yellow,
    Green,
    DarkBlue,
    Railroad,
    Utility,
}
