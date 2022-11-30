use std::{env::args, collections::HashMap};

use raylib::ffi::Vector3;


mod interactive;

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub struct VoxelPoint(i32, i32, i32);

impl std::ops::Add for VoxelPoint {
    type Output = VoxelPoint;

    fn add(self, rhs: Self) -> Self::Output {
        VoxelPoint(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Into<Vector3> for VoxelPoint {
    fn into(self) -> Vector3 {
        let VoxelPoint(x, y, z) = self;
        Vector3 {
            x: x as f32,
            y: y as f32,
            z: z as f32
        }
        
    }
}

pub struct VoxelMap<T> {
    pub map: HashMap<VoxelPoint, T>,
}

impl<T> VoxelMap<T> {
    fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    fn put(&mut self, pos: VoxelPoint, color: T) {
        self.map.insert(pos, color);
    }

    fn check_pos(&self, pos: &VoxelPoint) -> bool {
        self.map.contains_key(pos)
    }

}

fn main() {
    let path = args().nth(1).expect("no path specified");
    let size = args()
        .nth(2)
        .expect("no size specified")
        .parse::<f32>()
        .expect("invalid size");
    let image = image::open(&path)
        .unwrap()
        // .resize(200, 200, image::imageops::FilterType::Nearest)
        .to_rgb8();

    interactive::run(&image, size);
}
