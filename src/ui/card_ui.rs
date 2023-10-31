use bevy::prelude::*;
use crate::cards::CardType;
use crate::constants::*;

pub fn get_card_colour(card_type: &CardType) -> Color {
    match card_type {
        CardType::Hazard => HAZARD_CARD,
        CardType::Remedy => REMEDY_CARD,
        CardType::Safety => SAFETY_CARD,
        CardType::Distance => DISTANCE_CARD,
    }
}

// Link the UI representation of a card button to its physical entity
#[derive(Component)]
pub struct UILink {
    pub entity: Entity,
}

pub fn build_card_ui(name: &String, card_type: &CardType, card_entity: Entity, commands: &mut Commands) -> Entity {
    let mut binding = commands.spawn((
                UILink {
                    entity: card_entity
                },
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(16.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Center,
                        ..default()
                    },
                    background_color: get_card_colour(card_type).into(),
                    ..default()
                }));
    let node_bundle = //= binding.with_children(|parent| { parent.spawn((
            binding.with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            name.clone(),
                            TextStyle {
                                font_size: 24.,
                                color: TEXT_COLOUR.into(),
                                ..default()
                            }
                        ));
                    });
        //});

    return node_bundle.id();
}
