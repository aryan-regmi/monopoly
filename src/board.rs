use std::{cell::RefCell, fmt::Display, rc::Rc};

use positions::FREE_PARKING;
use rand::Rng;

use crate::{
    property::{Property, PropertyGroup, Rent},
    utils::RcCell,
};

pub(crate) const NUM_CELLS: usize = 40;
pub(crate) const NUM_CHANCE: usize = 16;
pub(crate) const NUM_COMMUNITY_CHEST: usize = 16;

/// Represents a position on the board.
#[derive(Debug)]
pub(crate) enum BoardCell {
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

impl Display for BoardCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardCell::Go => f.write_str("Go"),
            BoardCell::CommunityChest => f.write_str("Community Chest"),
            BoardCell::Tax(tax) => f.write_fmt(format_args!("Tax (${})", tax)),
            BoardCell::Chance => f.write_str("Chance"),
            BoardCell::Jail => f.write_str("Jail"),
            BoardCell::FreeParking(_) => f.write_str("Free Parking"),
            BoardCell::GoToJail => f.write_str("Go To Jail"),
            BoardCell::Property(property) => f.write_fmt(format_args!("{}", property.name)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ChanceCard {
    /// Advance to "Go", collect $200.
    AdvanceToGo = 0,

    /// Advance to Illinois Avenue. If you pass "Go", collect $200.
    AdvanceToIllinois,

    /// Advance to St. Charles Place. If you pass "Go", collect $200.
    AdvanceToStCharlesPlace,

    /// Advance to the nearest Utility. If unowned, you may buy it from the Bank.
    /// If owned, throw dice and pay owner a total 10x the amount thrown.
    AdvanceToNearestUtility,

    /// Advance to the nearest Railroad. If unowned, you may buy it from the Bank.
    /// If owned, pay owner twice the rent to which they are otherwise entitled.
    /// If Railroad is unowned, you may buy it from the Bank.
    AdvanceToNearestRailroad,

    /// Bank pays you dividend of $50.
    Dividend,

    /// Get out of jail free card.
    GetOutOfJailFree,

    /// Go back 3 spaces.
    GoBack3Spaces,

    /// Go directly to jail: don't pass "Go", don't collect $200.
    GoToJail,

    /// Make general repairs on all your property: for each house pay $25, for each hotel pay $100.
    GeneralRepairs,

    /// Take a trip to Reading Railroad. If you pass "Go", collect $200.
    AdvanceToReadingRailroad,

    /// Pay "Poor Tax" of $15
    PoorTax,

    /// Take a trip to Boardwalk. If you pass "Go", collect $200.
    AdvanceToBoardwalk,

    /// You have been elected Chairman of the Board, Pay each player $50.
    ChairmanOfTheBoard,

    /// Your building loan matures, receive $150.
    BuildingLoanMatures,

    /// Holiday fund matures, collect $100.
    HolidayFundMatures,
}

/// The various community chest cards.
#[derive(Debug, Clone, Copy)]
pub(crate) enum CommunityChestCard {
    /// Advance to "Go", collect $200.
    AdvanceToGo = 0,

    /// Bank error in your favor, collect $200.
    BankErrorInYourFavor,

    /// Doctor's fees, pay $50.
    DoctorsFees,

    /// From sale of stock you get $50.
    SaleOfStock,

    /// Get out of jail free card.
    GetOutOfJailFree,

    /// Go directly to jail: don't pass "Go", don't collect $200.
    GoToJail,

    /// Holiday fund matures, collect $100.
    HolidayFundMatures,

    /// Income tax refund, collect $20.
    IncomeTaxRefund,

    /// It's your birthday, collect $10 from every player.
    Birthday,

    /// Life insurance matures, collect $100
    LifeInsuranceMatures,

    /// Hospital Fees, pay $50.
    HospitalFees,

    /// School fees, pay $50.
    SchoolFees,

    /// Receive $25 consultancy fee.
    ConsultancyFee,

    /// You are assessed for street repairs: pay $40 per house and $115 per hotel you own.
    StreetRepairs,

    /// You have won second prize in a beauty contest, collect $10.
    BeautyContest,

    /// You inherit $100.
    Inherit,
}

#[derive(Debug)]
pub(crate) struct Board {
    /// All possible positions on the board.
    pub(crate) cells: Vec<RcCell<BoardCell>>,

    /// All possible chance cards.
    pub(crate) chance_cards: Vec<RcCell<ChanceCard>>,

    /// All possible community chest cards.
    pub(crate) community_chest_cards: Vec<RcCell<CommunityChestCard>>,
}

impl Board {
    pub(crate) fn new() -> Self {
        // Initialize board
        let mut cells = Vec::with_capacity(NUM_CELLS);
        {
            cells.push(Rc::new(RefCell::new(BoardCell::Go)));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::CommunityChest)));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Tax(200)))); // Income tax
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Chance)));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Jail)));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
                name: "Electric Company".into(),
                group: PropertyGroup::Utility,
                price: 150,
                mortgage: 75,
                building: (false, 0),
                rent: Rent::Utility {
                    base: 4,
                    monopoly: 10,
                },
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::CommunityChest)));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::FreeParking(0))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Chance)));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
                name: "Water Works".into(),
                group: PropertyGroup::Utility,
                price: 150,
                mortgage: 75,
                building: (false, 0),
                rent: Rent::Utility {
                    base: 4,
                    monopoly: 10,
                },
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::GoToJail)));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::CommunityChest)));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Chance)));
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
            cells.push(Rc::new(RefCell::new(BoardCell::Tax(100)))); // Luxury tax
            cells.push(Rc::new(RefCell::new(BoardCell::Property(Property {
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
                owner: None,
            }))));
        }

        // Initialize Community Chest
        let mut community_chest_cards = Vec::with_capacity(NUM_COMMUNITY_CHEST);
        {
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::AdvanceToGo)));
            community_chest_cards.push(Rc::new(RefCell::new(
                CommunityChestCard::BankErrorInYourFavor,
            )));
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::DoctorsFees)));
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::SaleOfStock)));
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::GetOutOfJailFree)));
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::GoToJail)));
            community_chest_cards.push(Rc::new(RefCell::new(
                CommunityChestCard::HolidayFundMatures,
            )));
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::IncomeTaxRefund)));
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::Birthday)));
            community_chest_cards.push(Rc::new(RefCell::new(
                CommunityChestCard::LifeInsuranceMatures,
            )));
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::HospitalFees)));
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::SchoolFees)));
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::ConsultancyFee)));
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::StreetRepairs)));
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::BeautyContest)));
            community_chest_cards.push(Rc::new(RefCell::new(CommunityChestCard::Inherit)));

            // Randomize order
            let mut idxs = Vec::with_capacity(NUM_COMMUNITY_CHEST);
            let mut shuffled = Vec::with_capacity(NUM_COMMUNITY_CHEST);
            while idxs.len() < NUM_COMMUNITY_CHEST {
                let idx = rand::thread_rng().gen_range(0..NUM_COMMUNITY_CHEST);
                if !idxs.contains(&idx) {
                    shuffled.push(community_chest_cards[idx].clone());
                    idxs.push(idx);
                }
            }
            community_chest_cards = shuffled;
        }

        // Initialize Community Chest
        let mut chance_cards = Vec::with_capacity(NUM_CHANCE);
        {
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::AdvanceToGo)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::AdvanceToIllinois)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::AdvanceToStCharlesPlace)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::AdvanceToNearestUtility)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::AdvanceToNearestRailroad)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::Dividend)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::GetOutOfJailFree)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::GoBack3Spaces)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::GoToJail)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::GeneralRepairs)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::AdvanceToReadingRailroad)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::PoorTax)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::AdvanceToBoardwalk)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::ChairmanOfTheBoard)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::BuildingLoanMatures)));
            chance_cards.push(Rc::new(RefCell::new(ChanceCard::HolidayFundMatures)));

            // Randomize order
            let mut idxs = Vec::with_capacity(NUM_CHANCE);
            let mut shuffled = Vec::with_capacity(NUM_CHANCE);
            while idxs.len() < NUM_CHANCE {
                let idx = rand::thread_rng().gen_range(0..NUM_CHANCE);
                if !idxs.contains(&idx) {
                    shuffled.push(chance_cards[idx].clone());
                    idxs.push(idx);
                }
            }
            chance_cards = shuffled;
        }

        Self {
            cells,
            chance_cards,
            community_chest_cards,
        }
    }

    /// Draw from the top of community chest pile and place at the bottom.
    pub(crate) fn draw_community_chest_card(&mut self) -> CommunityChestCard {
        let drawn = self.community_chest_cards.pop().unwrap();
        self.community_chest_cards.insert(0, drawn.clone());
        drawn.clone().borrow().clone()
    }

    /// Draw from the top of chance pile and place at the bottom.
    pub(crate) fn draw_chance_card(&mut self) -> ChanceCard {
        let drawn = self.chance_cards.pop().unwrap();
        self.chance_cards.insert(0, drawn.clone());
        drawn.clone().borrow().clone()
    }

    /// Adds the given amount to free parking.
    pub(crate) fn add_to_free_parking(&mut self, amount: usize) {
        let mut cell = self.cells[positions::FREE_PARKING].borrow_mut();
        if let BoardCell::FreeParking(curr) = *cell {
            *cell = BoardCell::FreeParking(curr + amount);
        }
    }

    /// Removes all the money from free parking.
    pub(crate) fn remove_from_free_parking(&mut self) -> usize {
        let mut cell = self.cells[positions::FREE_PARKING].borrow_mut();
        if let BoardCell::FreeParking(curr) = *cell {
            *cell = BoardCell::FreeParking(0);
            return curr;
        }
        unreachable!()
    }
}

