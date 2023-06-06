pub mod vec;
pub mod rng;
pub mod camera;
pub mod shape;
pub mod material;
pub mod ray;

use std::{
    fs::File,
    io::{Write},
};

use vec::{Color, Vec3};
use ray::Ray;
use shape::{Collidable, Collision};
use material::{Material, Reflectable};

const COLOR_MAX: f64 = 255f64;


pub fn write_color(file: &mut File, pixel_color: Color, gamma_scale: f64) -> std::io::Result<()> {
    let color_x = (COLOR_MAX * (pixel_color.x * gamma_scale).sqrt()) as i32;
    let color_y = (COLOR_MAX * (pixel_color.y * gamma_scale).sqrt()) as i32;
    let color_z = (COLOR_MAX * (pixel_color.z * gamma_scale).sqrt()) as i32;
    let r = color_x.clamp(0i32, 255i32);
    let g = color_y.clamp(0i32, 255i32);
    let b = color_z.clamp(0i32, 255i32);
    file.write_fmt(format_args!("{} {} {}\n", r, g, b))?;
    return Ok(());
}

// pub fn write_batch_color(file: &mut File, pixel_colors: Vec<&Color>, gamma_scale: f64)-> std::io::Result<()> {
//     let rgb_count = pixel_colors.len();
//     let color_bytes: Vec<u8> = Vec::new();
//     for pixel_color in pixel_colors.iter() {
//         let color_x = (COLOR_MAX * (pixel_color.x * gamma_scale).sqrt()) as i32;
//         let color_y = (COLOR_MAX * (pixel_color.y * gamma_scale).sqrt()) as i32;
//         let color_z = (COLOR_MAX * (pixel_color.z * gamma_scale).sqrt()) as i32;
//         let r: i32 = color_x.clamp(0i32, 255i32);
//         let g: i32 = color_y.clamp(0i32, 255i32);
//         let b: i32 = color_z.clamp(0i32, 255i32);
//     }
//     file.write_all(buf)
//     file.write_fmt(format_args!("{} {} {}\n", r, g, b))?;
//     return Ok(());
// }


fn get_closest_collision<T: Collidable>(ray: &Ray, hit_ables: &Vec<T>) -> Option<Collision> {
    let mut closest = f64::MAX;
    let mut closest_collision: Option<Collision> = None;
    for hit_able in hit_ables {
        if let Some(collision) = hit_able.collide(ray, 0.001, closest) {
            closest = collision.t;
            closest_collision = Some(collision);
        }
    }
    return closest_collision;
}


// the ray emits "photons" i.e. light through the space
// if it collides with some object it should change the color
// depending on the hit angle + material of the collision color
pub fn get_ray_color<T: Collidable>(ray: Ray, world: &Vec<T>, max_depth: usize) -> Color {
    if max_depth == 0 {
        return vec3!();
    }
    if let Some(collision) = get_closest_collision(&ray, world) {
        if let Some((scattered_ray, scattered_color)) = collision.material.scatter(
            &ray,
            &collision.pos,
            &collision.normal,
            collision.ray_is_inside,
            &collision.material,
        ) {
            return scattered_color * get_ray_color(scattered_ray, world, max_depth - 1);
        }
        return vec3!(0.0, 0.0, 0.0);
    }
    let unit_direction: Vec3 = ray.direction.to_unit_vec();
    let t: f64 = (0.5 * (unit_direction.y + 1.0)).clamp(0.0, 1.0);
    let color = (1.0 - t) * vec3!(1.0, 1.0, 1.0) + t * vec3!(0.5, 0.7, 1.0);
    return color;
}

