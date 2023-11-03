// Create the Mille Bornes plugin
use bevy::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::constants::*;
use crate::cards::*;
use crate::menu::*;
use crate::ui::board_ui::draw_board_ui;
use crate::ui::card_ui::UILink;
use crate::ui::card_ui::get_card_colour;

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
                    apply_deferred.after(update_cards),
                    process_player_turn,
                    //draw_board_ui
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

fn setup_game(mut game: ResMut<Game>, mut commands: Commands, 
              card_query: Query<(Entity, &Card)>,
              mut next_state: ResMut<NextState<GameState>>)
{
    for (entity, _card) in card_query.iter()
    {
        game.deck.push(entity);

        // Tag the card as being in the deck
        commands.entity(entity).insert(Deck);
    }

    game.deck.shuffle(&mut thread_rng());
    println!("{}", game.deck.len());
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

/************
 * GAME LOOP
 ************/

fn start_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerTurn);
}

fn update_cards(mut interaction_query: Query<(Entity, &Interaction, &UILink, &mut BackgroundColor),
                                             (Changed<Interaction>, With<Button>)>,
                mut commands: Commands,
                card_query: Query<(&CardName, &CardType, &SubType)>,
                board_query: Query<(Entity, &CardType, &SubType), With<PlayerBoard>>) 
{
    for (ui_entity, interaction, ui_link, mut colour) in &mut interaction_query {
        let (card_name, card_type, sub_type) = card_query.get(ui_link.entity).unwrap();
        
        if Card::is_valid(&board_query, card_type, sub_type) {
            match *interaction {
                Interaction::Pressed => {
                    *colour = PRESSED_BUTTON.into();
                    println!("Clicked {}", card_name.0);
                    if let Ok((entity, _, _)) = board_query.get_single() {
                        commands.entity(entity).remove::<PlayerBoard>();
                        // todo remove ui here
                    }
                    
                    commands.entity(ui_link.entity).insert(PlayerBoard);
                    commands.entity(ui_link.entity).remove::<PlayerHand>();
                    commands.entity(ui_entity).despawn_recursive();

                }
                Interaction::Hovered => {
                    *colour = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *colour = get_card_colour(card_type).into()
                }
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