/// Defines all positions as indicies.
pub(crate) mod positions {
    pub(crate) const GO: usize = 0;
    pub(crate) const MEDITERRANEAN_AVENUE: usize = 1;
    pub(crate) const COMMUNITY_CHEST_1: usize = 2;
    pub(crate) const BALTIC_AVENUE: usize = 3;
    pub(crate) const INCOME_TAX: usize = 4;
    pub(crate) const READING_RAILROAD: usize = 5;
    pub(crate) const ORIENTAL_AVENUE: usize = 6;
    pub(crate) const CHANCE_1: usize = 7;
    pub(crate) const VERMONT_AVENUE: usize = 8;
    pub(crate) const CONNECTICUT_AVENUE: usize = 9;
    pub(crate) const JAIL: usize = 10;
    pub(crate) const ST_CHARLES_PLACE: usize = 11;
    pub(crate) const ELECTRIC_COMPANY: usize = 12;
    pub(crate) const STATES_AVENUE: usize = 13;
    pub(crate) const VIRGINIA_AVENUE: usize = 14;
    pub(crate) const PENNSYLVANIA_RAILROAD: usize = 15;
    pub(crate) const ST_JAMES_PLACE: usize = 16;
    pub(crate) const COMMUNITY_CHEST_2: usize = 17;
    pub(crate) const TENNESSEE_AVENUE: usize = 18;
    pub(crate) const NEW_YORK_AVENUE: usize = 19;
    pub(crate) const FREE_PARKING: usize = 20;
    pub(crate) const KENTUCKY_AVENUE: usize = 21;
    pub(crate) const CHANCE_2: usize = 22;
    pub(crate) const INDIANA_AVENUE: usize = 23;
    pub(crate) const ILLINOIS_AVENUE: usize = 24;
    pub(crate) const B_O_RAILROAD: usize = 25;
    pub(crate) const ATLANTIC_AVENUE: usize = 26;
    pub(crate) const VENTNOR_AVENUE: usize = 27;
    pub(crate) const WATER_WORKS: usize = 28;
    pub(crate) const MARVIN_GARDENS: usize = 29;
    pub(crate) const GO_TO_JAIL: usize = 30;
    pub(crate) const PACIFIC_AVENUE: usize = 31;
    pub(crate) const NORTH_CAROLINA_AVENUE: usize = 32;
    pub(crate) const COMMUNITY_CHEST_3: usize = 33;
    pub(crate) const PENNSYLVANIA_AVENUE: usize = 34;
    pub(crate) const SHORT_LINE: usize = 35;
    pub(crate) const CHANCE_3: usize = 36;
    pub(crate) const PARK_PLACE: usize = 37;
    pub(crate) const LUXURY_TAX: usize = 38;
    pub(crate) const BOARDWALK: usize = 39;
}
