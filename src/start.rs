use bevy::prelude::*;
use super::AppState;
use benimator::*;
use std::time::Duration;

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
                SystemSet::on_update(AppState::Start)
                .with_system(removal_detection))
            .add_system_set(
                SystemSet::on_exit(AppState::Start)
                .with_system(cleanup_start)
            );
    }
}

fn spawn_start(mut commands: Commands, asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>) {

    let texture = asset_server.load("start_seq.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(1600.0, 1200.0), 5, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_handle = animations.add(SpriteSheetAnimation::from_range(
        0..=23,
        Duration::from_millis(1000),
    ).once());
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite {
            index: 0,
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(StartAnimationMarker)
    .insert(animation_handle)
    .insert(Play);
}

fn cleanup_start(mut commands: Commands, query: Query<Entity, With<StartAnimationMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn removal_detection(animation: Query<Option<&Play>, With<StartAnimationMarker>>, mut app_state: ResMut<State<AppState>>) {
    let ent = animation.single();
    if let None = ent {
        app_state.set(AppState::Hint).unwrap();
    }
}
