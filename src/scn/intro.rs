use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_xpbd_3d::{components::RigidBody, prelude::Collider};
use std::f32::consts::FRAC_PI_2;

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
	}
}

pub fn setup(
	mut cmds: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut mats: ResMut<Assets<StandardMaterial>>,
) {
	let panel_mesh = meshes.add(Cuboid::new(16.0, 16.0, 1.0));
	let panel_col = Collider::cuboid(16.0, 16.0, 1.0);
	let dark_gray = mats.add(Color::DARK_GRAY);

	cmds.spawn((
		PbrBundle {
			mesh: panel_mesh.clone(),
			material: dark_gray.clone(),
			transform: Transform::from_translation(Vec3::NEG_Z),
			..default()
		},
		RigidBody::Static,
		panel_col.clone(),
		NotShadowCaster,
	));

	
	cmds.spawn((
		Name::new("Walls"),
		TransformBundle::default(),
		VisibilityBundle::default(),
	)).with_children(|cmds| {
		cmds.spawn((
			PbrBundle {
				mesh: panel_mesh.clone(),
				material: dark_gray.clone(),
				transform: Transform {
					translation: Vec3::new(0.0, 8.5, 6.5),
					rotation: Quat::from_rotation_x(FRAC_PI_2),
					..default()
				},
				..default()
			},
			RigidBody::Static,
			panel_col.clone(),
			NotShadowCaster,
		));
		cmds.spawn((
			PbrBundle {
				mesh: panel_mesh.clone(),
				material: dark_gray.clone(),
				transform: Transform {
					translation: Vec3::new(-7.5, 0.0, 6.5),
					rotation: Quat::from_rotation_y(FRAC_PI_2),
					..default()
				},
				..default()
			},
			RigidBody::Static,
			panel_col.clone(),
			NotShadowCaster,
		));
		cmds.spawn((
			PbrBundle {
				mesh: panel_mesh.clone(),
				material: dark_gray.clone(),
				transform: Transform {
					translation: Vec3::new(7.5, 0.0, 6.5),
					rotation: Quat::from_rotation_y(FRAC_PI_2),
					..default()
				},
				..default()
			},
			RigidBody::Static,
			panel_col.clone(),
			NotShadowCaster,
		));
		cmds.spawn((
			TransformBundle {
				local: Transform {
					translation: Vec3::new(0.0, -8.5, 6.5),
					rotation: Quat::from_rotation_x(FRAC_PI_2),
					..default()
				},
				..default()
			},
			RigidBody::Static,
			panel_col.clone(),
			NotShadowCaster,
		));
	});
}