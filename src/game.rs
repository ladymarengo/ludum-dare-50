use bevy::prelude::*;
use super::AppState;

#[derive(Component)]
pub struct GameMarker;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                .with_system(spawn_game))
            .add_system_set(
                SystemSet::on_exit(AppState::Game)
                .with_system(cleanup_game)
            );
    }
}

fn spawn_game(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(400.0, 300.0, 0.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.0, 1.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(GameMarker);
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}