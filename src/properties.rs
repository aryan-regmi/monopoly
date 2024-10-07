use crate::utils::Money;

/// A property that can be owned by players.
///
/// Contains the various prices (cost, rent, mortgage) and the color of a property.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Property<'a> {
    /// The name of the property.
    pub(crate) name: &'a str,

    /// The color/group/type of the property (i.e. orange, utility, etc.).
    pub(crate) color: PropertyColor,

    /// The amount a player must pay to buy the property from the bank.
    pub(crate) price: Money,

    /// The amount received when the player mortgages the property.
    ///
    /// # Note
    /// The property must be "unimproved"; all buildings on all properties of the matching color
    /// must be sold to the bank first (at half price).
    pub(crate) mortgage: Money,

    /// Cost for each building on the property (house, hotel).
    pub(crate) building_cost: Money,

    /// The amount a player receives if another player land on this property.
    ///
    /// # Note
    /// The array is laid out in the following order:
    /// `Base, Monopoly, 1 House, 2 House, 3 House, 4 House, Hotel`
    pub(crate) rent: [Money; 7],

    /// The current rent state of the property
    pub(crate) rent_state: RentType,

    /// The current state of the property.
    pub(crate) state: PropertyState,
}

/// The state of a property.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) enum PropertyState {
    /// The initial state of all properties; all properties can be bought at the start of the
    /// game.
    NotBought,

    /// The property is bought and owned by a player.
    Bought(usize),

    /// The property is mortgaged to the bank.
    ///
    /// The owning player can't collect rent on a mortgaged property.
    Mortgaged,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum RentType {
    Base = 0,
    Monopoly,
    House1,
    House2,
    House3,
    House4,
    Hotel,
}

impl RentType {
    pub(crate) fn downgrade(&mut self) {
        match self {
            RentType::Base => *self = RentType::Base,
            RentType::Monopoly => *self = RentType::Base,
            RentType::House1 => *self = RentType::Monopoly,
            RentType::House2 => *self = RentType::House1,
            RentType::House3 => *self = RentType::House2,
            RentType::House4 => *self = RentType::House3,
            RentType::Hotel => *self = RentType::House4,
        }
    }

    pub(crate) fn upgrade(&mut self) {
        match self {
            RentType::Base => *self = RentType::Monopoly,
            RentType::Monopoly => *self = RentType::House1,
            RentType::House1 => *self = RentType::House2,
            RentType::House2 => *self = RentType::House3,
            RentType::House3 => *self = RentType::House4,
            RentType::House4 => *self = RentType::Hotel,
            RentType::Hotel => *self = RentType::Hotel,
        }
    }

    pub(crate) fn has_houses(&self) -> bool {
        match self {
            RentType::Base => false,
            RentType::Monopoly => false,
            RentType::House1 => true,
            RentType::House2 => true,
            RentType::House3 => true,
            RentType::House4 => true,
            RentType::Hotel => true,
        }
    }

    pub(crate) fn has_monopoly(&self) -> bool {
        match self {
            RentType::Base => false,
            RentType::Monopoly => true,
            RentType::House1 => true,
            RentType::House2 => true,
            RentType::House3 => true,
            RentType::House4 => true,
            RentType::Hotel => true,
        }
    }
}

/// The various types/colors of properties.
#[derive(Debug, Clone, Copy)]
pub(crate) enum PropertyColor {
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

/// All the properties up for sale.
pub(crate) const PROPERTIES: [Property; 28] = [Property {
    name: "Mediterranean Avenue",
    color: PropertyColor::Brown,
    price: Money(60),
    mortgage: Money(30),
    building_cost: Money(50),
    rent: [
        Money(2),
        Money(4),
        Money(10),
        Money(30),
        Money(90),
        Money(160),
        Money(250),
    ],
    rent_state: RentType::Base,
    state: PropertyState::NotBought,
}; 28];
