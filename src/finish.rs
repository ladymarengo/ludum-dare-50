use bevy::prelude::*;
use super::AppState;
use super::*;
use crate::places::*;
use crate::game::*;

#[derive(Component)]
pub struct FinishMarker;

#[derive(Component)]
pub struct PointLabel;

pub struct Finish;

impl Plugin for Finish {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Finish)
                .with_system(spawn_finish))
			.add_system_set(
				SystemSet::on_update(AppState::Finish)
				.with_system(update_text))
            .add_system_set(
                SystemSet::on_exit(AppState::Finish)
                .with_system(cleanup_finish)
            );
    }
}

pub fn spawn_finish(mut commands: Commands, asset_server: Res<AssetServer>, assets: Res<LoadedAssets>, place: Res<FinalPlace>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
		.spawn_bundle(TextBundle {
			style: Style {
				align_self: AlignSelf::FlexEnd,
				position_type: PositionType::Absolute,
				position: Rect {
					bottom: Val::Px(270.0),
					right: Val::Px(250.0),
					..Default::default()
				},
				..Default::default()
			},
			text: Text::with_section(
				"points",
				TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: 30.0,
					color: Color::WHITE,
				},
				TextAlignment {
					horizontal: HorizontalAlign::Center,
					vertical: VerticalAlign::Center,
					..Default::default()
				},
			),
			..Default::default()
		})
		.insert(PointLabel)
		.insert(FinishMarker);

	let path;

	match place.0 {
		CurrentPlace::Good => path = "room.png".to_string(),
		CurrentPlace::Vase => path = "broken_vase.png".to_string(),
		CurrentPlace::Wire => path = "broken_wire.png".to_string(),
		CurrentPlace::Photo => path = "broken_photo.png".to_string(),
		CurrentPlace::Chair => path = "broken_chair.png".to_string(),
		CurrentPlace::Glass => path = "broken_glass.png".to_string(),
	}
	
	let texture = assets.0.get(&path).unwrap();

	commands
		.spawn_bundle(SpriteBundle {
			texture: texture.clone(),
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
		.insert(FinishMarker);
        
}

pub fn update_text(mut points_label: Query<&mut Text, With<PointLabel>>, points: Res<Points>) {
    let section = &mut points_label.single_mut().sections[0];
    section.value = format!("You survived {} seconds.\nPress Space to try again.", points.0);
}

pub fn cleanup_finish(mut commands: Commands, query: Query<Entity, With<FinishMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}