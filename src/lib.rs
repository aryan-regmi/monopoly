#![allow(unused)]

mod board;
mod player;
mod property;
mod utils;

use board::{Board, BoardCell};
pub use player::Player;
use std::{cell::RefCell, rc::Rc};
use tracing::instrument;
use utils::RcCell;

/// Represents the various possible states of the game.
#[derive(Debug, PartialEq, PartialOrd)]
enum GameState {
    Created,
    Running,
    Paused,
    Finished,
}

/// The actual game to be run by users.
#[derive(Debug)]
pub struct Game {
    players: Vec<RcCell<Player>>,
    board: Board,
    state: GameState,
}

impl Game {
    /// Starts a new game with the given players.
    pub fn new(players: Vec<Player>) -> Self {
        assert!(
            players.len() >= 2,
            "At least 2 players are required to start a game"
        );
        assert!(players.len() <= 6, "There can be a max of 6 players");

        Self {
            players: players
                .iter()
                .map(|p| Rc::new(RefCell::new(p.clone())))
                .collect(),
            board: Board::new(),
            state: GameState::Created,
        }
    }

    /// Runs the game in a loop until a winner is determined.
    #[instrument(skip(self))]
    pub fn run(&mut self) {
        while self.state != GameState::Finished {
            self.advance();
        }
    }

    /// Advance the game by one round (each player gets a turn).
    pub fn advance(&mut self) {
        if self.state != GameState::Paused {
            self.state = GameState::Running;
            for player in self.players.clone() {
                self.take_turn(player.clone());
                let mut num_doubles = 0;
                let mut last_dice = player.borrow().last_dice.unwrap();
                while last_dice.0 == last_dice.1 {
                    num_doubles += 1;

                    // Roll again if double rolled
                    self.take_turn(player.clone());
                    last_dice = player.borrow().last_dice.unwrap();

                    // Go to jail if 3 doubles in a row
                    if num_doubles == 3 {
                        let mut player = player.borrow_mut();
                        player.in_jail = true;
                        player.current_position = board::positions::JAIL;
                    }
                }
            }
        }
    }

