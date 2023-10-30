use bevy::prelude::*;

use crate::constants::{HAZARD_CARD, REMEDY_CARD, SAFETY_CARD, DISTANCE_CARD};

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

pub fn get_card_colour(card_type: &CardType) -> Color {
    match card_type {
        CardType::Hazard => HAZARD_CARD,
        CardType::Remedy => REMEDY_CARD,
        CardType::Safety => SAFETY_CARD,
        CardType::Distance => DISTANCE_CARD,
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

#[derive(Component)]
pub struct Card;

#[derive(Component, Eq, PartialEq)]
pub enum CardType {
    Hazard,
    Remedy,
    Safety,
    Distance
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
    card_type: CardType,
    action_type: ActionType
}

/*****************************
* Hazards
******************************/
#[derive(Bundle)]
struct Hazard 
{
    card_config: CardBundle
}
impl Default for Hazard {
    fn default() -> Self {
        Self {
            card_config: CardBundle {
                card_type: CardType::Hazard,
                action_type: ActionType::Offensive
            }
        }
    }
}

#[derive(Bundle)]
pub struct Accident
{
    card: Card,
    category: Hazard, 
    name: CardName
}

impl Default for Accident {
    fn default() -> Self {
        Self {
            card: Card,
            category: Hazard::default(),
            name: CardName("Accident".into())
        }
    }
}

#[derive(Bundle)]
pub struct OutOfGas
{
    card: Card,
    category: Hazard,
    name: CardName
}
impl Default for OutOfGas {
    fn default() -> Self {
        Self {
            card: Card,
            category: Hazard::default(),
            name: CardName("Out of Gas".into())
        }
    }
}

#[derive(Bundle)]
pub struct FlatTyre
{
    card: Card,
    category: Hazard, 
    name: CardName
}
impl Default for FlatTyre {
    fn default() -> Self {
        Self {
            card: Card,
            category: Hazard::default(),
            name: CardName("Flat Tyre".into())
        }
    }
}

#[derive(Bundle)]
pub struct SpeedLimit
{
    card: Card,
    category: Hazard, 
    name: CardName
}

impl Default for SpeedLimit {
    fn default() -> Self {
        Self {
            card: Card,
            category: Hazard::default(),
            name: CardName("Speed Limit".into())
        }
    }
}

#[derive(Bundle)]
pub struct Stop
{
    card: Card,
    category: Hazard,
    name: CardName
}

impl Default for Stop {
    fn default() -> Self {
        Self {
            card: Card,
            category: Hazard::default(),
            name: CardName("Stop".into())
        }
    }
}

/*****************************
* Remedies
******************************/
#[derive(Bundle)]
struct Remedy 
{
    card_config: CardBundle
}
impl Default for Remedy {
    fn default() -> Self {
        Self {
            card_config: CardBundle {
                card_type: CardType::Remedy,
                action_type: ActionType::Defensive
            }
        }
    }
}

#[derive(Bundle)]
struct Repairs
{
    card: Card,
    category: Remedy,
    name: CardName
}
impl Default for Repairs {
    fn default() -> Self {
        Self {
            card: Card,
            category: Remedy::default(),
            name: CardName("Repairs".into())
        }
    }
}

#[derive(Bundle)]
struct Gasoline
{
    card: Card,
    category: Remedy,
    name: CardName
}
impl Default for Gasoline {
    fn default() -> Self {
        Self {
            card: Card,
            category: Remedy::default(),
            name: CardName("Gasoline".into())
        }
    }
}

#[derive(Bundle)]
struct SpareTyre
{
    card: Card,
    category: Remedy,
    name: CardName
}
impl Default for SpareTyre {
    fn default() -> Self {
        Self {
            card: Card,
            category: Remedy::default(),
            name: CardName("Spare Tyre".into())
        }
    }
}

#[derive(Bundle)]
struct EndOfLimit
{
    card: Card,
    category: Remedy,
    name: CardName
}
impl Default for EndOfLimit {
    fn default() -> Self {
        Self {
            card: Card,
            category: Remedy::default(),
            name: CardName("End of Limit".into())
        }
    }
}

#[derive(Bundle)]
struct Roll
{
    card: Card,
    category: Remedy,
    name: CardName
}
impl Default for Roll {
    fn default() -> Self {
        Self {
            card: Card,
            category: Remedy::default(),
            name: CardName("Roll".into())
        }
    }
}

/*****************************
* Safeties
******************************/
#[derive(Bundle)]
struct Safety
{
    card_config: CardBundle,
}
impl Default for Safety {
    fn default() -> Self {
        Self {
            card_config: CardBundle {
                card_type: CardType::Safety,
                action_type: ActionType::Defensive
            }
        }
    }
}

#[derive(Bundle)]
struct DrivingAce
{
    card: Card,
    category: Safety,
    name: CardName
}
impl Default for DrivingAce {
    fn default() -> Self {
        Self {
            card: Card,
            category: Safety::default(),
            name: CardName("Driving Ace".into())
        }
    }
}

#[derive(Bundle)]
struct ExtraTank
{
    card: Card,
    category: Safety,
    name: CardName
}
impl Default for ExtraTank {
    fn default() -> Self {
        Self {
            card: Card,
            category: Safety::default(),
            name: CardName("Extra Tank".into())
        }
    }
}

#[derive(Bundle)]
struct PunctureProof
{
    card: Card,
    category: Safety,
    name: CardName
}
impl Default for PunctureProof {
    fn default() -> Self {
        Self {
            card: Card,
            category: Safety::default(),
            name: CardName("Puncture Proof".into())
        }
    }
}

#[derive(Bundle)]
struct RightOfWay
{
    card: Card,
    category: Safety,
    name: CardName
}
impl Default for RightOfWay {
    fn default() -> Self {
        Self {
            card: Card,
            category: Safety::default(),
            name: CardName("Right of Way".into())
        }
    }
}

/*****************************
* Distances
******************************/
#[derive(Bundle)]
struct Distance
{
    card_config: CardBundle
}
impl Default for Distance {
    fn default() -> Self {
        Self {
            card_config: CardBundle {
                card_type: CardType::Distance,
                action_type: ActionType::Defensive
            }
        }
    }
}

#[derive(Bundle)]
struct TwentyFive
{
    card: Card,
    category: Distance,
    name: CardName
}
impl Default for TwentyFive {
    fn default() -> Self {
        Self {
            card: Card,
            category: Distance::default(),
            name: CardName("25km".into())
        }
    }
}

#[derive(Bundle)]
struct Fifty
{
    card: Card,
    category: Distance,
    name: CardName
}
impl Default for Fifty {
    fn default() -> Self {
        Self {
            card: Card,
            category: Distance::default(),
            name: CardName("50km".into())
        }
    }
}

#[derive(Bundle)]
struct SeventyFive
{
    card: Card,
    category: Distance,
    name: CardName
}
impl Default for SeventyFive {
    fn default() -> Self {
        Self {
            card: Card,
            category: Distance::default(),
            name: CardName("75km".into())
        }
    }
}

#[derive(Bundle)]
struct OneHundred
{
    card: Card,
    category: Distance,
    name: CardName
}
impl Default for OneHundred {
    fn default() -> Self {
        Self {
            card: Card,
            category: Distance::default(),
            name: CardName("100km".into())
        }
    }
}

#[derive(Bundle)]
struct TwoHundred
{
    card: Card,
    category: Distance,
    name: CardName
}
impl Default for TwoHundred {
    fn default() -> Self {
        Self {
            card: Card,
            category: Distance::default(),
            name: CardName("200km".into())
        }
    }
}
