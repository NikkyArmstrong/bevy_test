// Create the Mille Bornes plugin
use bevy::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::{cards::{Cards, Card, CardName}, menu::{setup_menu, cleanup_menu, PRESSED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, TEXT_COLOUR}};

pub struct MilleBornes;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    SetupGame,
    PlayerTurn,
    OpponentTurn
}

impl Plugin for MilleBornes {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Cards)
            .add_state::<GameState>()
            .init_resource::<Game>()
            .init_resource::<GameRules>()
            .add_systems(OnEnter(GameState::Menu), (setup_camera, setup_menu))
            .add_systems(Update, update_menu.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            .add_systems(OnEnter(GameState::SetupGame), (setup_game, deal, draw_board_ui, start_game).chain())
            .add_systems(Update, update_cards.run_if(in_state(GameState::PlayerTurn)))
            .add_systems(Update, process_player_turn.run_if(in_state(GameState::PlayerTurn)))
            .add_systems(Update, process_opponent_turn.run_if(in_state(GameState::OpponentTurn)));
    }
}

#[derive(Resource, Default)]
struct Game {
    deck: Vec<Entity>,
    player_hand: Vec<Entity>,
    opponent_hand: Vec<Entity>,
    discard_pile: Vec<Entity>
}

#[derive(Resource)]
struct GameRules {
    miles: i32,
    hand_size: i32
}
impl Default for GameRules {
    fn default() -> Self {
        Self {
            miles: 700,
            hand_size: 6
        }
    }
}

/*****
 * UPDATES
 */

fn update_menu(mut next_state: ResMut<NextState<GameState>>,
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
/*************
 * GAME SETUP
 *************/

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_game(mut game: ResMut<Game>, query: Query<(Entity, &Card)>)
{
    for (entity, _card) in query.iter()
    {
        game.deck.push(entity);
    }

    game.deck.shuffle(&mut thread_rng());
}

fn deal(mut game: ResMut<Game>, game_rules: Res<GameRules>)
{
    for _i in 0..game_rules.hand_size {

        // todo - this will break if deck is empty
        let player_card = game.deck.pop().unwrap();
        let opponent_card = game.deck.pop().unwrap();

        game.player_hand.push(player_card);
        game.opponent_hand.push(opponent_card);
    }
}

fn draw_board_ui(game: Res<Game>, mut commands: Commands, card_name_query: Query<&CardName>) {
    let holder = commands.spawn(
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Percent(25.),
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
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
                justify_content: JustifyContent::Center,
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
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }
    ).id();

    for entity in &game.player_hand {
        let player_card = build_card_ui(&card_name_query.get(*entity).unwrap().0, *entity, &mut commands);
        commands.entity(player_card_holder).push_children(&[player_card]);
    }

    for entity in &game.opponent_hand {
        let opponent_card = build_card_ui(&card_name_query.get(*entity).unwrap().0, *entity, &mut commands);
        commands.entity(opponent_card_holder).push_children(&[opponent_card]);
    }

    commands.entity(holder).push_children(&[player_card_holder, opponent_card_holder]);
}

#[derive(Component)]
struct UILink {
    entity: Entity,
}

fn build_card_ui(name: &String, card_entity: Entity, commands: &mut Commands) -> Entity {
    let mut binding = commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        });
    let node_bundle = binding.with_children(|parent| {
            parent.spawn((
                UILink {
                    entity: card_entity
                },
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(250.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })).with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            name.clone(),
                            TextStyle {
                                font_size: 32.,
                                color: TEXT_COLOUR.into(),
                                ..default()
                            }
                        ));
                    });
        });

    return node_bundle.id();
}

/****
 * GAME LOOP
 */

fn start_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerTurn);
}

fn update_cards(mut interaction_query: Query<(&Interaction, &UILink, &mut BackgroundColor),
                                             (Changed<Interaction>, With<Button>)>,
                card_name_query: Query<&CardName>) 
{
    for (interaction, ui_link, mut colour) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *colour = PRESSED_BUTTON.into();
                println!("Clicked {}", card_name_query.get(ui_link.entity).unwrap().0);
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

fn process_player_turn() {

}

fn process_opponent_turn() {

}

// todo
// remaining deck
// board
// better UI plugin
// ai
// player controls