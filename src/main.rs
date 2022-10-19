#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

//! A simple 3D scene with light shining over a cube sitting on a plane.
pub mod ability;
pub mod spawn;
pub mod plugme;
pub mod spawn2;
use bevy::{
    ecs::component,
    gltf::{GltfMesh, GltfNode, GltfPrimitive},
    math::Vec3A,
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        mesh,
        primitives::Aabb,
        render_asset::RenderAsset,
        settings::{WgpuFeatures, WgpuSettings},
        view::{ExtractedView, VisibleEntities},
    },
    utils::tracing::instrument::WithSubscriber,
};
use bevy_rapier3d::{
    parry::shape::Cuboid,
    prelude::{
        Collider, ComputedColliderShape, NoUserData, RapierConfiguration, RapierPhysicsPlugin,
        RigidBody, Sensor, TimestepMode, VHACDParameters, Velocity,
    },
    rapier::prelude::ColliderBuilder,
    render::RapierDebugRenderPlugin,
};
use gltf::Gltf;
use std::ops::Add;
/*
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .add_system(lets_get_ass)
        .add_startup_system(set)
        .run();
}*/
pub struct Run(bool);
fn main() {
    let g = Gltf::open("assets/models/komatsu_forwarder_yellow.glb").unwrap();
    println!("{:?}", g.meshes().len());
    let rapier = RapierConfiguration {
        timestep_mode: TimestepMode::Fixed {
            dt: 0.01,
            substeps: 1,
        },
        ..default()
    };
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .insert_resource(Run(false))
        .insert_resource(rapier)
        .add_plugin(WireframePlugin)
        .add_startup_system(setup)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_system(move_player)
        .add_system(show_me_kids)
        .add_plugin(RapierDebugRenderPlugin::default())
        //.add_system(lets_get_ass)
        //.add_system(camera_with_parent)
        .add_system(decrease_cds)
        .add_system(spawn2::annoyed)
        .run();
}

fn lets_get_ass(
    mut collider: Query<Entity, With<PlayerChild>>,
    mut pr: Query<Entity, With<Player>>,
    mut ter: Query<Entity, With<Terrain>>,
    mut run: ResMut<Run>,
    mut commands: Commands,
    mut ass: ResMut<Assets<Scene>>,
    asset_server: Res<AssetServer>,
    as_mesh: ResMut<Assets<Mesh>>,
) {
    if !run.0 {
        match ass.get_mut(&asset_server.load("models/animated/Fox.glb#Scene0")) {
            //.load("models/animated/Fox.glb#Scene0")) {
            Some(res) => {
                let mut query_one = res.world.query::<(&Aabb)>();
                let mut query_two = res.world.query::<(&Handle<Mesh>)>();

                for (c) in query_two.iter(&res.world) {
                    println!(
                        "hey {:?}",
                        as_mesh.get(c).unwrap().extract_asset().compute_aabb()
                    );
                }

                for d in query_one.iter(&res.world) {
                    let player = pr.single();
                    println!("{:?}", d);
                    let min = Vec3::from(d.min());
                    let max = Vec3::from(d.max());
                    println!("{:?}, {:?}", min, max);
                    println!("{:?}, {:?}", d.min(), d.max());
                    let a = d.half_extents.to_array();
                    let child = commands
                        .spawn()
                        .insert(Collider::cuboid(a[0] / 100.0, a[1] / 100.0, a[2] / 100.0))
                        .insert(PlayerChild)
                        .insert(Transform::from_xyz(0.0, (a[1]) / 100.0, 0.0))
                        .id();
                    commands.entity(player).push_children(&[child]);
                    let scale = Collider::cuboid(a[0], a[1], a[2]).scale();
                    println!("{:?}", scale);
                    println!("{:?}, {:?}, {:?}", a[0], a[1], a[2]);

                    //  println!("{:?}", c);
                }
                run.0 = true;
            }
            None => (run.0 = false),
        }
        /*
        match ass.get_mut(&asset_server.load("tabletop_terrain.glb#Scene0")) {
            //.load("models/animated/Fox.glb#Scene0")) {
            Some(res) => {
                let mut query_one = res.world.query::<(&Aabb)>();
                let mut query_two = res.world.query::<(&Handle<Mesh>)>();

                for (c) in query_two.iter(&res.world) {
                    println!(
                        "hey {:?}",
                        as_mesh.get(c).unwrap().extract_asset().compute_aabb()
                    );
                }

                for d in query_one.iter(&res.world) {
                    let t = ter.single();
                    println!("{:?}", d);
                    let min = Vec3::from(d.min());
                    let max = Vec3::from(d.max());
                    println!("{:?}, {:?}", min, max);
                    println!("{:?}, {:?}", d.min(), d.max());
                    let a = d.half_extents.to_array();
                    let child = commands
                        .spawn()
                        .insert(Collider::cuboid(a[0] / 10.0, a[1] / 10.0, a[2] / 10.0))
                        .insert(PlayerChild)
                        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
                        .id();
                    commands.entity(t).push_children(&[child]);
                    let scale = Collider::cuboid(a[0], a[1], a[2]).scale();
                    println!("{:?}", scale);
                    println!("{:?}, {:?}, {:?}", a[0], a[1], a[2]);

                    //  println!("{:?}", c);
                }
                run.0 = true;
            }
            None => (run.0 = false),
        }*/
    }
}

