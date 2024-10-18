use rand::Rng;
use tracing::instrument;

use crate::{board_old::NUM_CELLS, property_old::Property, utils::RcCell};

#[derive(Debug, Clone)]
pub struct Player {
    /// The player's name/id.
    pub(crate) name: String,

    /// The properties a player owns.
    pub(crate) properties: Vec<RcCell<Property>>,

    /// The last dice rolled by the player.
    pub(crate) last_dice: Option<(usize, usize)>,

    /// The player's current amount of money.
    pub(crate) money: usize,

    /// Current position on the board.
    pub(crate) current_position: usize,

    /// Used to determine if the player is just visiting jail or in it.
    pub(crate) in_jail: bool,

    /// Used to determine if the player owns the "Get out of jail free" card.
    pub(crate) get_out_of_jail_free: bool,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.into(),
            properties: vec![],
            last_dice: None,
            money: 1500,
            current_position: 0,
            in_jail: false,
            get_out_of_jail_free: false,
        }
    }

    /// Rolls the player's dice and stores the output in `last_dice`.
    fn roll_dice(&mut self) {
        let die1 = rand::thread_rng().gen_range(1..=6);
        let die2 = rand::thread_rng().gen_range(1..=6);
        self.last_dice = Some((die1, die2));
    }

    /// Combines the player's last dice rolls to get the number of spaces to move.
    fn spaces_to_move(&self) -> usize {
        let last_dice = self.last_dice.unwrap();
        last_dice.0 + last_dice.1
    }

    /// Gets the next position on the board, given the player's last dice roll;.
    fn get_next_position(&self) -> usize {
        let next_space = self.current_position + self.spaces_to_move();
        next_space % NUM_CELLS
    }

    /// Rolls the dice and moves the player to the new position.
    #[instrument(skip(self))]
    pub(crate) fn roll_and_move(&mut self) {
        self.roll_dice();
        self.current_position = self.get_next_position();
        tracing::info!("{} rolled {:?}", self.name, self.last_dice.unwrap());
    }
}
