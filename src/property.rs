use crate::{player::Player, utils::Ref};

/// A property that can be bought, sold, traded, and auctioned.
#[derive(Debug)]
pub(crate) struct Property {
    /// Name of the property.
    pub(crate) name: String,

    /// The group/color this property belongs to.
    pub(crate) group: PropertyGroup,

    /// The amount a player must pay to own the property.
    pub(crate) price: usize,

    /// The amount a player receives after mortaging the property.
    pub(crate) mortgage: usize,

    /// The cost of one building on the property, if it can be built on.
    pub(crate) building: (bool, usize),

    /// The various amounts players must pay for landing on this property.
    pub(crate) rent: Rent,

    /// The owner of the property.
    pub(crate) owner: Option<Ref<Player>>,
}

/// Represents different types of rents.
#[derive(Debug)]
pub(crate) enum Rent {
    Property {
        base: usize,
        monopoly: usize,
        house1: usize,
        house2: usize,
        house3: usize,
        house4: usize,
        hotel: usize,
    },

    Railroad {
        owned1: usize,
        owned2: usize,
        owned3: usize,
        owned4: usize,
    },

    Utility {
        base: usize,
        monopoly: usize,
    },
}

#[derive(Debug)]
pub(crate) enum PropertyGroup {
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
