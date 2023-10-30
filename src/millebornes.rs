// Create the Mille Bornes plugin
use bevy::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::constants::*;
use crate::cards::*;
use crate::menu::*;

pub struct MilleBornes;

impl Plugin for MilleBornes {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Cards)
            .add_plugins(Menu)
            .insert_resource(ClearColor(BACKGROUND_COLOUR))
            .add_state::<GameState>()
            // Resources 
            .init_resource::<Game>()
            .init_resource::<GameRules>()
            .add_systems(
                Startup,
                setup_camera
            )
            // Game Setup
            .add_systems(
                OnEnter(GameState::SetupGame), 
                setup_game.after(CardSet::CardInit)
            )
            // Game Start
            .add_systems(
                OnEnter(GameState::BeginGame), (
                    deal,
                    apply_deferred.after(deal),
                    draw_board_ui,
                    start_game
                ).chain()
            )
            // Player Turn
            .add_systems(
                Update, (
                    update_cards,
                    process_player_turn
                ).run_if(in_state(GameState::PlayerTurn))
            )
            // Opponent Turn
            .add_systems(Update, 
                process_opponent_turn.run_if(in_state(GameState::OpponentTurn))
            );
    }
}

#[derive(Resource, Default)]
struct Game {
    deck: Vec<Entity>
}

#[derive(Resource)]
struct GameRules {
    // miles: i32,
    hand_size: i32
}
impl Default for GameRules {
    fn default() -> Self {
        Self {
            // miles: 700,
            hand_size: 6
        }
    }
}

/*************
 * GAME SETUP
 *************/

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_game(mut game: ResMut<Game>, mut commands: Commands, query: Query<(Entity, &Card)>, mut next_state: ResMut<NextState<GameState>>)
{
    for (entity, _card) in query.iter()
    {
        game.deck.push(entity);

        // Tag the card as being in the deck
        commands.entity(entity).insert(Deck);
    }

    game.deck.shuffle(&mut thread_rng());

    next_state.set(GameState::BeginGame);
}

fn deal(game_rules: Res<GameRules>, mut game: ResMut<Game>, mut commands: Commands)
{
    for _i in 0..game_rules.hand_size {
        // This is safe because if this panics something went wrong in setup
        let player_card = game.deck.pop().unwrap();
        let opponent_card = game.deck.pop().unwrap();

        commands.entity(player_card).remove::<Deck>().insert(PlayerHand);
        commands.entity(opponent_card).remove::<Deck>().insert(OpponentHand);
    }
}

fn draw_board_ui(mut commands: Commands,
                 player_cards: Query<(Entity, &CardName, &CardType), (With<PlayerHand>, Without<OpponentHand>)>,
                 opponent_cards: Query<(Entity, &CardName, &CardType), (With<OpponentHand>, Without<PlayerHand>)>) 
{
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

    for (entity, card_name, card_type) in player_cards.iter() {
        let player_card = build_card_ui(&card_name.0, card_type, entity, &mut commands);
        commands.entity(player_card_holder).push_children(&[player_card]);
    }

    for (entity, card_name, card_type) in opponent_cards.iter() {
        let opponent_card = build_card_ui(&card_name.0, card_type, entity, &mut commands);
        commands.entity(opponent_card_holder).push_children(&[opponent_card]);
    }

    commands.entity(holder).push_children(&[player_card_holder, opponent_card_holder]);
}

// Link the UI representation of a card button to its physical entity
#[derive(Component)]
struct UILink {
    entity: Entity,
}

fn build_card_ui(name: &String, card_type: &CardType, card_entity: Entity, commands: &mut Commands) -> Entity {
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
                        // todo change to percent
                        width: Val::Px(200.),
                        height: Val::Px(250.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Center,
                        ..default()
                    },
                    background_color: get_card_colour(card_type).into(),
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

/************
 * GAME LOOP
 ************/

fn start_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerTurn);
}

fn update_cards(mut interaction_query: Query<(&Interaction, &UILink, &mut BackgroundColor),
                                             (Changed<Interaction>, With<Button>)>,
                card_query: Query<(&CardName, &CardType)>) 
{
    for (interaction, ui_link, mut colour) in &mut interaction_query {
        let card = card_query.get(ui_link.entity).unwrap();
        
        match *interaction {
            Interaction::Pressed => {
                *colour = PRESSED_BUTTON.into();
                println!("Clicked {}", card.0.0);
            }
            Interaction::Hovered => {
                *colour = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *colour = get_card_colour(card.1).into()
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