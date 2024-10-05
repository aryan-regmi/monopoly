use crate::{
    properties::Property,
    utils::{Money, D6},
};

/// A player that makes actions in the game.
#[derive(Debug)]
pub(crate) struct Player<'a> {
    /// Player's name/id.
    pub(crate) name: String,

    /// The properties owned by the player.
    pub(crate) properties: Vec<&'a Property<'a>>,

    /// The amount of money the player has.
    pub(crate) money: Money,

    /// The value of the last dice rolled by the player.
    pub(crate) last_dice: (D6, D6),
}

impl<'a> Player<'a> {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            properties: vec![],
            money: Money(1500),
            last_dice: (D6(1), D6(1)),
        }
    }
}
