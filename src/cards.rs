use bevy::prelude::*;

use crate::ui::card_ui::CardToUILink;

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

const DRIVING_ACE: i32 = 1;
const EXTRA_TANK: i32 = 1;
const PUNCTURE_PROOF: i32 = 1;
const RIGHT_OF_WAY: i32 = 1;

const TWENTY_FIVE: i32 = 10;
const FIFTY: i32 = 10;
const SEVENTY_FIVE: i32 = 10;
const ONE_HUNDRED: i32 = 12;
const TWO_HUNDRED: i32 = 4;

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
                          spawn_safeties,
                          spawn_distances)
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

fn spawn_safeties(mut commands: Commands)
{
    for _i in 0..DRIVING_ACE {
        commands.spawn(DrivingAce::default());
    }

    for _i in 0..EXTRA_TANK {
        commands.spawn(ExtraTank::default());
    }

    for _i in 0..PUNCTURE_PROOF {
        commands.spawn(PunctureProof::default());
    }

    for _i in 0..RIGHT_OF_WAY {
        commands.spawn(RightOfWay::default());
    }
}

fn spawn_distances(mut commands: Commands)
{
    for _i in 0..TWENTY_FIVE {
        commands.spawn(TwentyFive::default());
    }

    for _i in 0..FIFTY {
        commands.spawn(Fifty::default());
    }

    for _i in 0..SEVENTY_FIVE {
        commands.spawn(SeventyFive::default());
    }

    for _i in 0..ONE_HUNDRED {
        commands.spawn(OneHundred::default());
    }

    for _i in 0..TWO_HUNDRED {
        commands.spawn(TwoHundred::default());
    }
}

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
    fn is_valid(board_card_type: &SubType, card_type: &CardType, sub_type: &SubType) -> bool;
}

#[derive(Component)]
pub struct PlayerBoard;

#[derive(Component)]
pub struct OpponentBoard;

#[derive(Component, Eq, PartialEq)]
pub enum CardType {
    Hazard,
    Remedy,
    Safety,
    Distance
}

#[derive(Component, Eq, PartialEq, Copy, Clone)]
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
    Roll,
    PunctureProof,
    ExtraTank,
    DrivingAce,
    RightOfWay,
    TwentyFive,
    Fifty,
    SeventyFive,
    OneHundred,
    TwoHundred,
    NoCard
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
    card_tag: Card,
    ui_entity: CardToUILink
}
impl Default for CardBundle {
    fn default() -> Self {
        Self {
            card_name: CardName("EMPTY".into()),
            card_type: CardType::Hazard,
            sub_type: SubType::Accident,
            action_type: ActionType::Offensive,
            card_tag: Card,
            ui_entity: CardToUILink { ui_entity: Entity::PLACEHOLDER },
        }
    }
}

#[derive(Component)]
pub struct Card;

impl Playable for Card {
    fn is_valid(board_card_type: &SubType, card_type: &CardType, sub_type: &SubType) -> bool {
        match card_type {
            CardType::Hazard => Hazard::is_valid(board_card_type, card_type, sub_type),
            CardType::Remedy => Remedy::is_valid(board_card_type, card_type, sub_type),
            CardType::Safety => Safety::is_valid(board_card_type, card_type, sub_type),
            CardType::Distance => Distance::is_valid(board_card_type, card_type, sub_type),
        }
    }
}

/*****************************
* Hazards
******************************/

#[derive(Component)]
pub struct Hazard;

impl Playable for Hazard {
    fn is_valid(board_card_type: &SubType, _card_type: &CardType, _sub_type: &SubType) -> bool {
        return *board_card_type == SubType::Roll;
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
                ..Default::default()
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
                ..Default::default()
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
                ..Default::default()
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
                ..Default::default()
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
                ..Default::default()
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
    fn is_valid(board_card_type: &SubType, card_type: &CardType, sub_type: &SubType) -> bool {
        match sub_type {
            SubType::Repairs => Repairs::is_valid(board_card_type, card_type, sub_type),
            SubType::Gasoline => Gasoline::is_valid(board_card_type, card_type, sub_type),
            SubType::EndOfLimit => EndOfLimit::is_valid(board_card_type, card_type, sub_type),
            SubType::SpareTyre => SpareTyre::is_valid(board_card_type, card_type, sub_type),
            SubType::Roll => Roll::is_valid(board_card_type, card_type, sub_type),
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
                ..Default::default()
            },
            remedy: Remedy
        }
    }
}
impl Playable for Repairs {
    fn is_valid(board_card_type: &SubType, _card_type: &CardType, _sub_type: &SubType) -> bool {
        return *board_card_type == SubType::Accident;
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
                ..Default::default()
            },
            remedy: Remedy
        }
    }
}
impl Playable for Gasoline {
    fn is_valid(board_card_type: &SubType, _card_type: &CardType, _sub_type: &SubType) -> bool {
        return *board_card_type == SubType::OutOfGas;
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
                ..Default::default()
            },
            remedy: Remedy
        }
    }
}
impl Playable for SpareTyre {
    fn is_valid(board_card_type: &SubType, _card_type: &CardType, _sub_type: &SubType) -> bool {
        return *board_card_type == SubType::FlatTyre;
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
                ..Default::default()
            },
            remedy: Remedy
        }
    }
}
impl Playable for EndOfLimit {
    fn is_valid(board_card_type: &SubType, _card_type: &CardType, _sub_type: &SubType) -> bool {
        return *board_card_type == SubType::SpeedLimit;
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
                ..Default::default()
            },
            remedy: Remedy
        }
    }
}
impl Playable for Roll {
    fn is_valid(board_card_type: &SubType, _card_type: &CardType, _sub_type: &SubType) -> bool {
        match board_card_type {
            SubType::Accident => return false,
            SubType::OutOfGas => return false,
            SubType::SpeedLimit => return false,
            SubType::FlatTyre => return false,
            SubType::Roll => return false,
            _ => return true,
        }
    }
}