fn setup(
    mut wireframe_config: ResMut<WireframeConfig>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(Animations(vec![
        asset_server.load("models/animated/Fox.glb#Animation2"),
        asset_server.load("models/animated/Fox.glb#Animation1"),
        asset_server.load("models/animated/Fox.glb#Animation0"),
        asset_server.load("models/animated/dance.glb#Animation0"),
    ]));
    let handle: Handle<Scene> = asset_server.load("models/animated/Fox.glb#Scene0");
    /*commands*/
    /*
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::default())),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(handle);*/
    //.insert(bounding::debug::DebugBounds);
    // plane
    //wireframe_config.global = true;

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(0.0, -2.0, 0.0), //.with_scale(Vec3::new(0.01, 0.01, 0.01)),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5.0, 0.00000000000000001, 5.0));
    // cube
    let parent = commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("models/komatsu_forwarder_yellow.glb#Scene0"),
            //transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(10.0, 10.0, 10.0)),
            //global_transform: GlobalTransform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Player(CoolDowns::new()))
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(100.0, 100.0, 100.0)),
        ))
        //.insert(RigidBody::Dynamic)
        //.insert(Collider::ball(0.5))
        //.insert(Velocity::default())
        .id();
    /*let c = commands
    .spawn()
    .insert(RigidBody::Fixed)
    .insert_bundle(TransformBundle::from(
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(100.0, 100.0, 100.0)),
    ))
    .id();*/
    //commands.entity(c).push_children(&[parent]);
    /*
        commands
            .spawn_bundle(SceneBundle {
                scene: asset_server.load("models/komatsu_fo
    rwarder_yellow.glb#Scene0"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 1.0, 1.0)),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            //.insert(Collider::ball(0.5))
            .insert(Velocity::default())
            .insert(Terrain);*/
    /*
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("tabletop_terrain.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.1, 0.1, 0.1)),
        ..default()
    });*/
    /* .insert_bundle(TransformBundle::from(
        Transform::from_xyz(0.0, 2.0, 0.0).with_scale(Vec3::new(0.01, 0.01, 0.01)),
    ));*/
    /*
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::default())),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, -4.0).with_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..default()
        })
        .insert(Health(100.0));*/

    // light

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 10.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, -25.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut v: Query<&mut Velocity, With<Player>>,
    mut transforms: Query<(&mut Transform, &mut GlobalTransform, &mut Player), Without<Ability>>,
    mut transform_a: Query<(&mut Transform, &Ability), Without<Player>>,
    mut player: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
) {
    for (transform_p, _, mut cd) in &mut transforms {
        //if transform_a.is_empty() {
        let cds = cd.0 .0[0];
        if keyboard_input.pressed(KeyCode::Space) && cds == 0 {
            commands
                .spawn_bundle(SceneBundle {
                    scene: asset_server.load("models/animated/fireball.glb#Scene0"),
                    transform: transform_p.with_scale(Vec3::new(0.5, 0.5, 0.5)),
                    ..default()
                })
                .insert(Ability)
                //.insert(ColliderBuilder::ball(0.5))
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(0.5))
                .insert(Sensor)
                .insert(Velocity::linear(Vec3::new(0.0, 10.0, 5.0)));
            cd.0 .0[0] = 1000;
        }
        // } else {
        /*for (mut ta, _) in &mut transform_a {
            ta.translation.z += 1.0;
        }*/
        //}
    }

    for (mut transform_p, mut z, _) in &mut transforms {
        if keyboard_input.pressed(KeyCode::Z) {
            transform_p.translation.y += 0.1;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform_p.translation.x -= 0.1;
            transform_p.rotation = Quat::from_rotation_y(67.5);
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform_p.translation.x += 0.1;
            transform_p.rotation = Quat::from_rotation_y(-67.5);
        }
        if keyboard_input.pressed(KeyCode::W) {
            transform_p.translation.z += 0.1;
            transform_p.rotation = Quat::from_rotation_y(0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform_p.translation.z -= 0.1;
            transform_p.rotation = Quat::from_rotation_y(135.0);
        }
        if keyboard_input.just_pressed(KeyCode::Space) {
            if let Ok(mut player) = player.get_single_mut() {
                player.play(animations.0[2].clone_weak()).repeat();
            }
        }
        if keyboard_input.just_pressed(KeyCode::W)
            || keyboard_input.just_pressed(KeyCode::A)
            || keyboard_input.just_pressed(KeyCode::S)
            || keyboard_input.just_pressed(KeyCode::D)
        {
            if let Ok(mut player) = player.get_single_mut() {
                player.play(animations.0[0].clone_weak()).repeat();
            }
        } else if !(keyboard_input.pressed(KeyCode::W)
            || keyboard_input.pressed(KeyCode::A)
            || keyboard_input.pressed(KeyCode::S)
            || keyboard_input.pressed(KeyCode::D)
            || keyboard_input.pressed(KeyCode::Space))
        {
            if let Ok(mut player) = player.get_single_mut() {
                player.pause();
            }
        }
    }
}
fn decrease_cds(mut cds: Query<(&mut Player)>) {
    for mut cd in cds.iter_mut() {
        cd.decrease_cds();
    }
}
fn trans(mut transforms: Query<(&mut Transform, &GlobalTransform), With<Player>>) {
    for (t, g) in transforms.iter() {
        println!("{:?}", t);
        println!("{:?}", g);
    }
}
#[derive(Component)]
pub struct Player(CoolDowns);
impl Player {
    pub fn decrease_cds(self: &mut Self) {
        let mut i = 0;
        for cd in self.0 .0 {
            if cd > 0 {
                self.0 .0[i] = cd - 1;
            }
            i += 1;
        }
    }
}

