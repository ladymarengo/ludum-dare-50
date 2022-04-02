use bevy::prelude::*;
use heron::*;
use super::*;
use crate::game::*;
use instant::Instant;
use rand::prelude::Rng;

#[derive(Component)]
pub struct Cat;

#[derive(Component)]
pub struct MoveTime(Instant);

pub fn spawn_cats(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("test/cat_test.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 2.0),
            ..Default::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(200.0, 200.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(RigidBody::Static)
    .insert(SensorShape)
    .insert(CollisionShape::Sphere { radius: 20.0 })
    .insert(GameMarker)
    .insert(Cat)
    .insert(Seen(false))
    .insert(MoveTime(Instant::now()));
}

pub fn cat_move(mut cats: Query<(&mut Transform, &mut MoveTime), With<Cat>>, places: Query<(&Transform, &Seen), (With<Place>, Without<Cat>)>) {
    for (mut cat, mut time) in cats.iter_mut() {
        if time.0.elapsed().as_millis() > 500 {
            loop {
                let mut rng = ::rand::thread_rng();
                let mut new_place = rng.gen_range(0..places.iter().count());
                for (place, seen) in places.iter() {
                    new_place -= 1;
                    
                    if new_place == 0 {
                        if seen.0 == false && place.translation != cat.translation {
                            cat.translation = place.translation;
                            time.0 = Instant::now();
                            return;
                        }
                    }
                }
            }
        }
    }
}