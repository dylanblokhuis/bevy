//! Illustrates different lights of various types and colors, some static, some moving over
//! a simple scene.

use std::f32::consts::PI;

use bevy::{
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::camera::{ExposureSettings, PhysicalCameraParameters},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Parameters(PhysicalCameraParameters {
            aperture_f_stops: 1.0,
            shutter_speed_s: 1.0 / 15.0,
            sensitivity_iso: 400.0,
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_exposure, movement, animate_light_direction))
        .run();
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

#[derive(Component)]
struct Movable;

/// set up a simple 3D scene
fn setup(
    parameters: Res<Parameters>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(10.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // left wall
    let mut transform = Transform::from_xyz(2.5, 2.5, 0.0);
    transform.rotate_z(PI / 2.);
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(5.0, 0.15, 5.0)),
        transform,
        material: materials.add(StandardMaterial {
            base_color: Color::INDIGO,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });
    // back (right) wall
    let mut transform = Transform::from_xyz(0.0, 2.5, -2.5);
    transform.rotate_x(PI / 2.);
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(5.0, 0.15, 5.0)),
        transform,
        material: materials.add(StandardMaterial {
            base_color: Color::INDIGO,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // Bevy logo to demonstrate alpha mask shadows
    let mut transform = Transform::from_xyz(-2.2, 0.5, 1.0);
    transform.rotate_y(PI / 8.);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(2.0, 0.5))),
            transform,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("branding/bevy_logo_light.png")),
                perceptual_roughness: 1.0,
                alpha_mode: AlphaMode::Mask(0.5),
                cull_mode: None,
                ..default()
            }),
            ..default()
        },
        Movable,
    ));

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube { size: 1.0 }),
            material: materials.add(StandardMaterial {
                base_color: Color::PINK,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Movable,
    ));
    // sphere
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::UVSphere {
                radius: 0.5,
                ..default()
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::LIME_GREEN,
                ..default()
            }),
            transform: Transform::from_xyz(1.5, 1.0, 1.5),
            ..default()
        },
        Movable,
    ));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    // red point light
    commands
        .spawn(PointLightBundle {
            // transform: Transform::from_xyz(5.0, 8.0, 2.0),
            transform: Transform::from_xyz(1.0, 2.0, 0.0),
            point_light: PointLight {
                intensity: 4000.0, // lumens - roughly a 300W non-halogen incandescent bulb
                color: Color::RED,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: meshes.add(shape::UVSphere {
                    radius: 0.1,
                    ..default()
                }),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    emissive: Color::rgba_linear(7.13, 0.0, 0.0, 0.0),
                    ..default()
                }),
                ..default()
            });
        });

    // green spot light
    commands
        .spawn(SpotLightBundle {
            transform: Transform::from_xyz(-1.0, 2.0, 0.0)
                .looking_at(Vec3::new(-1.0, 0.0, 0.0), Vec3::Z),
            spot_light: SpotLight {
                intensity: 4000.0, // lumens - roughly a 300W non-halogen incandescent bulb
                color: Color::GREEN,
                shadows_enabled: true,
                inner_angle: 0.6,
                outer_angle: 0.8,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                transform: Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
                mesh: meshes.add(shape::Capsule {
                    depth: 0.125,
                    radius: 0.1,
                    ..default()
                }),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    emissive: Color::rgba_linear(0.0, 7.13, 0.0, 0.0),
                    ..default()
                }),
                ..default()
            });
        });

    // blue point light
    commands
        .spawn(PointLightBundle {
            // transform: Transform::from_xyz(5.0, 8.0, 2.0),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            point_light: PointLight {
                intensity: 4000.0, // lumens - roughly a 300W non-halogen incandescent bulb
                color: Color::BLUE,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: meshes.add(shape::UVSphere {
                    radius: 0.1,
                    ..default()
                }),
                material: materials.add(StandardMaterial {
                    base_color: Color::BLUE,
                    emissive: Color::rgba_linear(0.0, 0.0, 7.13, 0.0),
                    ..default()
                }),
                ..default()
            });
        });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // example instructions
    let style = TextStyle {
        font_size: 20.0,
        ..default()
    };
    commands.spawn(
        TextBundle::from_sections(vec![
            TextSection::new(
                format!("Aperture: f/{:.0}\n", parameters.aperture_f_stops),
                style.clone(),
            ),
            TextSection::new(
                format!(
                    "Shutter speed: 1/{:.0}s\n",
                    1.0 / parameters.shutter_speed_s
                ),
                style.clone(),
            ),
            TextSection::new(
                format!("Sensitivity: ISO {:.0}\n", parameters.sensitivity_iso),
                style.clone(),
            ),
            TextSection::new("\n\n", style.clone()),
            TextSection::new("Controls\n", style.clone()),
            TextSection::new("---------------\n", style.clone()),
            TextSection::new("Arrow keys - Move objects\n", style.clone()),
            TextSection::new("1/2 - Decrease/Increase aperture\n", style.clone()),
            TextSection::new("3/4 - Decrease/Increase shutter speed\n", style.clone()),
            TextSection::new("5/6 - Decrease/Increase sensitivity\n", style.clone()),
            TextSection::new("R - Reset exposure", style),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ExposureSettings::from_physical_camera(**parameters),
    ));
}

fn update_exposure(
    key_input: Res<ButtonInput<KeyCode>>,
    mut parameters: ResMut<Parameters>,
    mut query: Query<&mut ExposureSettings>,
    mut text: Query<&mut Text>,
) {
    // TODO: Clamp values to a reasonable range
    let mut text = text.single_mut();
    if key_input.just_pressed(KeyCode::Digit2) {
        parameters.aperture_f_stops *= 2.0;
    } else if key_input.just_pressed(KeyCode::Digit1) {
        parameters.aperture_f_stops *= 0.5;
    }
    if key_input.just_pressed(KeyCode::Digit4) {
        parameters.shutter_speed_s *= 2.0;
    } else if key_input.just_pressed(KeyCode::Digit3) {
        parameters.shutter_speed_s *= 0.5;
    }
    if key_input.just_pressed(KeyCode::Digit6) {
        parameters.sensitivity_iso += 100.0;
    } else if key_input.just_pressed(KeyCode::Digit5) {
        parameters.sensitivity_iso -= 100.0;
    }
    if key_input.just_pressed(KeyCode::KeyR) {
        *parameters = Parameters::default();
    }

    text.sections[0].value = format!("Aperture: f/{:.0}\n", parameters.aperture_f_stops);
    text.sections[1].value = format!(
        "Shutter speed: 1/{:.0}s\n",
        1.0 / parameters.shutter_speed_s
    );
    text.sections[2].value = format!("Sensitivity: ISO {:.0}\n", parameters.sensitivity_iso);

    *query.single_mut() = ExposureSettings::from_physical_camera(**parameters);
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        transform.translation += time.delta_seconds() * 2.0 * direction;
    }
}
