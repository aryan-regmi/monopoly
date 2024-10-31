#![allow(unused)]

use bevy_ecs::prelude::*;
use board::{
    draw_chance_card, draw_community_chest_card, positions, Board, ChanceCards,
    CommunityChestCards, Property, Space,
};
use player::{HasGetOutOfJailFreeCard, Id, InJail, Money, OwnedProperties, Position};
use rand::Rng;
use tracing::{info, instrument};
use utils::NUM_SPACES;

pub mod board;
pub mod player;
pub mod utils;

/// The game struct that contains all players and the ECS.
pub struct Game {
    ecs: World,
    running: bool,
}

impl Game {
    /// Initializes a new game.
    pub fn new(num_players: usize) -> Self {
        assert!(
            num_players <= 6 && num_players >= 2,
            "Monopoly only can be played with 2-6 players."
        );

        // Init ECS
        let mut ecs = World::new();

        // Insert board, community chest cards, and chance cards into ECS as resources
        ecs.insert_resource(Board::default());
        ecs.insert_resource(ChanceCards::default());
        ecs.insert_resource(CommunityChestCards::default());

        // Insert players (entities)
        for i in 0..num_players {
            let player_id = i + 1;
            ecs.spawn((
                Id(player_id),
                OwnedProperties(Vec::with_capacity(3)),
                Money(1500),
                Position(0),
                InJail(false),
                HasGetOutOfJailFreeCard(false),
            ));
        }

        // TODO: Determine player order

        Self { ecs, running: true }
    }

    /// Starts the game loop.
    pub fn start(&mut self) {
        let game_loop = self.ecs.register_system(Self::game_loop);

        while self.running {
            self.ecs.run_system(game_loop);
        }
    }

    fn roll_for_player_order() {}

    /// Runs the game in a loop until a winner is determined.
    fn game_loop(
        mut board: ResMut<Board>,
        mut community_chest_cards: ResMut<CommunityChestCards>,
        mut chance_cards: ResMut<ChanceCards>,
        mut players: Query<
            (
                &Id,
                &mut OwnedProperties,
                &mut Money,
                &mut Position,
                &mut InJail,
                &mut HasGetOutOfJailFreeCard,
            ),
            With<Id>,
        >,
    ) {
        for mut player in &mut players {
            // Roll dice and move player
            let mut last_roll = Self::roll_dice();
            let mut last_pos = player.3 .0;
            *player.3 = Position((last_pos + last_roll.0 + last_roll.1) % NUM_SPACES);

            Self::move_player(
                &mut board,
                &mut community_chest_cards,
                &mut chance_cards,
                &mut player,
                last_roll,
                last_pos,
            );

            // Reroll if double
            let mut double_count = 0;
            while last_roll.0 == last_roll.1 && double_count != 3 {
                double_count += 1;
                last_roll = Self::roll_dice();
                last_pos = player.3 .0;
                *player.3 = Position((last_pos + last_roll.0 + last_roll.1) % NUM_SPACES);

                Self::move_player(
                    &mut board,
                    &mut community_chest_cards,
                    &mut chance_cards,
                    &mut player,
                    last_roll,
                    last_pos,
                );

                // Send to jail after 3 doubles
                if double_count == 3 {
                    *player.3 = Position(positions::JAIL);
                    *player.4 = InJail(true);
                }
            }

            // End if only 1 player remains
        }
    }

