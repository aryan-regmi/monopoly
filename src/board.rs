use bevy_ecs::prelude::*;

use crate::utils::NUM_SPACES;

/// Represents the actual board.
#[derive(Resource)]
pub struct Board(pub Vec<Space>);

impl Default for Board {
    fn default() -> Self {
        let mut board = Vec::with_capacity(NUM_SPACES);
        {
            board.push(Space::Go);
            board.push(Space::Property(Property {
                name: "Mediterranean Avenue".into(),
                group: PropertyGroup::Brown,
                price: 60,
                mortgage: 30,
                building: (true, 50),
                rent: Rent::Property {
                    base: 2,
                    monopoly: 4,
                    house1: 10,
                    house2: 30,
                    house3: 90,
                    house4: 160,
                    hotel: 250,
                },
            }));
            board.push(Space::CommunityChest);
            board.push(Space::Property(Property {
                name: "Baltic Avenue".into(),
                group: PropertyGroup::Brown,
                price: 60,
                mortgage: 30,
                building: (true, 50),
                rent: Rent::Property {
                    base: 4,
                    monopoly: 8,
                    house1: 20,
                    house2: 60,
                    house3: 180,
                    house4: 320,
                    hotel: 450,
                },
            }));
            board.push(Space::Tax(200)); // Income tax
            board.push(Space::Property(Property {
                name: "Reading Railroad".into(),
                group: PropertyGroup::Railroad,
                price: 200,
                mortgage: 100,
                building: (false, 0),
                rent: Rent::Railroad {
                    owned1: 25,
                    owned2: 50,
                    owned3: 100,
                    owned4: 200,
                },
            }));
            board.push(Space::Property(Property {
                name: "Oriental Avenue".into(),
                group: PropertyGroup::LightBlue,
                price: 100,
                mortgage: 50,
                building: (true, 50),
                rent: Rent::Property {
                    base: 6,
                    monopoly: 12,
                    house1: 30,
                    house2: 90,
                    house3: 270,
                    house4: 400,
                    hotel: 550,
                },
            }));
            board.push(Space::Chance);
            board.push(Space::Property(Property {
                name: "Vermont Avenue".into(),
                group: PropertyGroup::LightBlue,
                price: 100,
                mortgage: 50,
                building: (true, 50),
                rent: Rent::Property {
                    base: 6,
                    monopoly: 12,
                    house1: 30,
                    house2: 90,
                    house3: 270,
                    house4: 400,
                    hotel: 550,
                },
            }));
            board.push(Space::Property(Property {
                name: "Connecticut Avenue".into(),
                group: PropertyGroup::LightBlue,
                price: 120,
                mortgage: 60,
                building: (true, 50),
                rent: Rent::Property {
                    base: 8,
                    monopoly: 16,
                    house1: 40,
                    house2: 100,
                    house3: 300,
                    house4: 450,
                    hotel: 600,
                },
            }));
            board.push(Space::Jail);
            board.push(Space::Property(Property {
                name: "St. Charles Place".into(),
                group: PropertyGroup::Pink,
                price: 140,
                mortgage: 70,
                building: (true, 100),
                rent: Rent::Property {
                    base: 10,
                    monopoly: 20,
                    house1: 50,
                    house2: 150,
                    house3: 450,
                    house4: 625,
                    hotel: 750,
                },
            }));
            board.push(Space::Property(Property {
                name: "Electric Company".into(),
                group: PropertyGroup::Utility,
                price: 150,
                mortgage: 75,
                building: (false, 0),
                rent: Rent::Utility {
                    base: 4,
                    monopoly: 10,
                },
            }));
            board.push(Space::Property(Property {
                name: "States Avenue".into(),
                group: PropertyGroup::Pink,
                price: 140,
                mortgage: 70,
                building: (true, 100),
                rent: Rent::Property {
                    base: 10,
                    monopoly: 20,
                    house1: 50,
                    house2: 150,
                    house3: 450,
                    house4: 625,
                    hotel: 750,
                },
            }));
            board.push(Space::Property(Property {
                name: "Virginia Avenue".into(),
                group: PropertyGroup::Pink,
                price: 160,
                mortgage: 80,
                building: (true, 100),
                rent: Rent::Property {
                    base: 12,
                    monopoly: 24,
                    house1: 60,
                    house2: 180,
                    house3: 500,
                    house4: 700,
                    hotel: 900,
                },
            }));
            board.push(Space::Property(Property {
                name: "Pennsylvania Railroad".into(),
                group: PropertyGroup::Railroad,
                price: 200,
                mortgage: 100,
                building: (false, 0),
                rent: Rent::Railroad {
                    owned1: 25,
                    owned2: 50,
                    owned3: 100,
                    owned4: 200,
                },
            }));
            board.push(Space::Property(Property {
                name: "St. James Place".into(),
                group: PropertyGroup::Orange,
                price: 180,
                mortgage: 90,
                building: (true, 100),
                rent: Rent::Property {
                    base: 14,
                    monopoly: 28,
                    house1: 70,
                    house2: 200,
                    house3: 550,
                    house4: 750,
                    hotel: 950,
                },
            }));
            board.push(Space::CommunityChest);
            board.push(Space::Property(Property {
                name: "Tennessee Avenue".into(),
                group: PropertyGroup::Orange,
                price: 180,
                mortgage: 90,
                building: (true, 100),
                rent: Rent::Property {
                    base: 14,
                    monopoly: 28,
                    house1: 70,
                    house2: 200,
                    house3: 550,
                    house4: 750,
                    hotel: 950,
                },
            }));
            board.push(Space::Property(Property {
                name: "New York Avenue".into(),
                group: PropertyGroup::Orange,
                price: 200,
                mortgage: 100,
                building: (true, 100),
                rent: Rent::Property {
                    base: 16,
                    monopoly: 32,
                    house1: 80,
                    house2: 220,
                    house3: 600,
                    house4: 800,
                    hotel: 1000,
                },
            }));
            board.push(Space::FreeParking(0));
            board.push(Space::Property(Property {
                name: "Kentucky Avenue".into(),
                group: PropertyGroup::Red,
                price: 220,
                mortgage: 110,
                building: (true, 150),
                rent: Rent::Property {
                    base: 18,
                    monopoly: 36,
                    house1: 90,
                    house2: 250,
                    house3: 700,
                    house4: 875,
                    hotel: 1050,
                },
            }));
            board.push(Space::Chance);
            board.push(Space::Property(Property {
                name: "Indiana Avenue".into(),
                group: PropertyGroup::Red,
                price: 220,
                mortgage: 110,
                building: (true, 150),
                rent: Rent::Property {
                    base: 18,
                    monopoly: 36,
                    house1: 90,
                    house2: 250,
                    house3: 700,
                    house4: 875,
                    hotel: 1050,
                },
            }));
            board.push(Space::Property(Property {
                name: "Illinois Avenue".into(),
                group: PropertyGroup::Red,
                price: 240,
                mortgage: 120,
                building: (true, 150),
                rent: Rent::Property {
                    base: 20,
                    monopoly: 40,
                    house1: 100,
                    house2: 300,
                    house3: 750,
                    house4: 925,
                    hotel: 1100,
                },
            }));
            board.push(Space::Property(Property {
                name: "B. & O. Railroad".into(),
                group: PropertyGroup::Railroad,
                price: 200,
                mortgage: 100,
                building: (false, 0),
                rent: Rent::Railroad {
                    owned1: 25,
                    owned2: 50,
                    owned3: 100,
                    owned4: 200,
                },
            }));
            board.push(Space::Property(Property {
                name: "Atlantic Avenue".into(),
                group: PropertyGroup::Yellow,
                price: 260,
                mortgage: 130,
                building: (true, 150),
                rent: Rent::Property {
                    base: 22,
                    monopoly: 44,
                    house1: 110,
                    house2: 330,
                    house3: 800,
                    house4: 975,
                    hotel: 1150,
                },
            }));
            board.push(Space::Property(Property {
                name: "Ventnor Avenue".into(),
                group: PropertyGroup::Yellow,
                price: 260,
                mortgage: 130,
                building: (true, 150),
                rent: Rent::Property {
                    base: 22,
                    monopoly: 44,
                    house1: 110,
                    house2: 330,
                    house3: 800,
                    house4: 975,
                    hotel: 1150,
                },
            }));
            board.push(Space::Property(Property {
                name: "Water Works".into(),
                group: PropertyGroup::Utility,
                price: 150,
                mortgage: 75,
                building: (false, 0),
                rent: Rent::Utility {
                    base: 4,
                    monopoly: 10,
                },
            }));
            board.push(Space::Property(Property {
                name: "Marvin Gardens".into(),
                group: PropertyGroup::Yellow,
                price: 280,
                mortgage: 140,
                building: (true, 150),
                rent: Rent::Property {
                    base: 24,
                    monopoly: 48,
                    house1: 120,
                    house2: 360,
                    house3: 850,
                    house4: 1025,
                    hotel: 1200,
                },
            }));
            board.push(Space::GoToJail);
            board.push(Space::Property(Property {
                name: "Pacific Avenue".into(),
                group: PropertyGroup::Green,
                price: 300,
                mortgage: 150,
                building: (true, 200),
                rent: Rent::Property {
                    base: 26,
                    monopoly: 52,
                    house1: 130,
                    house2: 390,
                    house3: 900,
                    house4: 1100,
                    hotel: 1275,
                },
            }));
            board.push(Space::Property(Property {
                name: "North Carolina Avenue".into(),
                group: PropertyGroup::Green,
                price: 300,
                mortgage: 150,
                building: (true, 200),
                rent: Rent::Property {
                    base: 26,
                    monopoly: 52,
                    house1: 130,
                    house2: 390,
                    house3: 900,
                    house4: 1100,
                    hotel: 1275,
                },
            }));
            board.push(Space::CommunityChest);
            board.push(Space::Property(Property {
                name: "Pennsylvania Avenue".into(),
                group: PropertyGroup::Green,
                price: 320,
                mortgage: 160,
                building: (true, 200),
                rent: Rent::Property {
                    base: 28,
                    monopoly: 56,
                    house1: 150,
                    house2: 450,
                    house3: 1000,
                    house4: 1200,
                    hotel: 1400,
                },
            }));
            board.push(Space::Property(Property {
                name: "Short Line".into(),
                group: PropertyGroup::Railroad,
                price: 200,
                mortgage: 100,
                building: (false, 0),
                rent: Rent::Railroad {
                    owned1: 25,
                    owned2: 50,
                    owned3: 100,
                    owned4: 200,
                },
            }));
            board.push(Space::Chance);
            board.push(Space::Property(Property {
                name: "Park Place".into(),
                group: PropertyGroup::DarkBlue,
                price: 350,
                mortgage: 175,
                building: (true, 200),
                rent: Rent::Property {
                    base: 35,
                    monopoly: 70,
                    house1: 175,
                    house2: 500,
                    house3: 1100,
                    house4: 1300,
                    hotel: 1500,
                },
            }));
            board.push(Space::Tax(100)); // Luxury tax
            board.push(Space::Property(Property {
                name: "Boardwalk".into(),
                group: PropertyGroup::DarkBlue,
                price: 400,
                mortgage: 200,
                building: (true, 200),
                rent: Rent::Property {
                    base: 50,
                    monopoly: 100,
                    house1: 200,
                    house2: 600,
                    house3: 1400,
                    house4: 1700,
                    hotel: 2000,
                },
            }));
        }
        Self(board)
    }
}

