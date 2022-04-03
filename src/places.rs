use super::*;
use crate::game::*;

#[derive(Component, Clone, Copy)]
pub struct Current(pub CurrentPlace);

#[derive(Clone, Copy)]
pub enum CurrentPlace {
	Chair,
	Glass,
	Photo,
	Vase,
	Wire,
	Good
}

pub fn spawn_places(mut commands: Commands) {
    
	let good_places = vec![
        (-263.0, 185.0, CurrentPlace::Good),
        (-325.0, -100.0, CurrentPlace::Good),
        (-250.0, -35.0, CurrentPlace::Good),
        (-85.0, -65.0, CurrentPlace::Good),
        (140.0, -65.0, CurrentPlace::Good),
    ];

	let bad_places = vec![
		(20.0, 185.0, CurrentPlace::Photo),
		(-240.0, -215.0, CurrentPlace::Chair), 
		(40.0, -180.0, CurrentPlace::Glass), 
		(310.0, 7.0, CurrentPlace::Vase), 
		(330.0, -190.0, CurrentPlace::Wire), 
    ];

	for (x, y, place) in good_places.iter() {
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
		.insert(GoodPlace)
		.insert(Current(*place));

	}

	for (x, y, place) in bad_places.iter() {
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
		.insert(BadPlace)
		.insert(Current(*place));
	}
}
