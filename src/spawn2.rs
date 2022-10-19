//tiger
use crate::{Player, PlayerChild, Run};
use bevy::ecs::system::CommandQueue;
use bevy::gltf::GltfMesh;
use bevy::pbr::SkinnedMeshJoints;
use bevy::prelude::*;
use bevy::prelude::{Entity, Query};
use bevy::render::mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes};
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::RenderAsset;
use bevy_rapier3d::prelude::*;
use gltf::Gltf;

pub fn annoyed(
    mut pr: Query<Entity, With<Player>>,
    mut plyr: Query<(&GlobalTransform, &Transform), With<Player>>,
    mut plyq: Query<(&GlobalTransform)>,
    skinned_mesh_inverse_bindposes_assets: Res<Assets<SkinnedMeshInverseBindposes>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ass: ResMut<Assets<GltfMesh>>,
    mut ass_world: ResMut<Assets<Scene>>,
    mut ss: Res<Assets<bevy::gltf::Gltf>>,
    mut run: ResMut<Run>,
) {
    if !run.0 {
        let g = Gltf::open("assets/models/tiger.glb").unwrap();
        let count = g.meshes().len();
        match ass_world.get_mut(&asset_server.load("models/tiger.glb#Scene0")) {
            //.load("models/animated/Fox.glb#Scene0")) {
            Some(world) => {
                let mut i = 0;
                let player = pr.single();
                while i < count {
                    let mut string = "models/tiger.glb#Mesh".to_string() + &*i.to_string();
                    match ass.get_mut(&asset_server.load(string.as_str())) {
                        Some(res) => {
                            let mut query_one = world.world.query::<(Entity, &Handle<Mesh>)>();
                            for (kms, mesh) in query_one.iter_mut(&mut world.world) {
                                for prim in &res.primitives {
                                    if mesh == &prim.mesh {
                                        //commands.entity(kms).remove::<Parent>();
                                        /*let e = commands
                                        .spawn()
                                        .insert(
                                            Collider::from_bevy_mesh(
                                                &meshes.get(&prim.mesh).unwrap(),
                                                &ComputedColliderShape::ConvexDecomposition(
                                                    VHACDParameters::default(),
                                                ),
                                            )
                                            .unwrap(),
                                        )
                                        .id();*/
                                        println!(
                                            "{:?}",
                                            &meshes.get(&prim.mesh).unwrap().compute_aabb()
                                        );
                                        let parent = commands
                                            .entity(kms)
                                            .insert(
                                                Collider::from_bevy_mesh(
                                                    &meshes.get(&prim.mesh).unwrap(),
                                                    &ComputedColliderShape::ConvexDecomposition(
                                                        VHACDParameters::default(),
                                                    ),
                                                )
                                                .unwrap(),
                                            )
                                            .insert(
                                                Transform::from_xyz(0.0, 0.0, 0.0)
                                                    .with_rotation(Quat::from_xyzw(
                                                        0.0, 0.0, 0.0, 0.0,
                                                    ))
                                                    .with_scale(Vec3::new(0.01, 0.01, 0.01)),
                                            )
                                            .id();
                                        commands.entity(player).push_children(&[parent]);
                                    }
                                }
                                run.0 = true;
                            }

                            /*
                                let player = pr.single();
                                let plyrs_gt = plyr.single();
                                let mut query_one = res.world.query::<(Entity, &Handle<Mesh>)>();
                                let mut sauce = res.world.query::<(Entity, &Transform)>();
                                // let mut query_two = res.world.query::<(&Handle<Mesh>)>();
                                let mut query_two = res.world.query::<(&Handle<Mesh>, &SkinnedMesh)>();
                                let mut query_three = res.world.query::<(&GlobalTransform)>();
                                let mut query_said = res.world.query::<(&Aabb)>();
                                for d in query_said.iter(&res.world) {
                                    println!("{:?}", d);
                                }
                                /*for c in res.world.components().iter() {
                                    println!("{:?}", c);
                                }
                                for kms in query_one.iter(&res.world) {
                                    println!("{:?}", kms);
                                }*/

                                /////////// bevy gods bless
                                for (mesh_h, skinned_mesh) in query_two.iter(&mut res.world) {
                                    if let Some(mesh) = meshes.get(mesh_h) {
                                        // Get required vertex attributes
                                        let mesh_positions =
                                            if let Some(VertexAttributeValues::Float32x3(positions)) =
                                                mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                                            {
                                                positions
                                            } else {
                                                continue;
                                            };
                                        let mesh_indices = if let Some(VertexAttributeValues::Uint16x4(indices)) =
                                            mesh.attribute(Mesh::ATTRIBUTE_JOINT_INDEX)
                                        {
                                            indices
                                        } else {
                                            continue;
                                        };
                                        let mesh_weights = if let Some(VertexAttributeValues::Float32x4(weights)) =
                                            mesh.attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT)
                                        {
                                            weights
                                        } else {
                                            continue;
                                        };

                                        // get skinned mesh joint models
                                        let mut joints = Vec::new();
                                        if let Some(_) = SkinnedMeshJoints::build(
                                            skinned_mesh,
                                            &skinned_mesh_inverse_bindposes_assets,
                                            &plyq,
                                            &mut joints,
                                        ) {
                                            // Use skin model to get world space vertex positions
                                            let ws_positions: Vec<Vec3> = mesh_positions
                                                .iter()
                                                .zip(mesh_indices)
                                                .zip(mesh_weights)
                                                .map(|((pos, indices), weights)| {
                                                    let model = skin_model(&joints, indices, Vec4::from(*weights));
                                                    model.transform_point3(Vec3::from(*pos))
                                                })
                                                .collect();
                                            //println!("{:?}", ws_positions);
                                            //compute world space aabb
                                            let ws_aabb = compute_aabb(&ws_positions).unwrap();
                                            println!("{:?}", ws_aabb);
                                        }
                                    }
                                }

                                fn skin_model(
                                    joint_matrices: &Vec<Mat4>,
                                    indexes: &[u16; 4],
                                    weights: Vec4,
                                ) -> Mat4 {
                                    weights.x * joint_matrices[indexes[0] as usize]
                                        + weights.y * joint_matrices[indexes[1] as usize]
                                        + weights.z * joint_matrices[indexes[2] as usize]
                                        + weights.w * joint_matrices[indexes[3] as usize]
                                }

                                const VEC3_MIN: Vec3 = Vec3::splat(std::f32::MIN);
                                const VEC3_MAX: Vec3 = Vec3::splat(std::f32::MAX);

                                /// Compute the Axis-Aligned Bounding Box of the mesh vertices in model space
                                /// from https://github.com/bevyengine/bevy/blob/main/crates/bevy_render/src/mesh/mesh/mod.rs#L375
                                pub fn compute_aabb(values: &[Vec3]) -> Option<Aabb> {
                                    let mut minimum = VEC3_MAX;
                                    let mut maximum = VEC3_MIN;
                                    for p in values {
                                        minimum = minimum.min(*p);
                                        maximum = maximum.max(*p);
                                    }
                                    if minimum.x != std::f32::MAX
                                        && minimum.y != std::f32::MAX
                                        && minimum.z != std::f32::MAX
                                        && maximum.x != std::f32::MIN
                                        && maximum.y != std::f32::MIN
                                        && maximum.z != std::f32::MIN
                                    {
                                        return Some(Aabb::from_min_max(minimum, maximum));
                                    }

                                    None
                                }

                                ///////////

                                let mut queue = CommandQueue::default();
                                //let mut wurld = World::from_world(&mut res.world);
                                //res.world.clear_entities();
                                //let mut commands = Commands::new_from_entities(&mut queue, &res.world.entities());
                                for (e, _) in sauce.iter(&res.world) {
                                    print!("{:?}", e);
                                }
                                let mut m;
                                let mut d = Handle::default();
                                for (kms, mesh) in query_one.iter_mut(&mut res.world) {
                                    commanda
                                        .entity(kms)
                                        .insert(
                                            Collider::from_bevy_mesh(
                                                &meshes.get(mesh).unwrap(),
                                                &ComputedColliderShape::ConvexDecomposition(
                                                    VHACDParameters::default(),
                                                ),
                                            )
                                            .unwrap(),
                                        )
                                        .insert(PlayerChild)
                                        .insert(
                                            Transform::from_xyz(0.0, 0.0, 0.0)
                                                .with_rotation(Quat::from_xyzw(0.0, 0.0, 0.0, 0.0)),
                                        )
                                        .insert(GlobalTransform::from(*plyrs_gt.0));
                                    m = kms;
                                    d = mesh.clone();
                                }
                                let plz = d;
                                // for (kms, mesh) in query_one.iter_mut(&mut res.world) {

                                let id = res
                                    .world
                                    .spawn()
                                    .insert(RigidBody::Dynamic)
                                    .insert(
                                        Collider::from_bevy_mesh(
                                            &meshes.get(&plz).unwrap(),
                                            &ComputedColliderShape::ConvexDecomposition(VHACDParameters::default()),
                                        )
                                        .unwrap(),
                                    )
                                    //.insert(PlayerChild)
                                    .insert(
                                        Transform::from_xyz(0.0, 0.0, 0.0)
                                            .with_rotation(Quat::from_xyzw(0.0, 0.0, 0.0, 0.0))
                                            .with_scale(Vec3::new(100.0, 100.0, 100.0)),
                                    )
                                    .insert(GlobalTransform::from(*plyrs_gt.0))
                                    .id();
                                let mut query_saidrt = res.world.query::<(&Collider)>();
                                for d in query_saidrt.iter_mut(&mut res.world) {
                                    let iz = commanda.spawn().insert(d.clone()).id();
                                    commanda.entity(player).push_children(&[iz]);
                                    //println!("{:?}", d.raw);
                                }

                                //commanda.entity(id).despawn();
                                // println!("{:?}", mesh);
                                run.0 = true;
                            }

                            /*
                            let child = commands
                                .entity(kms)
                                .insert(
                                    Collider::from_bevy_mesh(
                                        &meshes.get(mesh).unwrap(),
                                        &ComputedColliderShape::ConvexDecomposition(
                                            VHACDParameters::default(),
                                        ),
                                    )
                                    .unwrap(),
                                )
                                .insert(PlayerChild)
                                .insert(
                                    Transform::from_xyz(0.0, 0.0, 0.0)
                                        .with_rotation(Quat::from_xyzw(0.0, 0.0, 0.0, 0.0)),
                                )
                                .insert(GlobalTransform::from(*plyrs_gt.0))
                                .despawn();*/
                            //.id();
                            // commanda.entity(player).push_children(&[child]);
                            // }*/
                        }
                        None => (run.0 = false),
                    }

                    i = i + 1;
                }
            }
            None => (run.0 = false),
        }
    }
}
