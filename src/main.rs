pub mod millebornes;
pub mod cards;
pub mod menu;
pub mod constants;
pub mod ui;

use bevy::prelude::*;
use millebornes::MilleBornes;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MilleBornes))
        .run();
}
