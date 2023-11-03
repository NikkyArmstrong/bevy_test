use bevy::prelude::*;

// Card type initialisation consts
const ACCIDENT: i32 = 3;
const OUT_OF_GAS: i32 = 3;
const FLAT_TYRE: i32 = 3;
const SPEED_LIMIT: i32 = 4;
const STOP: i32 = 5;

const REPAIRS: i32 = 6;
const GASOLINE: i32 = 6;
const SPARE_TYRE: i32 = 6;
const END_OF_LIMIT: i32 = 6;
const ROLL: i32 = 14;

// const DRIVING_ACE: i32 = 1;
// const EXTRA_TANK: i32 = 1;
// const PUNCTURE_PROOF: i32 = 1;
// const RIGHT_OF_WAY: i32 = 1;

// const TWENTY_FIVE: i32 = 10;
// const FIFTY: i32 = 10;
// const SEVENTY_FIVE: i32 = 10;
// const ONE_HUNDRED: i32 = 12;
// const TWO_HUNDRED: i32 = 4;

pub struct Cards;

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub enum CardSet {
    CardInit
}

impl Plugin for Cards {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup,
                (spawn_hazards,
                          spawn_remedies,
                          //spawn_safeties,
                          //spawn_distances
                        )
                        .in_set(CardSet::CardInit)
                    );
    }
}

fn spawn_hazards(mut commands: Commands)
{
    for _i in 0..ACCIDENT {
        commands.spawn(Accident::default());
    }

    for _i in 0..OUT_OF_GAS {
        commands.spawn(OutOfGas::default());
    }

    for _i in 0..FLAT_TYRE {
        commands.spawn(FlatTyre::default());
    }

    for _i in 0..SPEED_LIMIT {
        commands.spawn(SpeedLimit::default());
    }

    for _i in 0..STOP {
        commands.spawn(Stop::default());
    }
}

fn spawn_remedies(mut commands: Commands)
{
    for _i in 0..REPAIRS {
        commands.spawn(Repairs::default());
    }

    for _i in 0..GASOLINE {
        commands.spawn(Gasoline::default());
    }

    for _i in 0..SPARE_TYRE {
        commands.spawn(SpareTyre::default());
    }

    for _i in 0..END_OF_LIMIT {
        commands.spawn(EndOfLimit::default());
    }

    for _i in 0..ROLL {
        commands.spawn(Roll::default());
    }
}

// fn spawn_safeties(mut commands: Commands)
// {
//     for _i in 0..DRIVING_ACE {
//         commands.spawn(DrivingAce::default());
//     }

//     for _i in 0..EXTRA_TANK {
//         commands.spawn(ExtraTank::default());
//     }

//     for _i in 0..PUNCTURE_PROOF {
//         commands.spawn(PunctureProof::default());
//     }

//     for _i in 0..RIGHT_OF_WAY {
//         commands.spawn(RightOfWay::default());
//     }
// }

// fn spawn_distances(mut commands: Commands)
// {
//     for _i in 0..TWENTY_FIVE {
//         commands.spawn(TwentyFive::default());
//     }

//     for _i in 0..FIFTY {
//         commands.spawn(Fifty::default());
//     }

//     for _i in 0..SEVENTY_FIVE {
//         commands.spawn(SeventyFive::default());
//     }

//     for _i in 0..ONE_HUNDRED {
//         commands.spawn(OneHundred::default());
//     }

//     for _i in 0..TWO_HUNDRED {
//         commands.spawn(TwoHundred::default());
//     }
// }

// Ideally this would have an int indicating order of the deck 
// but ordered queries aren't supported yet
#[derive(Component)]
pub struct Deck;

#[derive(Component)]
pub struct PlayerHand;

#[derive(Component)]
pub struct OpponentHand;

#[derive(Component)]
pub struct DiscardPile;

// Play area components
pub trait Playable {
    fn is_valid(query: &Query<(Entity, &CardType, &SubType), With<PlayerBoard>>, card_type: &CardType, sub_type: &SubType) -> bool;
}