// /*****************************
// * Safeties
// ******************************/
#[derive(Component)]
pub struct Safety;
impl Playable for Safety {
    fn is_valid(_board_card_type: &SubType, _card_type: &CardType, _sub_type: &SubType) -> bool {
        // it is always valid to play a safety
        // todo : effects of playing cards
        
        return true;
    }
}

#[derive(Bundle)]
pub struct DrivingAce
{
    card: CardBundle,
    safety: Safety
}
impl Default for DrivingAce {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Driving Ace".into()),
                card_type: CardType::Safety,
                sub_type: SubType::DrivingAce,
                action_type: ActionType::Defensive,
                ..Default::default()
            },
            safety: Safety
        }
    }
}

#[derive(Bundle)]
pub struct ExtraTank
{
    card: CardBundle,
    safety: Safety
}
impl Default for ExtraTank {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Extra Tank".into()),
                card_type: CardType::Safety,
                sub_type: SubType::ExtraTank,
                action_type: ActionType::Defensive,
                ..Default::default()
            },
            safety: Safety
        }
    }
}

#[derive(Bundle)]
pub struct PunctureProof
{
    card: CardBundle,
    safety: Safety
}
impl Default for PunctureProof {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Puncture Proof".into()),
                card_type: CardType::Safety,
                sub_type: SubType::PunctureProof,
                action_type: ActionType::Defensive,
                ..Default::default()
            },
            safety: Safety
        }
    }
}

#[derive(Bundle)]
pub struct RightOfWay
{
    card: CardBundle,
    safety: Safety
}
impl Default for RightOfWay {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("Right of Way".into()),
                card_type: CardType::Safety,
                sub_type: SubType::RightOfWay,
                action_type: ActionType::Defensive,
                ..Default::default()
            },
            safety: Safety
        }
    }
}

/*****************************
* Distances
******************************/
#[derive(Component)]
pub struct Distance;
impl Playable for Distance {
    fn is_valid(board_card_type: &SubType, _card_type: &CardType, _sub_type: &SubType) -> bool {
        return *board_card_type == SubType::Roll;
    }
}

#[derive(Bundle)]
pub struct TwentyFive
{
    card: CardBundle,
    distance: Distance
}
impl Default for TwentyFive {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("25km".into()),
                card_type: CardType::Distance,
                sub_type: SubType::TwentyFive,
                action_type: ActionType::Defensive,
                ..Default::default()
            },
            distance: Distance
        }
    }
}

#[derive(Bundle)]
pub struct Fifty
{
    card: CardBundle,
    distance: Distance
}
impl Default for Fifty {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("50km".into()),
                card_type: CardType::Distance,
                sub_type: SubType::Fifty,
                action_type: ActionType::Defensive,
                ..Default::default()
            },
            distance: Distance
        }
    }
}

#[derive(Bundle)]
pub struct SeventyFive
{
    card: CardBundle,
    distance: Distance
}
impl Default for SeventyFive {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("75km".into()),
                card_type: CardType::Distance,
                sub_type: SubType::SeventyFive,
                action_type: ActionType::Defensive,
                ..Default::default()
            },
            distance: Distance
        }
    }
}

#[derive(Bundle)]
pub struct OneHundred
{
    card: CardBundle,
    distance: Distance
}
impl Default for OneHundred {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("100km".into()),
                card_type: CardType::Distance,
                sub_type: SubType::OneHundred,
                action_type: ActionType::Defensive,
                ..Default::default()
            },
            distance: Distance
        }
    }
}

#[derive(Bundle)]
pub struct TwoHundred
{
    card: CardBundle,
    distance: Distance
}
impl Default for TwoHundred {
    fn default() -> Self {
        Self {
            card: CardBundle {
                card_name: CardName("200km".into()),
                card_type: CardType::Distance,
                sub_type: SubType::TwoHundred,
                action_type: ActionType::Defensive,
                ..Default::default()
            },
            distance: Distance
        }
    }
}

