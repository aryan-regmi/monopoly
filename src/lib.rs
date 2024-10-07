#![allow(unused)]

mod board;
mod player;
mod properties;
mod utils;

use board::{BoardCell, BOARD};
use player::Player;
use properties::{Property, PropertyColor, PropertyInner, PROPERTIES};
use rand::Rng;
use utils::{Money, D6};

/// Total number of houses in the game.
const NUM_HOUSES: u8 = 32;

/// Total number of hotels in the game.
const NUM_HOTLES: u8 = 12;

/// Total number of properties in the game.
const NUM_PROPERTIES: u8 = 28;

/// Total number of locations/board cells in the game.
const NUM_LOCATIONS: u8 = 41;

/// Contains the game state and logic.
#[derive(Debug)]
struct Game<'a> {
    /// All the properties in the game.
    properties: [Property<'a>; NUM_PROPERTIES as usize],

    /// All players in the game.
    players: Vec<Player<'a>>,

    /// Represents the game board.
    ///
    /// 41 cells for each position in the game.
    board: [BoardCell; NUM_LOCATIONS as usize],
}

impl<'a> Game<'a> {
    /// Initializes a new game.
    fn new(players: Vec<Player<'a>>) -> Self {
        Self {
            players,
            properties: PROPERTIES,
            board: BOARD,
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

                // Move to rolled location
                let new_position = Self::board_index_from_dice(player.position, &player.last_dice);
                player.position = new_position;

                // TODO: Determine type of location
                match self.board[new_position] {
                    BoardCell::Go => {
                        player.money += Money(200);
                    }
                    BoardCell::Property(_) => todo!(),
                    BoardCell::CommunityChest => todo!(),
                    BoardCell::Chance => todo!(),
                    BoardCell::VisitingJail => todo!(),
                    BoardCell::Jail => todo!(),
                    BoardCell::FreeParking(money) => todo!(),
                    BoardCell::GoToJail => todo!(),
                    BoardCell::IncomeTax(money) => todo!(),
                }

                // TODO: Take possible actions
                //  - Determine status of the property
                //      - Buy if `NotBought`
                //          - Auction if not enough money
                //      - Nothing if `Mortgaged`
                //      - Pay rent if `Bought`

                // TODO: Reroll if double was rolled
                //  - 3 doubles in a row = jail
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

    /// Gets the index in the `board` array from a given dice roll.
    ///
    /// # Note
    /// `curr_idx` is the current position of the **player**.
    fn board_index_from_dice(curr_position: usize, dice: &(D6, D6)) -> usize {
        // Add dice total to curr_position, clamped by total number of cells in the board
        let dice_total = Self::total_dice(dice);
        let mut next_pos = curr_position + dice_total as usize;
        if next_pos >= NUM_LOCATIONS as usize {
            next_pos = 0;
        }
        next_pos
    }
}
