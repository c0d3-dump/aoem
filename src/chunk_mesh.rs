use bevy::math::Vec3;

use crate::chunk_util::{Vector3, Voxel, VoxelType};

#[derive(Debug, Clone)]
pub struct ChunkMesh {
    chunk_size: usize,
}

#[derive(Debug, Clone)]
pub struct MeshData {
    pub positions: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
}

impl ChunkMesh {
    pub fn new(chunk_size: usize) -> ChunkMesh {
        ChunkMesh { chunk_size }
    }

    fn get_directions(&self) -> [(Vec3, [f32; 3], [[f32; 2]; 4]); 6] {
        [
            (
                Vec3::X,
                [1.0, 0.0, 0.0],
                [[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
            ), // Right
            (
                -Vec3::X,
                [-1.0, 0.0, 0.0],
                [[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
            ), // Left
            (
                Vec3::Y,
                [0.0, 1.0, 0.0],
                [[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
            ), // Up
            (
                -Vec3::Y,
                [0.0, -1.0, 0.0],
                [[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
            ), // Down
            (
                Vec3::Z,
                [0.0, 0.0, 1.0],
                [[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
            ), // Front
            (
                -Vec3::Z,
                [0.0, 0.0, -1.0],
                [[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
            ), // Back
        ]
    }

    pub fn generate_chunk(&self, world: &Vec<Vec<Vec<Voxel>>>) -> MeshData {
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut index_offset = 0;

        for x in 0..self.chunk_size {
            for y in 0..self.chunk_size {
                for z in 0..self.chunk_size {
                    let voxel = &world[x][y][z];

                    if voxel.value.to_string() != VoxelType::AIR.to_string() {
                        // traverse all directions
                        for (direction, normal, face_uvs) in &self.get_directions() {
                            let neighbor = self.get_voxel(
                                world,
                                Vector3 {
                                    x: x as isize + direction.x as isize,
                                    y: y as isize + direction.y as isize,
                                    z: z as isize + direction.z as isize,
                                },
                            );

                            if neighbor
                                .map_or(true, |v| v.value.to_string() == VoxelType::AIR.to_string())
                            {
                                let base_index = index_offset as u32;

                                let face_positions = self.get_face_position(normal, x, y, z);

                                positions.extend_from_slice(&face_positions);
                                indices.extend_from_slice(&[
                                    base_index,
                                    base_index + 1,
                                    base_index + 2,
                                    base_index + 2,
                                    base_index + 1,
                                    base_index + 3,
                                ]);

                                normals.extend_from_slice(&[*normal; 4]);

                                index_offset += 4;

                                let uv_mod = voxel.value.get_uvs_modifier();

                                let voxel_uvs = face_uvs.clone().map(|t| {
                                    [t[0] * 0.0625 + uv_mod[0], t[1] * 0.0625 + uv_mod[1]]
                                });

                                uvs.extend_from_slice(&voxel_uvs);
                            }
                        }
                    }
                }
            }
        }

        MeshData {
            positions,
            indices,
            normals,
            uvs,
        }
    }

    fn get_voxel<'a>(&self, world: &'a Vec<Vec<Vec<Voxel>>>, offset: Vector3) -> Option<&'a Voxel> {
        if offset.x >= 0
            && offset.x < self.chunk_size as isize
            && offset.y >= 0
            && offset.y < self.chunk_size as isize
            && offset.z >= 0
            && offset.z < self.chunk_size as isize
        {
            Some(&world[offset.x as usize][offset.y as usize][offset.z as usize])
        } else {
            None
        }
    }

    fn get_face_position(&self, normal: &[f32; 3], x: usize, y: usize, z: usize) -> [[f32; 3]; 4] {
        match *normal {
            [1.0, 0.0, 0.0] => [
                // Right face (x+)
                [x as f32 + 1.0, y as f32, z as f32],
                [x as f32 + 1.0, y as f32 + 1.0, z as f32],
                [x as f32 + 1.0, y as f32, z as f32 + 1.0],
                [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0],
            ],
            [-1.0, 0.0, 0.0] => [
                // Left face (x-)
                [x as f32, y as f32, z as f32],
                [x as f32, y as f32, z as f32 + 1.0],
                [x as f32, y as f32 + 1.0, z as f32],
                [x as f32, y as f32 + 1.0, z as f32 + 1.0],
            ],
            [0.0, 1.0, 0.0] => [
                // Top face (y+)
                [x as f32, y as f32 + 1.0, z as f32],
                [x as f32, y as f32 + 1.0, z as f32 + 1.0],
                [x as f32 + 1.0, y as f32 + 1.0, z as f32],
                [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0],
            ],
            [0.0, -1.0, 0.0] => [
                // Bottom face (y-)
                [x as f32, y as f32, z as f32],
                [x as f32 + 1.0, y as f32, z as f32],
                [x as f32, y as f32, z as f32 + 1.0],
                [x as f32 + 1.0, y as f32, z as f32 + 1.0],
            ],
            [0.0, 0.0, 1.0] => [
                // Front face (z+)
                [x as f32, y as f32, z as f32 + 1.0],
                [x as f32 + 1.0, y as f32, z as f32 + 1.0],
                [x as f32, y as f32 + 1.0, z as f32 + 1.0],
                [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0],
            ],
            [0.0, 0.0, -1.0] => [
                // Back face (z-)
                [x as f32, y as f32, z as f32],
                [x as f32, y as f32 + 1.0, z as f32],
                [x as f32 + 1.0, y as f32, z as f32],
                [x as f32 + 1.0, y as f32 + 1.0, z as f32],
            ],
            _ => panic!("Invalid normal direction"),
        }
    }
}