    /// Simulates a player's turn by rolling dice to move them to the next position, and handling
    /// the newly landed position.
    #[instrument(skip(self, player))]
    fn take_turn(&mut self, player: RcCell<Player>) {
        // Roll dice and move the player
        {
            player.borrow_mut().roll_and_move();
        }

        let curr_space = {
            let curr_pos = player.borrow().current_position;
            self.board.cells[curr_pos].clone()
        };
        tracing::info!(
            "Player {} landed on {}",
            player.borrow().name,
            curr_space.borrow()
        );

        match &*curr_space.borrow_mut() {
            board::BoardCell::Go => {
                player.borrow_mut().money += 200;
            }
            board::BoardCell::CommunityChest => {
                let card = self.board.draw_community_chest_card();
                match card {
                    board::CommunityChestCard::AdvanceToGo => {
                        let mut player = player.borrow_mut();
                        player.money += 200;
                        player.current_position = board::positions::GO;
                    }
                    board::CommunityChestCard::BankErrorInYourFavor => {
                        player.borrow_mut().money += 200;
                    }
                    board::CommunityChestCard::DoctorsFees => {
                        // TODO: Handle if not enought money
                        player.borrow_mut().money -= 50;
                        self.board.add_to_free_parking(50);
                    }
                    board::CommunityChestCard::SaleOfStock => {
                        player.borrow_mut().money += 50;
                    }
                    board::CommunityChestCard::GetOutOfJailFree => {
                        player.borrow_mut().get_out_of_jail_free = true;
                    }
                    board::CommunityChestCard::GoToJail => {
                        player.borrow_mut().current_position = board::positions::JAIL;
                    }
                    board::CommunityChestCard::HolidayFundMatures => {
                        player.borrow_mut().money += 100;
                    }
                    board::CommunityChestCard::IncomeTaxRefund => {
                        player.borrow_mut().money += 20;
                    }
                    board::CommunityChestCard::Birthday => {
                        for other in &mut self.players {
                            if player.borrow().name != other.borrow().name {
                                // TODO: Pay the current player
                                //      - Add payment methods in Player struct
                            }
                        }
                    }
                    board::CommunityChestCard::LifeInsuranceMatures => {
                        player.borrow_mut().money += 100;
                    }
                    board::CommunityChestCard::HospitalFees => {
                        // TODO: Handle if not enought money
                        player.borrow_mut().money -= 50;
                        self.board.add_to_free_parking(50);
                    }
                    board::CommunityChestCard::SchoolFees => {
                        // TODO: Handle if not enought money
                        player.borrow_mut().money -= 50;
                        self.board.add_to_free_parking(50);
                    }
                    board::CommunityChestCard::ConsultancyFee => {
                        player.borrow_mut().money += 25;
                    }
                    board::CommunityChestCard::StreetRepairs => {
                        // TODO Implement
                    }
                    board::CommunityChestCard::BeautyContest => {
                        player.borrow_mut().money += 10;
                    }
                    board::CommunityChestCard::Inherit => {
                        player.borrow_mut().money += 100;
                    }
                }
            }
            board::BoardCell::Tax(tax) => {
                // TODO: Handle if not enough money!
                player.borrow_mut().money -= tax;
            }
            board::BoardCell::Chance => {
                let card = self.board.draw_community_chest_card();
                match card {
                    board::CommunityChestCard::AdvanceToGo => {
                        let mut player = player.borrow_mut();
                        player.money += 200;
                        player.current_position = board::positions::GO;
                    }
                    board::CommunityChestCard::BankErrorInYourFavor => {
                        player.borrow_mut().money += 200;
                    }
                    board::CommunityChestCard::DoctorsFees => {
                        // TODO: Handle if not enought money
                        player.borrow_mut().money -= 50;
                        self.board.add_to_free_parking(50);
                    }
                    board::CommunityChestCard::SaleOfStock => {
                        player.borrow_mut().money += 50;
                    }
                    board::CommunityChestCard::GetOutOfJailFree => {
                        player.borrow_mut().get_out_of_jail_free = true;
                    }
                    board::CommunityChestCard::GoToJail => {
                        player.borrow_mut().current_position = board::positions::JAIL;
                    }
                    board::CommunityChestCard::HolidayFundMatures => {
                        player.borrow_mut().money += 100;
                    }
                    board::CommunityChestCard::IncomeTaxRefund => {
                        player.borrow_mut().money += 20;
                    }
                    board::CommunityChestCard::Birthday => {
                        for other in &mut self.players {
                            if player.borrow().name != other.borrow().name {
                                // TODO: Pay the current player
                                //      - Add payment methods in Player struct
                            }
                        }
                    }
                    board::CommunityChestCard::LifeInsuranceMatures => {
                        player.borrow_mut().money += 20;
                    }
                    board::CommunityChestCard::HospitalFees => {
                        // TODO: Handle if not enought money
                        player.borrow_mut().money -= 50;
                        self.board.add_to_free_parking(50);
                    }
                    board::CommunityChestCard::SchoolFees => {
                        // TODO: Handle if not enought money
                        player.borrow_mut().money -= 50;
                        self.board.add_to_free_parking(50);
                    }
                    board::CommunityChestCard::ConsultancyFee => {
                        player.borrow_mut().money += 25;
                    }
                    board::CommunityChestCard::StreetRepairs => {
                        // TODO: Implement!
                    }
                    board::CommunityChestCard::BeautyContest => {
                        player.borrow_mut().money += 10;
                    }
                    board::CommunityChestCard::Inherit => {
                        player.borrow_mut().money += 100;
                    }
                }
            }
            board::BoardCell::Jail => {
                if player.borrow().in_jail == true {
                    // TODO: Handle jail actions
                }

                // Do nothing if player is just visiting!
                tracing::info!("{} is visiting jail.", player.borrow().name);
            }
            board::BoardCell::FreeParking(money) => {
                let free_parking_money = self.board.cells[board::positions::FREE_PARKING].clone();
                let free_parking_cell = free_parking_money.borrow();
                if let BoardCell::FreeParking(money) = *free_parking_cell {
                    player.borrow_mut().money += money;
                }
                *self.board.cells[board::positions::FREE_PARKING].borrow_mut() =
                    BoardCell::FreeParking(0);
            }
            board::BoardCell::GoToJail => {
                let mut player = player.borrow_mut();
                player.in_jail = true;
                player.current_position = board::positions::JAIL;
            }
            board::BoardCell::Property(property) => {
                // TODO:
                //      - Buy/auction if unowned
                //      - Pay rent if owned
                //      - Pay tax if tax
                //      - Handle if not enought money
            }
        };
    }
}
