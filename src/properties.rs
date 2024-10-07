use crate::utils::Money;

/// The state of a property.
#[derive(Debug, Clone, Copy)]
pub(crate) enum PropertyState {
    /// The initial state of all properties; all properties can be bought at the start of the
    /// game.
    NotBought,

    /// The property is bought and owned by a player.
    Bought,

    /// The property is mortgaged to the bank.
    ///
    /// The owning player can't collect rent on a mortgaged property.
    Mortgaged,
}

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

    /// The current state of the property.
    pub(crate) state: PropertyState,
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
    state: PropertyState::NotBought,
}; 28];
