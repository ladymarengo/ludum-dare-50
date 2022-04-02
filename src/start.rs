use bevy::prelude::*;
use super::AppState;

#[derive(Component)]
pub struct StartAnimationMarker;

pub struct StartAnimation;

impl Plugin for StartAnimation {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Start)
                .with_system(spawn_start))
            .add_system_set(
                SystemSet::on_exit(AppState::Start)
                .with_system(cleanup_start)
            );
    }
}

fn spawn_start(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("test/start_test.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(StartAnimationMarker);
}

fn cleanup_start(mut commands: Commands, query: Query<Entity, With<StartAnimationMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