    /// Moves the player to their new position.
    #[instrument(skip(
        board,
        community_chest_cards,
        chance_cards,
        player,
        last_roll,
        last_pos
    ))]
    fn move_player(
        board: &mut ResMut<Board>,
        community_chest_cards: &mut ResMut<CommunityChestCards>,
        chance_cards: &mut ResMut<ChanceCards>,
        mut player: &mut (
            &Id,
            Mut<'_, OwnedProperties>,
            Mut<'_, Money>,
            Mut<'_, Position>,
            Mut<'_, InJail>,
            Mut<'_, HasGetOutOfJailFreeCard>,
        ),
        last_roll: (usize, usize),
        last_pos: usize,
    ) {
        // Handle the newly moved-to position
        let curr_space = &board.0[player.3 .0];
        info!(
            "Player {:?} rolled {:?} and landed on {:?}",
            player.0, last_roll, curr_space
        );
        match curr_space {
            board::Space::Go => {
                if last_pos != 0 {
                    *player.2 += Money(200);
                    info!(
                        "Player {:?} passed go and collected $200 (current money: ${})",
                        player.0, player.2 .0
                    );
                }
            }
            board::Space::CommunityChest => {
                let card = draw_community_chest_card(community_chest_cards);
                match card {
                    board::CommunityChestCard::AdvanceToGo => {
                        *player.3 = Position(positions::GO);
                        *player.2 += Money(200);
                        info!(
                            "Player {:?} passed go and collected $200 (current money: ${})",
                            player.0, player.2 .0
                        );
                    }
                    board::CommunityChestCard::BankErrorInYourFavor => {
                        *player.2 += Money(200);
                        info!(
                            "Bank error in player's {:?} favor (current money: ${})",
                            player.0, player.2 .0
                        );
                    }
                    board::CommunityChestCard::DoctorsFees => {
                        let fine = 50;
                        // Pay and add fine to free parking
                        if player.2 .0 >= fine {
                            let free_parking = &mut board.0[positions::FREE_PARKING];
                            if let Space::FreeParking(amount) = free_parking {
                                *amount += fine;
                            }
                            *player.2 -= Money(fine);
                            info!(
                                "Player {:?} paid doctor's fees of $50 (current money: ${})",
                                player.0, player.2 .0
                            );
                        } else {
                            Self::handle_not_enough_money(player);
                        }
                    }
                    board::CommunityChestCard::SaleOfStock => {
                        *player.2 += Money(50);
                        info!(
                            "From sale of stock player {:?} received $50 (current money: ${})",
                            player.0, player.2 .0
                        );
                    }
                    board::CommunityChestCard::GetOutOfJailFree => {
                        *player.5 = HasGetOutOfJailFreeCard(true);
                        info!(
                            "Player {:?} received a `Get Out Of Jail Free` card",
                            player.0
                        );
                    }
                    board::CommunityChestCard::GoToJail => {
                        *player.3 = Position(positions::JAIL);
                        *player.4 = InJail(true);
                        info!("Player {:?} was sent to jail", player.0);
                    }
                    board::CommunityChestCard::HolidayFundMatures => {
                        *player.2 += Money(100);
                        info!(
                            "Player {:?} received $100 from their holiday fund (current money: ${})",
                            player.0, player.2.0
                        );
                    }
                    board::CommunityChestCard::IncomeTaxRefund => {
                        *player.2 += Money(20);
                        info!(
                            "Player {:?} received $20 from their income tax refund (current money: ${})",
                            player.0, player.2.0
                        );
                    }
                    board::CommunityChestCard::Birthday => todo!(),
                    board::CommunityChestCard::LifeInsuranceMatures => {
                        *player.2 += Money(100);
                        info!(
                            "Player {:?} received $20 from their life insurance (current money: ${})",
                            player.0, player.2.0
                        );
                    }
                    board::CommunityChestCard::HospitalFees => {
                        let fine = 50;
                        // Pay and add fine to free parking
                        if player.2 .0 >= fine {
                            let free_parking = &mut board.0[positions::FREE_PARKING];
                            if let Space::FreeParking(amount) = free_parking {
                                *amount += fine;
                            }
                            *player.2 -= Money(fine);
                            info!(
                                "Player {:?} paid $50 for hospital fees (current money: ${})",
                                player.0, player.2 .0
                            );
                        } else {
                            Self::handle_not_enough_money(player);
                        }
                    }
                    board::CommunityChestCard::SchoolFees => {
                        let fine = 50;
                        // Pay and add fine to free parking
                        if player.2 .0 >= fine {
                            let free_parking = &mut board.0[positions::FREE_PARKING];
                            if let Space::FreeParking(amount) = free_parking {
                                *amount += fine;
                            }
                            *player.2 -= Money(fine);
                            info!(
                                "Player {:?} paid $50 for school fees (current money: ${})",
                                player.0, player.2 .0
                            );
                        } else {
                            Self::handle_not_enough_money(player);
                        }
                    }
                    board::CommunityChestCard::ConsultancyFee => {
                        *player.2 += Money(25);
                        info!(
                            "Player {:?} received $25 as a consultancy fee (current money: ${})",
                            player.0, player.2 .0
                        );
                    }
                    board::CommunityChestCard::StreetRepairs => todo!(),
                    board::CommunityChestCard::BeautyContest => {
                        *player.2 += Money(10);
                        info!(
                            "Player {:?} received $10 for 2nd place in a beauty contest (current money: ${})",
                            player.0, player.2.0
                        );
                    }
                    board::CommunityChestCard::Inherit => {
                        *player.2 += Money(100);
                        info!(
                            "Player {:?} inherited $100 (current money: ${})",
                            player.0, player.2 .0
                        );
                    }
                }
            }
            board::Space::Tax(fine) => {
                let fine = *fine;
                // Pay and add fine to free parking
                if player.2 .0 >= fine {
                    let free_parking = &mut board.0[positions::FREE_PARKING];
                    if let Space::FreeParking(amount) = free_parking {
                        *amount += fine;
                    }
                    *player.2 -= Money(fine);
                    info!(
                        "Player {:?} paid ${} in taxes (current money: ${})",
                        player.0, fine, player.2 .0
                    );
                } else {
                    Self::handle_not_enough_money(player);
                }
            }
            board::Space::Chance => {
                let card = draw_chance_card(chance_cards);
                match card {
                    board::ChanceCard::AdvanceToGo => {
                        *player.3 = Position(positions::GO);
                        *player.2 += Money(200);
                        info!(
                            "Player {:?} advanced to `Go` and collected $200 (current money: ${})",
                            player.0, player.2 .0
                        );
                    }
                    board::ChanceCard::AdvanceToIllinois => {
                        // Handle if player passes Go
                        if player.3 .0 > positions::ILLINOIS_AVENUE {
                            *player.2 += Money(200);
                            info!(
                                "Player {:?} passed `Go` and collected $200 (current money: ${})",
                                player.0, player.2 .0
                            );
                        }

                        *player.3 = Position(positions::ILLINOIS_AVENUE);
                        info!("Player {:?} advanced to `Illinois Avenue`", player.0);

                        // TODO: Handle property
                        let illinois_avenue = &mut board.0[positions::ILLINOIS_AVENUE];
                        if let Space::Property(property) = illinois_avenue {
                            if property.owner.is_none() {
                                // TODO: Can buy/auction
                            } else {
                                // TODO: Pay owner
                            }
                        }
                    }
                    board::ChanceCard::AdvanceToStCharlesPlace => {
                        // Handle if player passes Go
                        if player.3 .0 > positions::ST_CHARLES_PLACE {
                            *player.2 += Money(200);
                            info!(
                                "Player {:?} passed `Go` and collected $200 (current money: ${})",
                                player.0, player.2 .0
                            );
                        }

                        *player.3 = Position(positions::ST_CHARLES_PLACE);
                        info!("Player {:?} advanced to `St. Charles Place`", player.0);

                        // TODO: Handle property
                    }
                    board::ChanceCard::AdvanceToNearestUtility => {
                        // TODO: List all utilities
                        //  - Sub player's positions, and move to smallest (non-negative) one
                        let utilities = [positions::ELECTRIC_COMPANY, positions::WATER_WORKS];
                        let player_pos = player.3 .0;

                        // Find closest utility
                        let nearest = utilities
                            .iter()
                            .filter_map(|u| {
                                let distance = *u as isize - player_pos as isize;
                                if distance >= 0 {
                                    Some(u)
                                } else {
                                    None
                                }
                            })
                            .min()
                            .expect("Unable to find nearest utility");

                        // Move player to nearest utility
                        *player.3 = Position(*nearest);

                        // TODO: Buy if available, pay 10x dice otherwise
                    }
                    board::ChanceCard::AdvanceToNearestRailroad => todo!(),
                    board::ChanceCard::Dividend => todo!(),
                    board::ChanceCard::GetOutOfJailFree => todo!(),
                    board::ChanceCard::GoBack3Spaces => todo!(),
                    board::ChanceCard::GoToJail => todo!(),
                    board::ChanceCard::GeneralRepairs => todo!(),
                    board::ChanceCard::AdvanceToReadingRailroad => todo!(),
                    board::ChanceCard::PoorTax => todo!(),
                    board::ChanceCard::AdvanceToBoardwalk => todo!(),
                    board::ChanceCard::ChairmanOfTheBoard => todo!(),
                    board::ChanceCard::BuildingLoanMatures => todo!(),
                    board::ChanceCard::HolidayFundMatures => todo!(),
                }
            }
            board::Space::Jail => {
                // TODO: Implement
            }
            board::Space::FreeParking(_) => {
                // TODO: Implement
            }
            board::Space::GoToJail => {
                // TODO: Implement
            }
            board::Space::Property(property) => {
                // TODO: Implement
            }
        }

        // TODO: Handle bankruptcy
    }

    fn handle_not_enough_money(
        mut player: &mut (
            &Id,
            Mut<'_, OwnedProperties>,
            Mut<'_, Money>,
            Mut<'_, Position>,
            Mut<'_, InJail>,
            Mut<'_, HasGetOutOfJailFreeCard>,
        ),
    ) {
        // TODO: Implement!
    }

    /// Rolls two D6s (six-sided dice).
    fn roll_dice() -> (usize, usize) {
        let die1 = rand::thread_rng().gen_range(1..=6);
        let die2 = rand::thread_rng().gen_range(1..=6);
        (die1, die2)
    }
}