#[derive(Component)]
pub struct PlayerBoard;

#[derive(Component, Eq, PartialEq)]
pub enum CardType {
    Hazard,
    Remedy,
    Safety,
    Distance
}

#[derive(Component, Eq, PartialEq)]
pub enum SubType {
    Accident,
    OutOfGas,
    SpeedLimit,
    FlatTyre,
    Stop,
    Repairs,
    Gasoline,
    EndOfLimit,
    SpareTyre,
    Roll
}

#[derive(Component, Debug)]
pub struct CardName(pub String);

#[derive(Component, Eq, PartialEq)]
pub enum ActionType {
    Offensive,
    Defensive
}

#[derive(Bundle)]
pub struct CardBundle
{
    card_name: CardName,
    card_type: CardType,
    sub_type: SubType,
    action_type: ActionType,
    card_tag: Card
}

#[derive(Component)]
pub struct Card;

impl Playable for Card {
    fn is_valid(query: &Query<(Entity, &CardType, &SubType), With<PlayerBoard>>, card_type: &CardType, sub_type: &SubType) -> bool {
        match card_type {
            CardType::Hazard => Hazard::is_valid(query, card_type, sub_type),
            CardType::Remedy => Remedy::is_valid(query, card_type, sub_type),
            CardType::Safety => todo!(),
            CardType::Distance => todo!(),
        }
    }
}

/*****************************
* Hazards
******************************/

#[derive(Component)]
pub struct Hazard;

impl Playable for Hazard {
    fn is_valid(query: &Query<(Entity, &CardType, &SubType), With<PlayerBoard>>, _card_type: &CardType, _sub_type: &SubType) -> bool {
        if query.is_empty() {
            return false;
        }

        let (_card, _board_card_type, board_sub_type) = query.single();
        return *board_sub_type == SubType::Roll;
    }
}

#[derive(Bundle)]
pub struct Accident
{
    card: CardBundle,
    hazard: Hazard
}
impl Default for Accident {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Accident".into()),
                card_type: CardType::Hazard,
                sub_type: SubType::Accident,
                action_type: ActionType::Offensive,
                card_tag: Card
            },
            hazard: Hazard
        }
    }
}

#[derive(Bundle)]
pub struct FlatTyre
{
    card: CardBundle,
    hazard: Hazard
}
impl Default for FlatTyre {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Flat Tyre".into()),
                card_type: CardType::Hazard,
                sub_type: SubType::FlatTyre,
                action_type: ActionType::Offensive,
                card_tag: Card
            },
            hazard: Hazard
        }
    }
}

#[derive(Bundle)]
pub struct OutOfGas
{
    card: CardBundle,
    hazard: Hazard
}
impl Default for OutOfGas {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Out of Gas".into()),
                card_type: CardType::Hazard,
                sub_type: SubType::OutOfGas,
                action_type: ActionType::Offensive,
                card_tag: Card
            },
            hazard: Hazard
        }
    }
}

#[derive(Bundle)]
pub struct SpeedLimit
{
    card: CardBundle,
    hazard: Hazard
}
impl Default for SpeedLimit {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Speed Limit".into()),
                card_type: CardType::Hazard,
                sub_type: SubType::SpeedLimit,
                action_type: ActionType::Offensive,
                card_tag: Card
            },
            hazard: Hazard
        }
    }
}

#[derive(Bundle)]
pub struct Stop
{
    card: CardBundle,
    hazard: Hazard
}
impl Default for Stop {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Stop".into()),
                card_type: CardType::Hazard,
                sub_type: SubType::Stop,
                action_type: ActionType::Offensive,
                card_tag: Card
            },
            hazard: Hazard
        }
    }
}

