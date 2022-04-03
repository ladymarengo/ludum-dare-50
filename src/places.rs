use super::*;
use crate::game::*;
use bevy::prelude::*;

pub fn spawn_places(mut commands: Commands, asset_server: Res<AssetServer>) {
    
	let good_places = vec![
        (-263.0, 185.0),
        (-325.0, -100.0),
        (-250.0, -35.0),
        (-85.0, -65.0),
        (140.0, -65.0),
    ];

	let bad_places = vec![
		(20.0, 185.0),
		(-240.0, -215.0), 
		(40.0, -180.0), 
		(310.0, 7.0), 
		(330.0, -190.0), 
    ];

	for (x, y) in good_places.iter() {
		commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(*x, *y, 1.0),
                scale: Vec3::new(100.0, 100.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 1.0, 0.0),
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
        .insert(Place)
        .insert(Seen(false))
		.insert(GoodPlace);

	}

	for (x, y) in bad_places.iter() {
		commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(*x, *y, 1.0),
                scale: Vec3::new(100.0, 100.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgba(1.0, 0.0, 0.0, 0.0),
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
        .insert(Place)
        .insert(Seen(false))
		.insert(BadPlace);
	}
}
