/// Represents a six-sided die.
#[derive(Debug)]
pub(crate) struct D6(pub(crate) u8);

impl D6 {
    /// Creates a six-sided die.
    fn new(value: u8) -> Self {
        assert!(
            value <= 6 && value >= 1,
            "The six-sided die must have a value between 1 and 6."
        );
        Self(value)
    }
}

/// Represents money and prices.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Money(pub(crate) usize);
