use bevy::{
    asset::Assets,
    input::mouse::MouseMotion,
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};

use crate::{
    chunk_mesh::ChunkMesh,
    chunk_util::{ChunkUtil, Vector2, Vector3},
};

const CHUNK_SIZE: usize = 32;
const SEED: u32 = 2;
const TOUCH_SENSITIVITY: f32 = 0.1;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

#[derive(Resource)]
struct ChunkData {
    position: Vector2,
    chunks: Vec<Vec<Entity>>,
    chunk_util: ChunkUtil,
    chunk_mesh: ChunkMesh,
}

#[derive(Debug, Component)]
pub struct PlayerCamera;

#[derive(Debug, Clone)]
pub struct Chunk {}

impl Plugin for Chunk {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkData {
            position: Vector2 { x: 0, y: 0 },
            chunks: vec![vec![Entity::PLACEHOLDER; 3]; 3],
            chunk_util: ChunkUtil::new(CHUNK_SIZE, SEED),
            chunk_mesh: ChunkMesh::new(CHUNK_SIZE),
        });

        app.add_systems(Startup, Self::voxel_world);
        // app.add_systems(Update, Self::move_player);
        app.add_systems(Update, Self::move_screen);
    }
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {}
    }

    fn move_screen(
        mut chunk_data: ResMut<ChunkData>,
        buttons: Res<ButtonInput<MouseButton>>,
        mut mouse_motion: EventReader<MouseMotion>,
        mut camera: Query<&mut Transform, With<PlayerCamera>>,
    ) {
        let Ok(mut camera) = camera.get_single_mut() else {
            return;
        };

        if buttons.pressed(MouseButton::Left) {
            for ev in mouse_motion.read() {
                let pos3 = Vec3 {
                    x: (camera.translation.x as f32) - (ev.delta.y * TOUCH_SENSITIVITY),
                    y: 150.0,
                    z: (camera.translation.z as f32) + (ev.delta.x * TOUCH_SENSITIVITY),
                };

                camera.translation = pos3.clone();

                let distance = ((camera.translation.x - chunk_data.position.x as f32).powi(2)
                    + (camera.translation.z - chunk_data.position.y as f32).powi(2))
                .sqrt();

                dbg!(distance);

                // when camera moves from certain threshold then add more chunks and update positions

                // chunk_data.position = Vector2 {
                //     x: pos3.x as isize,
                //     y: pos3.z as isize,
                // };
            }
        }
    }

    // fn move_player(
    //     mut commands: Commands,
    //     mut meshes: ResMut<Assets<Mesh>>,
    //     mut materials: ResMut<Assets<StandardMaterial>>,
    //     keyboard_input: Res<ButtonInput<KeyCode>>,
    //     mut chunk_data: ResMut<ChunkData>,
    // ) {
    //     if keyboard_input.just_pressed(KeyCode::ArrowUp) {
    //         // remove bottom layer
    //         for i in 0..3 {
    //             let offset = Vector2 { x: 2, y: i };
    //             Self::remove_chunk(&mut commands, offset, &mut chunk_data);
    //             chunk_data.chunks[2][i as usize] = Entity::PLACEHOLDER;
    //         }

    //         // move all ref to bottom
    //         for i in 1..3 {
    //             for j in 0..3 {
    //                 chunk_data.chunks[3 - i][j] = chunk_data.chunks[2 - i][j].clone();
    //             }
    //         }

    //         // add chunks to top
    //         for i in 0..3 {
    //             let offset = Vector2 {
    //                 x: chunk_data.position.x - (CHUNK_SIZE * 2) as isize,
    //                 y: chunk_data.position.y + (i - 1) * (CHUNK_SIZE as isize),
    //             };
    //             let chunk = Self::add_chunk(
    //                 &mut commands,
    //                 &mut meshes,
    //                 &mut materials,
    //                 offset,
    //                 &mut chunk_data,
    //             );
    //             chunk_data.chunks[0][i as usize] = chunk;
    //         }

    //         chunk_data.position.x -= CHUNK_SIZE as isize;
    //     }
    // }

    fn voxel_world(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut chunk_data: ResMut<ChunkData>,
        mut asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        for i in 0..3 {
            for j in 0..3 {
                let offset: Vector2 = Vector2 {
                    x: chunk_data.position.x + (i - 1) * (CHUNK_SIZE as isize),
                    y: chunk_data.position.y + (j - 1) * (CHUNK_SIZE as isize),
                };

                let chunk = Self::add_chunk(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    offset,
                    &mut chunk_data,
                    &mut asset_server,
                    &mut texture_atlas_layouts,
                );
                chunk_data.chunks[i as usize][j as usize] = chunk;
            }
        }
    }

    fn add_chunk(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        offset: Vector2,
        chunk_data: &mut ResMut<ChunkData>,
        asset_server: &mut Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Entity {
        let position = Vector3 {
            x: offset.x,
            y: -(CHUNK_SIZE as isize) / 2,
            z: offset.y,
        };

        let world = chunk_data.chunk_util.generate_voxel_world(&position);

        let mesh_data = chunk_data.chunk_mesh.generate_chunk(&world);

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_data.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_data.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_data.uvs);
        mesh.insert_indices(Indices::U32(mesh_data.indices));

        let texture_handle = asset_server.load("textures/atlas.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 16, 16, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        // voxels
        commands
            .spawn((
                MaterialMeshBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(StandardMaterial {
                        // base_color: Color::Srgba(BLACK),
                        base_color_texture: Some(texture_handle.clone()),
                        ..default()
                    }),
                    transform: Transform::from_xyz(
                        position.x as f32 - (CHUNK_SIZE as f32) / 2.0,
                        -(CHUNK_SIZE as f32) / 2.0,
                        position.z as f32 - (CHUNK_SIZE as f32) / 2.0,
                    )
                    .with_scale(Vec3::new(1.0, 1.0, 1.0)),
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                },
            ))
            .id()
    }

    fn remove_chunk(commands: &mut Commands, offset: Vector2, chunk_data: &mut ResMut<ChunkData>) {
        let chunk = chunk_data.chunks[offset.x as usize][offset.y as usize];
        commands.entity(chunk).despawn();
    }
}
