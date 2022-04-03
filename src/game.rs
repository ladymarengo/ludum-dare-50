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

fn spawn_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("room.png"),
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
        texture: asset_server.load("room_transparent.png"),
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
            texture: asset_server.load("torch.png"),
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
    mut events: EventReader<CollisionEvent>,
    torch: Query<Entity, With<Torch>>,
    mut good_places: Query<(
        Entity,
        &mut Transform,
        &mut Seen,
        Option<&Place>,
        Option<&Bad>,
        Option<&mut LookTime>,
    )>,
) {
    let torch = torch.single();
    for event in events.iter() {
        match event {
            CollisionEvent::Started(t, some_place) if t.rigid_body_entity() == torch => {
                for (place, mut place_transform, mut seen, is_place, bad, look_time) in
                    good_places.iter_mut()
                {
                    if place == some_place.rigid_body_entity() {
                        if let Some(_is_place) = is_place {
                            place_transform.scale = Vec3::new(10.0, 10.0, 0.0);
                        }
                        if let Some(mut look_time) = look_time {
                            if let Some(bad) = bad {
                                if bad.0 {
                                    look_time.0 = Instant::now();
                                }
                            }
                        }
                        seen.0 = true;
                    }
                }
            }
            CollisionEvent::Stopped(t, some_place) if t.rigid_body_entity() == torch => {
                for (place, mut place_transform, mut seen, is_place, _bad, _look_time) in
                    good_places.iter_mut()
                {
                    if place == some_place.rigid_body_entity() {
                        if let Some(_is_place) = is_place {
                            place_transform.scale = Vec3::new(100.0, 100.0, 0.0);
                        }
                        seen.0 = false;
                    }
                }
            }
            _ => (),
        }
    }
}

pub fn update_points(mut points: ResMut<Points>, time: Res<GameTime>) {
    points.0 = (time.0.elapsed().as_millis() / 1000) as u32;
}

pub fn reset_time(mut time: ResMut<GameTime>) {
    time.0 = Instant::now();
}

