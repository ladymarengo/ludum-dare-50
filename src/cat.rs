use super::*;
use crate::game::*;
use benimator::*;
use bevy::prelude::*;
use heron::*;
use instant::Instant;
use rand::prelude::Rng;
use std::time::Duration;

#[derive(Component)]
pub struct Cat;

#[derive(Component)]
pub struct MoveTime(pub Instant);

#[derive(Component)]
pub struct BadTime(pub Instant);

#[derive(Component)]
pub struct LookTime(pub Instant);

#[derive(Component)]
pub struct RunTime(pub Instant);

#[derive(Component)]
pub struct Bad(pub bool);

#[derive(Component)]
pub struct GoAway(pub bool);

#[derive(Component)]
pub struct Running(pub bool);

#[derive(Default)]
pub struct Animations {
    good: Handle<SpriteSheetAnimation>,
    bad: Handle<SpriteSheetAnimation>,
    run: Handle<SpriteSheetAnimation>,
}

pub fn spawn_cats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    mut handles: ResMut<Animations>,
) {
    let texture = asset_server.load("cat1.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(500.0, 500.0), 3, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    handles.good = animations.add(SpriteSheetAnimation::from_range(
        0..=0,
        Duration::from_millis(100),
    ));

    handles.bad = animations.add(SpriteSheetAnimation::from_range(
        1..=1,
        Duration::from_millis(100),
    ));

    handles.run = animations.add(SpriteSheetAnimation::from_range(
        1..=5,
        Duration::from_millis(30),
    ).once());

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(200.0, 200.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(SensorShape)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(100.0 / 2.0, 100.0 / 2.0, 0.0),
            border_radius: None,
        })
        .insert(GameMarker)
        .insert(Cat)
        .insert(Seen(false))
        .insert(MoveTime(Instant::now()))
        .insert(BadTime(Instant::now()))
        .insert(LookTime(Instant::now()))
        .insert(RunTime(Instant::now()))
        .insert(Bad(false))
        .insert(GoAway(false))
        .insert(Running(false))
        .insert(handles.good.clone())
        .insert(Play);
}

pub fn cat_move(
    mut commands: Commands,
    mut cats: Query<
        (
            Entity,
            &mut Transform,
            &mut MoveTime,
            &mut Bad,
            &mut BadTime,
            &mut GoAway,
            &mut Handle<SpriteSheetAnimation>,
            &Seen,
            &Running,
        ),
        With<Cat>,
    >,
    places: Query<(&Transform, &Seen, Option<&BadPlace>), (With<Place>, Without<Cat>)>,
    animations: Res<Animations>,
) {
    for (id, mut cat, mut time, mut bad, mut badtime, mut go_away, mut animation, cat_seen, running) in
        cats.iter_mut()
    {
        if !running.0
            && (go_away.0 || (!cat_seen.0 && time.0.elapsed().as_millis() > 500 && !bad.0))
        {
            let mut rng = ::rand::thread_rng();
            let multiplier;
            if !go_away.0 {
                multiplier = 3;
            } else {
                multiplier = 1;
            }
            let mut new_place = rng.gen_range(0..places.iter().count() * multiplier);
            for (place, seen, bad_place) in places.iter() {
                if let (true, Some(_bp)) = (go_away.0, bad_place) {
                    continue;
                }
                new_place -= 1;
                if new_place == 0 {
                    if seen.0 == false && place.translation != cat.translation {
                        cat.translation = place.translation;
                        if let Some(_bp) = bad_place {
                            *animation = animations.bad.clone();
                            bad.0 = true;
                            badtime.0 = Instant::now();
                        } else {
                            *animation = animations.good.clone();
                        }
                        commands.entity(id).insert(Play);
                        go_away.0 = false;
                        break;
                    }
                }
            }
            time.0 = Instant::now();
        }
    }
}

pub fn check_defeat(
    cats: Query<(&mut Bad, &mut BadTime), With<Cat>>,
    mut app_state: ResMut<State<AppState>>,
) {
    for (bad, bad_time) in cats.iter() {
        if bad.0 && bad_time.0.elapsed().as_millis() > 5000 {
            app_state.set(AppState::Start).unwrap();
        }
    }
}

pub fn go_away(
    mut cats: Query<
        (
            &mut Bad,
            &LookTime,
            &mut GoAway,
            &mut Seen,
            &mut Running,
            &mut RunTime,
            &mut Handle<SpriteSheetAnimation>,
        ),
        With<Cat>,
    >,
    animations: Res<Animations>,
) {
    for (mut bad, look_time, mut go_away, mut seen, mut running, mut run_time, mut animation) in
        cats.iter_mut()
    {
        if !running.0 && bad.0 && seen.0 && look_time.0.elapsed().as_millis() > 1000 {
            bad.0 = false;
            go_away.0 = true;
            seen.0 = false;
            running.0 = true;
            run_time.0 = Instant::now();
            *animation = animations.run.clone();
        }
    }
}

pub fn stop_running(mut cats: Query<(&mut Running, &mut RunTime), With<Cat>>) {
    for (mut running, mut run_time) in cats.iter_mut() {
        if running.0 && run_time.0.elapsed().as_millis() > 100 {
            running.0 = false;
            run_time.0 = Instant::now();
        }
    }
}
