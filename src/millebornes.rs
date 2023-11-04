// Create the Mille Bornes plugin
use bevy::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::constants::*;
use crate::cards::*;
use crate::menu::*;
use crate::ui::board_ui::create_board_ui;
use crate::ui::board_ui::update_board_ui;
use crate::ui::card_ui::CardToUILink;
use crate::ui::card_ui::UIToCardLink;
use crate::ui::card_ui::get_card_colour;

pub struct MilleBornes;

impl Plugin for MilleBornes {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Cards)
            .add_plugins(Menu)
            .insert_resource(ClearColor(BACKGROUND_COLOUR))
            .add_state::<GameState>()
            .add_state::<TurnState>()
            // Resources 
            .init_resource::<Game>()
            .init_resource::<GameRules>()
            .init_resource::<Score>()
            .add_systems(
                Startup,
                setup_camera
            )
            // Game Setup
            .add_systems(
                OnEnter(GameState::SetupGame), (
                    setup_game,
                    create_board_ui,
                    begin_game
                ).chain().after(CardSet::CardInit)
            )
            // Game Start
            .add_systems(
                OnEnter(GameState::BeginGame), (
                    deal,
                    apply_deferred.after(deal),
                    next_turn
                ).chain()
            )
            .add_systems(
                OnEnter(TurnState::PlayerTurn),
                draw_player_card
            )
            .add_systems(
                OnEnter(TurnState::OpponentTurn),
                draw_opponent_card
            )
            .add_systems(
                PreUpdate,
                update_board_ui.run_if(in_state(GameState::DuringTurn))
            )
            // Player Turn
            .add_systems(
                Update, 
                process_player_turn.run_if(in_state(TurnState::PlayerTurn))
            )
            // Opponent Turn
            .add_systems(Update,
                process_opponent_turn.run_if(in_state(TurnState::OpponentTurn))
            )
            .add_systems(
                PostUpdate, (
                    despawn_old_ui.run_if(in_state(GameState::DuringTurn)),
                    next_turn.run_if(in_state(GameState::NextTurn))
                )
            );
    }
}

