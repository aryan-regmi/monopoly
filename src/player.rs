use crate::{property::Property, utils::Ref};

#[derive(Debug)]
pub(crate) struct Player {
    /// The player's name/id.
    pub(crate) name: String,

    /// The properties a player owns.
    pub(crate) properties: Vec<Ref<Property>>,

    /// The last dice rolled by the player.
    pub(crate) last_dice: (u8, u8),
}
