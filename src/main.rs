use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, add_people)
        .add_systems(Update, (greet_people, greet_pets));
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Nikky".to_string())));
    commands.spawn((Person, Name("Rory".to_string())));
    commands.spawn((Dog, Name("Growlithe".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn greet_pets(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Dog>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("AW YEAH {}!", name.0.to_uppercase())
        }
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Dog;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);