#[derive(Resource, Default)]
struct Game {
    deck: Vec<Entity>
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

#[derive(Resource, Default)]
struct Score {
    player_score: i32,
    opponent_score: i32
}

/*************
 * GAME SETUP
 *************/

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_game(mut game: ResMut<Game>, mut commands: Commands, 
              card_query: Query<(Entity, &Card)>)
{
    for (entity, _card) in card_query.iter()
    {
        game.deck.push(entity);

        // Tag the card as being in the deck
        commands.entity(entity).insert(Deck);
    }

    game.deck.shuffle(&mut thread_rng());
}

fn begin_game(mut next_state: ResMut<NextState<GameState>>)
{
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

fn draw_player_card(mut game: ResMut<Game>, mut commands: Commands)
{
    if let Some(card) = game.deck.pop() {
        commands.entity(card).remove::<Deck>().insert(PlayerHand);
    }
}

fn draw_opponent_card(mut game: ResMut<Game>, mut commands: Commands)
{
    if let Some(card) = game.deck.pop() {
        commands.entity(card).remove::<Deck>().insert(OpponentHand);
    }
}

/************
 * GAME LOOP
 ************/
fn process_player_turn(mut interaction_query: Query<(&Interaction, &UIToCardLink, &mut BackgroundColor),
                                             (Changed<Interaction>, With<Button>)>,
                       mut commands: Commands,
                       card_query: Query<(&CardName, &CardType, &SubType), With<PlayerHand>>,
                       board_query: Query<(Entity, &CardType, &SubType), With<PlayerBoard>>,
                       mut next_turn: ResMut<NextState<GameState>>) 
{
    for (interaction, ui_link, mut colour) in &mut interaction_query {

        if let Ok((card_name, card_type, sub_type)) = card_query.get(ui_link.card_entity) {
            let mut board_card_type = SubType::NoCard;
            if board_query.get_single().is_ok() {
                let (_board, _board_card, board_sub_card) = board_query.single();
                board_card_type = *board_sub_card;
            }

            if Card::is_valid(&board_card_type, card_type, sub_type) {
                match *interaction {
                    Interaction::Pressed => {
                        *colour = PRESSED_BUTTON.into();
                        println!("Player clicked {}", card_name.0);
                        if let Ok((entity, _, _)) = board_query.get_single() {
                            commands.entity(entity).remove::<PlayerBoard>();
                        }
                        
                        commands.entity(ui_link.card_entity).insert(PlayerBoard);
                        commands.entity(ui_link.card_entity).remove::<PlayerHand>();

                        next_turn.set(GameState::NextTurn);
                    }
                    Interaction::Hovered => {
                        *colour = HOVERED_BUTTON.into();
                    }
                    Interaction::None => {
                        *colour = get_card_colour(card_type).into()
                    }
                }
            }
            else {
                *colour = get_card_colour(card_type).into();
            }
        }
    }
}

fn process_opponent_turn(mut interaction_query: Query<(&Interaction, &UIToCardLink, &mut BackgroundColor),
                                                      (Changed<Interaction>, With<Button>)>,
                         mut commands: Commands,
                         card_query: Query<(&CardName, &CardType, &SubType), With<OpponentHand>>,
                         board_query: Query<(Entity, &CardType, &SubType), With<OpponentBoard>>,
                         mut next_turn: ResMut<NextState<GameState>>) 
{
    for (interaction, ui_link, mut colour) in &mut interaction_query {

        if let Ok((card_name, card_type, sub_type)) = card_query.get(ui_link.card_entity) {
            let mut board_card_type = SubType::NoCard;
            if board_query.get_single().is_ok() {
                board_card_type = *board_query.single().2;
            }

            if Card::is_valid(&board_card_type, card_type, sub_type) {
                match *interaction {
                    Interaction::Pressed => {
                        *colour = PRESSED_BUTTON.into();
                        println!("Opponent clicked {}", card_name.0);
                        if let Ok((entity, _, _)) = board_query.get_single() {
                            commands.entity(entity).remove::<OpponentBoard>();
                        }

                        commands.entity(ui_link.card_entity).insert(OpponentBoard);
                        commands.entity(ui_link.card_entity).remove::<OpponentHand>();

                        next_turn.set(GameState::NextTurn);
                    }
                    Interaction::Hovered => {
                        *colour = HOVERED_BUTTON.into();
                    }
                    Interaction::None => {
                        *colour = get_card_colour(card_type).into()
                    }
                }
            }
            else {
                *colour = get_card_colour(card_type).into();
            }
        }
    }
}

fn despawn_old_ui(mut commands: Commands,
                  mut player_board_removals: RemovedComponents<PlayerBoard>,
                  mut player_card_removals: RemovedComponents<PlayerHand>,
                  mut opponent_board_removals: RemovedComponents<OpponentBoard>,
                  mut opponent_card_removals: RemovedComponents<OpponentHand>,
                  query: Query<&CardToUILink>)
{
    for entity in player_board_removals.iter() {
        if let Ok(ui_entity) = query.get(entity) {
            commands.entity(ui_entity.ui_entity).despawn_recursive();
        }
    }

    for entity in player_card_removals.iter() {
        if let Ok(ui_entity) = query.get(entity) {
            commands.entity(ui_entity.ui_entity).despawn_recursive();
        }
    }

    for entity in opponent_board_removals.iter() {
        if let Ok(ui_entity) = query.get(entity) {
            commands.entity(ui_entity.ui_entity).despawn_recursive();
        }
    }

    for entity in opponent_card_removals.iter() {
        if let Ok(ui_entity) = query.get(entity) {
            commands.entity(ui_entity.ui_entity).despawn_recursive();
        }
    }
}

fn next_turn(current_state: Res<State<TurnState>>,
             mut next_state: ResMut<NextState<TurnState>>,
             mut next_game_state: ResMut<NextState<GameState>>)
{
    match current_state.get() {
        TurnState::PlayerTurn => next_state.set(TurnState::OpponentTurn),
        TurnState::OpponentTurn => next_state.set(TurnState::PlayerTurn),
        TurnState::NoTurn => next_state.set(TurnState::PlayerTurn),
    }

    next_game_state.set(GameState::DuringTurn);
}

// todo
// board
// better UI plugin
// ai