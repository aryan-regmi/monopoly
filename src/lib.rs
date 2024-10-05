#![allow(unused)]

mod player;
mod properties;
mod utils;

use player::Player;
use properties::{Property, PropertyColor, PropertyInner, PROPERTIES};
use rand::Rng;
use utils::{Money, D6};

/// Total number of houses in the game.
const NUM_HOUSES: u8 = 32;

/// Total number of hotels in the game.
const NUM_HOTLES: u8 = 12;

/// Contains the game state and logic.
#[derive(Debug)]
struct Game<'a> {
    /// All the properties in the game.
    properties: [Property<'a>; 28],

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
