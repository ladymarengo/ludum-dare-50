use bevy::prelude::*;
use super::*;

#[derive(Component)]
pub struct GameMarker;

#[derive(Component)]
pub struct Torch;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                .with_system(spawn_game))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                .with_system(move_torch))
            .add_system_set(
                SystemSet::on_exit(AppState::Game)
                .with_system(cleanup_game)
            );
    }
}

fn spawn_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("test/game_test.png"),
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
    .insert(GameMarker);

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("test/torch_test.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 2.0),
            ..Default::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(3000.0, 2000.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(GameMarker)
    .insert(Torch);
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn move_torch(mut torch: Query<&mut Transform, With<Torch>>, wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>) {
    
    let mut torch = torch.single_mut();
    let (camera, camera_transform) = q_camera.single();
    let wnd = wnds.get(camera.window).unwrap();

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        torch.translation.x = world_pos.x;
        torch.translation.y = world_pos.y;
    }
}