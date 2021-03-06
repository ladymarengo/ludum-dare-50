use std::collections::HashMap;

use benimator::*;
use bevy::prelude::*;
use heron::*;

mod cat;
mod finish;
mod game;
mod logo_hint;
mod places;
mod start;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Logo,
    Start,
    Hint,
    Game,
    Finish,
}

pub struct Points(pub u32);

fn main() {
    App::new()
        .add_state(AppState::Logo)
        .insert_resource(WindowDescriptor {
            title: "Crazy Cats".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Points(0))
        .insert_resource(LoadedAssets(HashMap::new()))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(AnimationPlugin::default())
        .add_plugin(logo_hint::Logo)
        .add_plugin(logo_hint::Hint)
        .add_plugin(start::StartAnimation)
        .add_plugin(game::Game)
        .add_plugin(finish::Finish)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(spawn_camera)
        .add_startup_system(load_assets)
        .add_system(handle_input)
        .run()
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn handle_input(keys: Res<Input<KeyCode>>, app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::Space) {
        change_state(app_state);
    }
}

fn change_state(mut app_state: ResMut<State<AppState>>) {
    match app_state.current() {
        AppState::Logo => app_state.set(AppState::Start).unwrap(),
        AppState::Start => app_state.set(AppState::Hint).unwrap(),
        AppState::Hint => app_state.set(AppState::Game).unwrap(),
        AppState::Finish => app_state.set(AppState::Hint).unwrap(),
        _ => (),
    }
}

pub struct LoadedAssets(HashMap<String, Handle<Image>>);

fn load_assets(mut assets: ResMut<LoadedAssets>, asset_server: Res<AssetServer>) {
    let names = [
        "broken_chair.png",
        "broken_wire.png",
        "broken_vase.png",
        "broken_photo.png",
        "broken_glass.png",
        "cat1.png",
        "cat2.png",
        "hint.png",
        "logo.png",
        "room_transparent.png",
        "room.png",
        "start_seq.png",
        "torch.png",
    ];

    for name in names {
        assets.0.insert(name.to_string(), asset_server.load(name));
    }
}
