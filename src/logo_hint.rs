use bevy::prelude::*;
use super::*;

#[derive(Component)]
pub struct LogoMarker;

pub struct Logo;

impl Plugin for Logo {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Logo)
                .with_system(spawn_logo))
            .add_system_set(
                SystemSet::on_exit(AppState::Logo)
                .with_system(cleanup_logo)
            );
    }
}

fn spawn_logo(mut commands: Commands, assets: Res<LoadedAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.0.get("logo.png").unwrap().clone(),
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
        .insert(LogoMarker);
}

fn cleanup_logo(mut commands: Commands, query: Query<Entity, With<LogoMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Component)]
pub struct HintMarker;

pub struct Hint;

impl Plugin for Hint {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Hint)
                .with_system(spawn_hint))
            .add_system_set(
                SystemSet::on_exit(AppState::Hint)
                .with_system(cleanup_hint)
            );
    }
}

fn spawn_hint(mut commands: Commands, assets: Res<LoadedAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.0.get("hint.png").unwrap().clone(),
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
        .insert(HintMarker);
}

fn cleanup_hint(mut commands: Commands, query: Query<Entity, With<HintMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}