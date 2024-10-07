use crate::{
    properties::Property,
    utils::{Money, D6},
};

/// A player that makes actions in the game.
#[derive(Debug)]
pub(crate) struct Player {
    /// Player's name/id.
    pub(crate) name: String,

    /// The properties owned by the player.
    pub(crate) properties: Vec<usize>,

    /// The amount of money the player has.
    pub(crate) money: Money,

    /// The value of the last dice rolled by the player.
    pub(crate) last_dice: (D6, D6),

    /// Current position on the board.
    ///
    /// This is an index of the `board::BOARD` array.
    pub(crate) position: usize,
}

impl Player {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            properties: vec![],
            money: Money(1500),
            last_dice: (D6(1), D6(1)),
            position: 0,
        }
    }
}
