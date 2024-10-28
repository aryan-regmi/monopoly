use std::ops::{AddAssign, SubAssign};

use crate::board::Property;
use bevy_ecs::prelude::*;

/// A unique identifier.
#[derive(Component, Debug)]
pub struct Id(pub(crate) usize);

/// A list of properties owned by a player.
#[derive(Component)]
pub struct OwnedProperties(pub(crate) Vec<Property>);

/// Amount of money a player has.
#[derive(Component)]
pub struct Money(pub(crate) usize);

/// Determines if the player has a `Get out of Jail free` card.
#[derive(Component)]
pub struct HasGetOutOfJailFreeCard(pub(crate) bool);

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0 - rhs.0;
    }
}

/// A player's position on the board.
#[derive(Component)]
pub struct Position(pub(crate) usize);

/// Determines if a player is **in** jail or just visiting.
#[derive(Component)]
pub struct InJail(pub(crate) bool);
