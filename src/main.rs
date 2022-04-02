use bevy::prelude::*;

mod start;
mod game;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Start,
    Game,
    Finish,
}

fn main() {
    App::new()
        .add_state(AppState::Start)
        .insert_resource(WindowDescriptor {
            title: "Crazy Cats".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(start::StartAnimation)
        .add_plugin(game::Game)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(spawn_camera)
        .add_system(handle_input)
        .run()
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn handle_input(keys: Res<Input<KeyCode>>, app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::Up) {
        change_state(app_state);
    }
}

fn change_state(mut app_state: ResMut<State<AppState>>) {
    match app_state.current() {
        AppState::Start => app_state.set(AppState::Game).unwrap(),
        AppState::Game => app_state.set(AppState::Start).unwrap(),
        _ => ()
    }
}