use bevy::prelude::{States, Color};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    SetupGame,
    BeginGame,
    DuringTurn,
    NextTurn
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TurnState {
    #[default]
    NoTurn,
    PlayerTurn,
    OpponentTurn
}

pub const BACKGROUND_COLOUR: Color = Color::rgb(0.91, 0.86, 0.79);

pub const NORMAL_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const TEXT_COLOUR: Color = Color::rgb(0.9, 0.9, 0.9);

pub const HAZARD_CARD: Color = Color::RED;
pub const REMEDY_CARD: Color = Color::DARK_GREEN;
pub const SAFETY_CARD: Color = Color::PURPLE;
pub const DISTANCE_CARD: Color = Color::MIDNIGHT_BLUE;