#[derive(Component)]
struct CoolDowns([u32; 4]);
impl CoolDowns {
    pub fn new() -> CoolDowns {
        CoolDowns([0, 0, 0, 0])
    }
}
#[derive(Component)]
pub struct Ability;

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Terrain;

#[derive(Component)]
pub struct PlayerChild;
struct Animations(Vec<Handle<AnimationClip>>);

/*
fn camera_with_parent(
    q_child: Query<(&Parent, &GlobalTransform), With<PlayerChild>>,
    q_parent: Query<&GlobalTransform>,
) {
    for (parent, child_transform) in q_child.iter() {
        // `parent` contains the Entity ID we can use
        // to query components from the parent:
        let parent_global_transform = q_parent.get(parent.get());
        println!("{:?}", parent_global_transform);
        println!("{:?}", child_transform);
        // do something with the components
    }
}*/

fn show_me_kids(
    q: Query<(&Children)>,
    qp: Query<(&Parent)>,
    mut commands: Commands,
    mut views: Query<(&ExtractedView, &VisibleEntities)>,
) {
    for (e, v) in views.iter() {
        println!("{:?}", e.projection);
    }
    for (c) in q.iter() {
        // println!("{:?}", c);
        // println!("{:?}", child_transform);
        // do something with the components
    }
    for (parent) in qp.iter() {
        // commands.entity(parent.get()).remove::<Parent>();
        //println!("{:?}", parent);
        // println!("{:?}", child_transform);
        // do something with the components
    }
}
