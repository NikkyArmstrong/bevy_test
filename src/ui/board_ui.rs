use bevy::prelude::*;
use crate::cards::*;
use super::card_ui::{build_card_ui, UIToCardLink, CardToUILink};

#[derive(Resource)]
pub struct BoardUI {
    player_hand: Entity,
    opponent_hand: Entity,
    play_area: Entity,
    opponent_play_area: Entity
}

pub fn create_board_ui(mut commands: Commands) {
    let board = commands.spawn(
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Percent(10.),
                width: Val::Percent(100.),
                justify_content: JustifyContent::SpaceEvenly,
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

    let board_card_holder = commands.spawn(
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

    let player_area = commands.spawn(
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

    let opponent_area = commands.spawn(
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

    commands.entity(board_card_holder).push_children(&[player_area, opponent_area]);
    commands.entity(board).push_children(&[player_card_holder, board_card_holder, opponent_card_holder]);
    commands.insert_resource(BoardUI { 
        player_hand: player_card_holder,
        opponent_hand: opponent_card_holder,
        play_area: player_area,
        opponent_play_area: opponent_area
    });
}

pub fn update_board_ui(mut commands: Commands, board_ui: Res<BoardUI>,
    card_ui_query: Query<&UIToCardLink>,
    mut player_cards: Query<(Entity, &mut CardToUILink, &CardName, &CardType), (With<PlayerHand>, Without<OpponentHand>)>,
    mut opponent_cards: Query<(Entity, &mut CardToUILink, &CardName, &CardType), (With<OpponentHand>, Without<PlayerHand>)>,
    mut player_board_card: Query<(Entity, &mut CardToUILink, &CardName, &CardType), (With<PlayerBoard>, Without<OpponentBoard>, Without<PlayerHand>, Without<OpponentHand>)>,
    mut opponent_board_card: Query<(Entity, &mut CardToUILink, &CardName, &CardType), (With<OpponentBoard>, Without<PlayerBoard>, Without<OpponentHand>, Without<PlayerHand>)>)
{

    if let Ok((entity, mut ui_entity, card_name, card_type)) = player_board_card.get_single_mut() {
        if !card_ui_query.contains(ui_entity.ui_entity) {
            let board_card = build_card_ui(&card_name.0, &card_type, entity, &mut commands);

            ui_entity.ui_entity = board_card;
            commands.entity(board_ui.play_area).push_children(&[board_card]);
        }
    }

    if let Ok((entity, mut ui_entity, card_name, card_type)) = opponent_board_card.get_single_mut() {
        if !card_ui_query.contains(ui_entity.ui_entity) {
            let board_card = build_card_ui(&card_name.0, &card_type, entity, &mut commands);

            ui_entity.ui_entity = board_card;
            commands.entity(board_ui.opponent_play_area).push_children(&[board_card]);
        }
    }

    for (entity, mut ui_entity, card_name, card_type) in player_cards.iter_mut() {
        if !card_ui_query.contains(ui_entity.ui_entity) {
            let player_card = build_card_ui(&card_name.0, &card_type, entity, &mut commands);

            ui_entity.ui_entity = player_card;
            commands.entity(board_ui.player_hand).push_children(&[player_card]);
        }
    }

    for (entity, mut ui_entity, card_name, card_type) in opponent_cards.iter_mut() {
        if !card_ui_query.contains(ui_entity.ui_entity) {
            let opponent_card = build_card_ui(&card_name.0, &card_type, entity, &mut commands);

            ui_entity.ui_entity = opponent_card;
            commands.entity(board_ui.opponent_hand).push_children(&[opponent_card]);
        }
    }
}