use noise::{NoiseFn, Perlin, Simplex};

const LOW_NOISE_SCALE: f64 = 0.01;
const HIGH_NOISE_SCALE: f64 = 0.05;
const NOISE_SCALE_FACTOR: f64 = 0.8;

#[derive(Debug, Clone)]
pub struct Voxel {
    pub value: VoxelType,
}

#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone)]
pub struct ChunkUtil {
    chunk_size: usize,
    seed: u32,
}

#[derive(Debug, Clone)]
pub enum VoxelType {
    AIR,
    GRASS,
    DIRT,
    WOOD,
    SAND,
    ROCK,
    COAL,
    ICE,
    GOLD,
    LAVA,
}

impl VoxelType {
    pub fn to_string(&self) -> &str {
        match self {
            VoxelType::AIR => "AIR",
            VoxelType::GRASS => "GRASS",
            VoxelType::DIRT => "DIRT",
            VoxelType::WOOD => "WOOD",
            VoxelType::SAND => "SAND",
            VoxelType::ROCK => "ROCK",
            VoxelType::COAL => "COAL",
            VoxelType::ICE => "ICE",
            VoxelType::GOLD => "GOLD",
            VoxelType::LAVA => "LAVA",
        }
    }

    pub fn get_uvs_modifier(&self) -> [f32; 2] {
        match self {
            VoxelType::AIR => todo!(),
            VoxelType::GRASS => [0.0625, 0.0625 * 9.0],
            VoxelType::DIRT => [0.0625 * 2.0, 0.0],
            VoxelType::WOOD => todo!(),
            VoxelType::SAND => [0.0625 * 2.0, 0.0625],
            VoxelType::ROCK => [0.0, 0.0625],
            VoxelType::COAL => todo!(),
            VoxelType::ICE => [0.0625 * 2.0, 0.0625 * 4.0],
            VoxelType::GOLD => todo!(),
            VoxelType::LAVA => todo!(),
        }
    }
}

impl ChunkUtil {
    pub fn new(chunk_size: usize, seed: u32) -> ChunkUtil {
        ChunkUtil { chunk_size, seed }
    }

    pub fn generate_voxel_world(&self, offset: &Vector3) -> Vec<Vec<Vec<Voxel>>> {
        let mut world = vec![
            vec![
                vec![
                    Voxel {
                        value: VoxelType::AIR
                    };
                    self.chunk_size
                ];
                self.chunk_size
            ];
            self.chunk_size
        ];

        let perlin = Perlin::new(self.seed);
        let simplex = Simplex::new(self.seed);

        for x in 0..self.chunk_size {
            for z in 0..self.chunk_size {
                let xo = (x as f64) + (offset.x as f64);
                let zo = (z as f64) + (offset.z as f64);

                let low_noise_value =
                    simplex.get([xo * LOW_NOISE_SCALE, zo * LOW_NOISE_SCALE]) * 3.0;
                let high_noise_value =
                    perlin.get([xo * HIGH_NOISE_SCALE, zo * HIGH_NOISE_SCALE]) * 5.0;

                let noise_value = low_noise_value + high_noise_value * NOISE_SCALE_FACTOR;

                let value = if noise_value > 5.0 {
                    VoxelType::ICE
                } else if low_noise_value > 1.2 {
                    VoxelType::ROCK
                } else if noise_value > 0.0 {
                    VoxelType::GRASS
                } else {
                    VoxelType::SAND
                };

                world[x][(noise_value + 16.0) as usize][z] = Voxel { value };
            }
        }

        world
    }
}
