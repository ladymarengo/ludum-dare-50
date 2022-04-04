use super::*;
use crate::cat::*;
use crate::places::*;
use instant::Instant;

#[derive(Component)]
pub struct GameMarker;

#[derive(Component)]
pub struct Torch;

#[derive(Component)]
pub struct Place;

#[derive(Component)]
pub struct GoodPlace;

#[derive(Component)]
pub struct BadPlace;

#[derive(Component)]
pub struct Seen(pub bool);

pub struct GameTime(pub Instant);

pub struct Game;

pub struct FinalPlace(pub CurrentPlace);

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<cat::Animations>()
        .insert_resource(GameTime(Instant::now()))
        .insert_resource(FinalPlace(CurrentPlace::Good))
        .add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(spawn_game)
                .with_system(spawn_places)
                .with_system(reset_time)
                .with_system(spawn_cats),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(move_torch)
                .with_system(check_collisions)
                .with_system(cat_move)
                .with_system(check_defeat)
                .with_system(stop_running)
                .with_system(update_points)
                .with_system(go_away),
        )
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(cleanup_game));
    }
}

fn spawn_game(mut commands: Commands, assets: Res<LoadedAssets>) {

    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.0.get("room.png").unwrap().clone(),
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

    commands
    .spawn_bundle(SpriteBundle {
        texture: assets.0.get("room_transparent.png").unwrap().clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 3.0),
            ..Default::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(GameMarker);

    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.0.get("torch.png").unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(1800.0, 1400.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(SensorShape)
        .insert(CollisionShape::Cuboid {
			half_extends: Vec3::new(80.0 / 2.0, 80.0 / 2.0, 0.0),
			border_radius: None,
		})
        .insert(GameMarker)
        .insert(Torch);
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn move_torch(
    mut torch: Query<&mut Transform, With<Torch>>,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
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

fn check_collisions(
    torch: Query<&Transform, With<Torch>>,
    mut objects: Query<(
        &Transform,
        &mut Seen,
        Option<&Bad>,
        Option<&mut LookTime>,
    )>,
) {
    let torch = torch.single();

    for (object, mut seen, bad, look_time) in objects.iter_mut() {
        if (torch.translation.x - object.translation.x).abs() < 100.0 && (torch.translation.y - object.translation.y).abs() < 100.0 {
            seen.0 = true;
            if let Some(mut look_time) = look_time {
                if let Some(bad) = bad {
                    if bad.0 {
                        look_time.0 += 1;
                    }
                }
            }
        } else {
            seen.0 = false;
            if let Some(mut look_time) = look_time {
                if let Some(bad) = bad {
                    if bad.0 {
                        look_time.0 = 0;
                    }
                }
            }
        }
    }
}

pub fn update_points(mut points: ResMut<Points>, time: Res<GameTime>) {
    points.0 = (time.0.elapsed().as_millis() / 1000) as u32;
}

pub fn reset_time(mut time: ResMut<GameTime>) {
    time.0 = Instant::now();
}