/*****************************
* Remedies
******************************/
#[derive(Component)]
pub struct Remedy;
impl Playable for Remedy {
    fn is_valid(query: &Query<(Entity, &CardType, &SubType), With<PlayerBoard>>, card_type: &CardType, sub_type: &SubType) -> bool {
        match sub_type {
            SubType::Repairs => Repairs::is_valid(query, card_type, sub_type),
            SubType::Gasoline => Gasoline::is_valid(query, card_type, sub_type),
            SubType::EndOfLimit => EndOfLimit::is_valid(query, card_type, sub_type),
            SubType::SpareTyre => SpareTyre::is_valid(query, card_type, sub_type),
            SubType::Roll => Roll::is_valid(query, card_type, sub_type),
            _ => panic!()
        }
    }
}

#[derive(Bundle)]
pub struct Repairs
{
    card: CardBundle,
    remedy: Remedy
}
impl Default for Repairs {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Repairs".into()),
                card_type: CardType::Remedy,
                sub_type: SubType::Repairs,
                action_type: ActionType::Defensive,
                card_tag: Card
            },
            remedy: Remedy
        }
    }
}
impl Playable for Repairs {
    fn is_valid(query: &Query<(Entity, &CardType, &SubType), With<PlayerBoard>>, _card_type: &CardType, _sub_type: &SubType) -> bool {
        if let Ok((_card, _board_card_type, board_sub_type)) = query.get_single() {
            return *board_sub_type == SubType::Accident;
        }

        return false;
    }
}

#[derive(Bundle)]
pub struct Gasoline 
{
    card: CardBundle,
    remedy: Remedy
}
impl Default for Gasoline {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Gasoline".into()),
                card_type: CardType::Remedy,
                sub_type: SubType::Gasoline,
                action_type: ActionType::Defensive,
                card_tag: Card
            },
            remedy: Remedy
        }
    }
}
impl Playable for Gasoline {
    fn is_valid(query: &Query<(Entity, &CardType, &SubType), With<PlayerBoard>>, _card_type: &CardType, _sub_type: &SubType) -> bool {
        if let Ok((_card, _board_card_type, board_sub_type)) = query.get_single() {
            return *board_sub_type == SubType::OutOfGas;
        }

        return false;
    }
}

#[derive(Bundle)]
pub struct SpareTyre
{
    card: CardBundle,
    remedy: Remedy
}
impl Default for SpareTyre {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Spare Tyre".into()),
                card_type: CardType::Remedy,
                sub_type: SubType::SpareTyre,
                action_type: ActionType::Defensive,
                card_tag: Card
            },
            remedy: Remedy
        }
    }
}
impl Playable for SpareTyre {
    fn is_valid(query: &Query<(Entity, &CardType, &SubType), With<PlayerBoard>>, _card_type: &CardType, _sub_type: &SubType) -> bool {
        if let Ok((_card, _board_card_type, board_sub_type)) = query.get_single() {
            return *board_sub_type == SubType::FlatTyre;
        }

        return false;
    }
}

#[derive(Bundle)]
pub struct EndOfLimit
{
    card: CardBundle,
    remedy: Remedy
}
impl Default for EndOfLimit {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("End of Limit".into()),
                card_type: CardType::Remedy,
                sub_type: SubType::EndOfLimit,
                action_type: ActionType::Defensive,
                card_tag: Card
            },
            remedy: Remedy
        }
    }
}
impl Playable for EndOfLimit {
    fn is_valid(query: &Query<(Entity, &CardType, &SubType), With<PlayerBoard>>, _card_type: &CardType, _sub_type: &SubType) -> bool {
        if let Ok((_card, _board_card_type, board_sub_type)) = query.get_single() {
            return *board_sub_type == SubType::SpeedLimit;
        }

        return false;
    }
}

#[derive(Bundle)]
pub struct Roll
{
    card: CardBundle,
    remedy: Remedy
}
impl Default for Roll {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Roll".into()),
                card_type: CardType::Remedy,
                sub_type: SubType::Roll,
                action_type: ActionType::Defensive,
                card_tag: Card
            },
            remedy: Remedy
        }
    }
}
impl Playable for Roll {
    fn is_valid(query: &Query<(Entity, &CardType, &SubType), With<PlayerBoard>>, _card_type: &CardType, _sub_type: &SubType) -> bool {
        if let Ok((_card, board_card_type, board_sub_type)) = query.get_single() {
            return (*board_card_type == CardType::Remedy && *board_sub_type != SubType::Roll) || *board_sub_type == SubType::Stop;
        }

        // valid to play Roll if there are no cards in play
        return true;
    }
}

