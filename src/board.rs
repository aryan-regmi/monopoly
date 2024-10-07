use crate::utils::Money;

/// A location on the board.
#[derive(Debug, Clone, Copy)]
pub(crate) enum BoardCell {
    /// The starting position.
    Go,

    /// A location that contains a property.
    ///
    /// Contains an index in `Game`'s array of properties.
    Property(usize),

    /// A location that contains a community chest.
    CommunityChest,

    /// A location that contains a chance card.
    Chance,

    /// The jail location for those just visiting and not in jail.
    VisitingJail,

    /// The actual jail location.
    Jail,

    /// The free parking location where all taxes and fines go.
    ///
    /// Contains the amount of money a player will receive upon landing here.
    FreeParking(Money),

    /// The "Go To Jail" location.
    GoToJail,

    /// The various income tax locations.
    ///
    /// Contains the amount of tax a player pays.
    IncomeTax(Money),
}

/// THe size of the board.
pub(crate) const BOARD_SIZE: usize = 40;

/// The actual game board.
pub(crate) const BOARD: [BoardCell; BOARD_SIZE] = [BoardCell::Go; BOARD_SIZE];
