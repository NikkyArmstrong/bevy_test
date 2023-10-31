use bevy::prelude::*;
use crate::cards::*;
use super::card_ui::build_card_ui;

pub fn draw_board_ui(mut commands: Commands,
    player_cards: Query<(Entity, &CardName, &CardType), (With<PlayerHand>, Without<OpponentHand>)>,
    opponent_cards: Query<(Entity, &CardName, &CardType), (With<OpponentHand>, Without<PlayerHand>)>) 
{
    let board = commands.spawn(
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Percent(25.),
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(20.), Val::Px(20.)),
                ..default()
            },
            ..default()
        }
    ).id();

    let player_card_holder = commands.spawn(
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }
    ).id();

    let opponent_card_holder = commands.spawn(
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }
    ).id();

    for (entity, card_name, card_type) in player_cards.iter() {
        let player_card = build_card_ui(&card_name.0, card_type, entity, &mut commands);
        commands.entity(player_card_holder).push_children(&[player_card]);
    }

    for (entity, card_name, card_type) in opponent_cards.iter() {
        let opponent_card = build_card_ui(&card_name.0, card_type, entity, &mut commands);
        commands.entity(opponent_card_holder).push_children(&[opponent_card]);
    }

    commands.entity(board).push_children(&[player_card_holder, opponent_card_holder]);
}