/// Represents a space on the board.
#[derive(Debug)]
pub enum Space {
    /// The initial position of all player.
    ///
    /// Collect $200 if this is passed.
    Go,

    /// The various community chest cards.
    CommunityChest,

    /// Income and luxury taxes.
    ///
    /// Contains the tax amount.
    Tax(usize),

    /// The various chance cards.
    Chance,

    /// The jail position (Visiting or in jail).
    Jail,

    /// Contains the money stored in free parking.
    ///
    /// First player to land on it gets the money stored here.
    ///
    /// All taxes and fines will go to free parking.
    FreeParking(usize),

    /// Send a player to jail.
    GoToJail,

    /// A property.
    Property(Property),
}

/// A property that can be bought, sold, traded, and auctioned.
#[derive(Debug, Clone)]
pub struct Property {
    /// Name of the property.
    pub name: String,

    /// The group/color this property belongs to.
    pub group: PropertyGroup,

    /// The amount a player must pay to own the property.
    pub price: usize,

    /// The amount a player receives after mortaging the property.
    pub mortgage: usize,

    /// The cost of one building on the property, if it can be built on.
    pub building: (bool, usize),

    /// The various amounts players must pay for landing on this property.
    pub rent: Rent,
}

/// Represents different types of rents.
#[derive(Debug, Clone)]
pub enum Rent {
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

/// The groups that a property can belong to.
#[derive(Debug, Clone, Copy)]
pub enum PropertyGroup {
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
