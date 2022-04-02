use bevy::prelude::*;
use heron::*;
use super::*;
use crate::game::*;

#[derive(Component)]
pub struct Cat;

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
    .insert(Seen(false));
}

pub fn cat_move(mut cats: )