use bevy::prelude::*;

/**************
 * MENU SETUP
 **************/

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
pub const TEXT_COLOUR: Color = Color::rgb(0.9, 0.9, 0.9);
 
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
 
pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}
 