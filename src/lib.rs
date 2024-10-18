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
    board: RcCell<Board>,
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
            board: Rc::new(RefCell::new(Board::new())),
            state: GameState::Created,
        }
    }

    /// Runs the game in a loop until a winner is determined.
    #[instrument(skip(self))]
    pub fn run(&mut self) {
        while self.state != GameState::Finished {
            for player in &self.players {
                self.take_turn(player.clone());
            }
        }
    }

    #[instrument(skip(self))]
    fn take_turn(&self, player: RcCell<Player>) {
        // Roll for player's position and move there
        {
            player.borrow_mut().roll_and_move();
        }

        // Handle the position the player landed on
        let position = {
            let pos_idx = player.borrow().current_position;
            self.board.borrow().cells[pos_idx].clone()
        };
        let position = position.borrow_mut().clone();
        match position {
            BoardCell::Go => {
                self.advance_to_position(player.clone(), board::positions::GO);
            }
            BoardCell::CommunityChest => {
                let card = self.board.borrow_mut().draw_community_chest_card();
                match card {
                    board::CommunityChestCard::AdvanceToGo => {
                        self.advance_to_position(player.clone(), board::positions::GO);
                        self.receive_money(player.clone(), 200);
                    }
                    board::CommunityChestCard::BankErrorInYourFavor => {
                        self.receive_money(player.clone(), 200);
                    }
                    board::CommunityChestCard::DoctorsFees => todo!(),
                    board::CommunityChestCard::SaleOfStock => todo!(),
                    board::CommunityChestCard::GetOutOfJailFree => todo!(),
                    board::CommunityChestCard::GoToJail => todo!(),
                    board::CommunityChestCard::HolidayFundMatures => todo!(),
                    board::CommunityChestCard::IncomeTaxRefund => todo!(),
                    board::CommunityChestCard::Birthday => todo!(),
                    board::CommunityChestCard::LifeInsuranceMatures => todo!(),
                    board::CommunityChestCard::HospitalFees => todo!(),
                    board::CommunityChestCard::SchoolFees => todo!(),
                    board::CommunityChestCard::ConsultancyFee => todo!(),
                    board::CommunityChestCard::StreetRepairs => todo!(),
                    board::CommunityChestCard::BeautyContest => todo!(),
                    board::CommunityChestCard::Inherit => todo!(),
                }
            }
            BoardCell::Tax(_) => todo!(),
            BoardCell::Chance => todo!(),
            BoardCell::Jail => todo!(),
            BoardCell::FreeParking(_) => todo!(),
            BoardCell::GoToJail => todo!(),
            BoardCell::Property(property) => todo!(),
        };
    }

    fn advance_to_position(&self, player: RcCell<Player>, position: usize) {
        player.borrow_mut().current_position = position;
    }

    fn receive_money(&self, player: RcCell<Player>, amount: usize) {
        player.borrow_mut().money += amount;
    }

    #[instrument(skip(self))]
    pub(crate) fn pay_bank(&self, player: RcCell<Player>, amount: usize) {
        // TODO: Handle if not enough money!

        let mut player = player.borrow_mut();
        player.money -= amount;
        tracing::info!("{} payed the bank ${}", player.name, amount);
    }
}
