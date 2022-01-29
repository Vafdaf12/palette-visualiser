use image::{Rgb, RgbImage};
use kolor::ColorConversion;
use raylib::prelude::*;

const SCALE: f32 = 100.0;

fn round_to_nearest(value: f32, step: f32) -> f32 {
    if step == 0.0 {
        value
    } else {
        (value / step).round() * step
    }
}

pub fn compute_palette(image: &RgbImage, voxel_size: f32) -> Vec<(Color, Vector3)> {
    let lab = ColorConversion::new(kolor::spaces::LINEAR_SRGB, kolor::spaces::OKLAB);

    let mut points: Vec<(Color, Vector3)> = Vec::new();

    for Rgb([r, g, b]) in image.pixels() {
        let color = Color::new(*r, *g, *b, 255);

        let c = lab.convert(kolor::Vec3::new(
            f32::from(*r) / 255.0,
            f32::from(*g) / 255.0,
            f32::from(*b) / 255.0,
        ));
        let mut pos = Vector3::new(c.y, c.x, c.z) * SCALE;

        pos.x = round_to_nearest(pos.x, voxel_size);
        pos.y = round_to_nearest(pos.y, voxel_size);
        pos.z = round_to_nearest(pos.z, voxel_size);

        let exists = points.iter().map(|(_, p)| *p).any(|p| p == pos);

        if !exists {
            points.push((color, pos));
        }
    }

    points
}

pub fn run(image: &RgbImage, voxel_size: f32) {
    let palette = compute_palette(&image, voxel_size);
    println!(
        "Image: {}x{} ({} pixels)",
        image.width(),
        image.height(),
        image.width() * image.height()
    );
    println!("Palette Size: {}", palette.len());

    let center: Vector3 = palette
        .clone()
        .into_iter()
        .map(|(_, p)| p)
        .reduce(|a, b| a + b)
        .unwrap()
        / palette.len() as f32;
    raylib::set_trace_log(TraceLogLevel::LOG_WARNING);
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Hello World")
        .resizable()
        .build();

    let mut camera = Camera3D::perspective(
        center + Vector3::right() * SCALE,
        center,
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );
    rl.set_camera_mode(camera, CameraMode::CAMERA_ORBITAL);
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(40, 40, 40, 255));

        {
            let mut d2 = d.begin_mode3D(camera);

            for (col, pos) in palette.iter() {
                d2.draw_cube_v(*pos, Vector3::one() * voxel_size, col);
            }
        }
    }
}
