use image::{Rgb, RgbImage};
use kolor::ColorConversion;
use raylib::prelude::*;

use crate::{VoxelMap, VoxelPoint};

const SCALE: f32 = 100.0;

fn round_to_nearest(value: f32, step: f32) -> f32 {
    if step == 0.0 {
        value
    } else {
        (value / step).round() * step
    }
}

pub fn compute_palette(image: &RgbImage, voxel_size: f32) -> VoxelMap<Color> {
    let mut map: VoxelMap<Color> = VoxelMap::new();
    let lab = ColorConversion::new(kolor::spaces::LINEAR_SRGB, kolor::spaces::OKLAB);

    for Rgb([r, g, b]) in image.pixels() {
        let color = Color::new(*r, *g, *b, 255);

        let c = lab.convert(kolor::Vec3::new(
            f32::from(*r) / 255.0,
            f32::from(*g) / 255.0,
            f32::from(*b) / 255.0,
        ));
        let mut pos = Vector3::new(c.y, c.x, c.z) * SCALE;

        pos.x = (pos.x / voxel_size).round();
        pos.y = (pos.y / voxel_size).round();
        pos.z = (pos.z / voxel_size).round();

        let point = VoxelPoint(pos.x as i32, pos.y as i32, pos.z as i32);

        if !map.check_pos(&point) {
            map.put(point, color);
        }
    }

    map
}

pub fn run(image: &RgbImage, voxel_size: f32) {
    let palette = compute_palette(&image, voxel_size);
    println!(
        "Image: {}x{} ({} pixels)",
        image.width(),
        image.height(),
        image.width() * image.height()
    );
    println!("Palette Size: {}", palette.map.keys().len());


    
    raylib::set_trace_log(TraceLogLevel::LOG_WARNING);
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Hello World")
        .resizable()
        .build();

    let mut camera = Camera3D::perspective(
        Vector3::right() * SCALE,
        Vector3::zero(),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );
    rl.set_camera_mode(camera, CameraMode::CAMERA_FREE);
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(40, 40, 40, 255));

        {
            let mut d2 = d.begin_mode3D(camera);

            for (&pos, col) in palette.map.iter() {
                

                d2.draw_cube_v(pos, Vector3::one(), col);
            }

        }
    }
}
