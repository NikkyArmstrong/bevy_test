use bevy::prelude::*;
use crate::constants::*;

/**************
 * MENU SETUP
 **************/

pub struct Menu;
impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app
            // Menu
            .add_systems(
                OnEnter(GameState::Menu), 
                setup_menu
            )
            .add_systems(
                Update, 
                update_menu.run_if(in_state(GameState::Menu))
            )
            .add_systems(
                OnExit(GameState::Menu), 
                cleanup_menu
            );
    }
}

#[derive(Resource)]
pub struct MenuData {
    button_entity: Entity,
}
 
pub fn setup_menu(mut commands: Commands) {
    let button_entity = commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn(
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(65.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "New Game",
                            TextStyle {
                                font_size: 40.,
                                color: TEXT_COLOUR.into(),
                                ..default()
                            }
                        ));
                    });
        }).id();

        commands.insert_resource(MenuData { button_entity });
}

pub fn update_menu(mut next_state: ResMut<NextState<GameState>>,
                   mut interaction_query: Query<(&Interaction, &mut BackgroundColor),
                                                (Changed<Interaction>, With<Button>)>) 
{
    for (interaction, mut colour) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *colour = PRESSED_BUTTON.into();
                next_state.set(GameState::SetupGame);
            }
            Interaction::Hovered => {
                *colour = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *colour = NORMAL_BUTTON.into();
            }
        }
    }
}
 
pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}
 