// /*****************************
// * Safeties
// ******************************/
// #[derive(Bundle)]
// struct Safety
// {
//     card_config: CardBundle,
// }
// impl Default for Safety {
//     fn default() -> Self {
//         Self {
//             card_config: CardBundle {
//                 card_type: CardType::Safety,
//                 action_type: ActionType::Defensive
//             }
//         }
//     }
// }

// #[derive(Bundle)]
// struct DrivingAce
// {
//     card: Card,
//     category: Safety,
//     name: CardName
// }
// impl Default for DrivingAce {
//     fn default() -> Self {
//         Self {
//             card: Card,
//             category: Safety::default(),
//             name: CardName("Driving Ace".into())
//         }
//     }
// }

// #[derive(Bundle)]
// struct ExtraTank
// {
//     card: Card,
//     category: Safety,
//     name: CardName
// }
// impl Default for ExtraTank {
//     fn default() -> Self {
//         Self {
//             card: Card,
//             category: Safety::default(),
//             name: CardName("Extra Tank".into())
//         }
//     }
// }

// #[derive(Bundle)]
// struct PunctureProof
// {
//     card: Card,
//     category: Safety,
//     name: CardName
// }
// impl Default for PunctureProof {
//     fn default() -> Self {
//         Self {
//             card: Card,
//             category: Safety::default(),
//             name: CardName("Puncture Proof".into())
//         }
//     }
// }

// #[derive(Bundle)]
// struct RightOfWay
// {
//     card: Card,
//     category: Safety,
//     name: CardName
// }
// impl Default for RightOfWay {
//     fn default() -> Self {
//         Self {
//             card: Card,
//             category: Safety::default(),
//             name: CardName("Right of Way".into())
//         }
//     }
// }

// /*****************************
// * Distances
// ******************************/
// #[derive(Bundle)]
// struct Distance
// {
//     card_config: CardBundle
// }
// impl Default for Distance {
//     fn default() -> Self {
//         Self {
//             card_config: CardBundle {
//                 card_type: CardType::Distance,
//                 action_type: ActionType::Defensive
//             }
//         }
//     }
// }

// #[derive(Bundle)]
// struct TwentyFive
// {
//     card: Card,
//     category: Distance,
//     name: CardName
// }
// impl Default for TwentyFive {
//     fn default() -> Self {
//         Self {
//             card: Card,
//             category: Distance::default(),
//             name: CardName("25km".into())
//         }
//     }
// }

// #[derive(Bundle)]
// struct Fifty
// {
//     card: Card,
//     category: Distance,
//     name: CardName
// }
// impl Default for Fifty {
//     fn default() -> Self {
//         Self {
//             card: Card,
//             category: Distance::default(),
//             name: CardName("50km".into())
//         }
//     }
// }

// #[derive(Bundle)]
// struct SeventyFive
// {
//     card: Card,
//     category: Distance,
//     name: CardName
// }
// impl Default for SeventyFive {
//     fn default() -> Self {
//         Self {
//             card: Card,
//             category: Distance::default(),
//             name: CardName("75km".into())
//         }
//     }
// }

// #[derive(Bundle)]
// struct OneHundred
// {
//     card: Card,
//     category: Distance,
//     name: CardName
// }
// impl Default for OneHundred {
//     fn default() -> Self {
//         Self {
//             card: Card,
//             category: Distance::default(),
//             name: CardName("100km".into())
//         }
//     }
// }

// #[derive(Bundle)]
// struct TwoHundred
// {
//     card: Card,
//     category: Distance,
//     name: CardName
// }
// impl Default for TwoHundred {
//     fn default() -> Self {
//         Self {
//             card: Card,
//             category: Distance::default(),
//             name: CardName("200km".into())
//         }
//     }
// }

