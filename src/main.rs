use std::{
    fs::File,
    io::{Result, Write},
};

mod camera;
mod material;
mod ray;
mod rng;
mod shape;
mod vec;

use camera::setup_camera;
use material::{
    Material, Reflectable, DIAMOND_REFRACTION, WATER_20_CELSIUS_REFRACTION, WINDOW_GLASS_REFRACTION,
};
use ray::Ray;
use rng::rand_f64;
use shape::{Collidable, Collision, Sphere};
use vec::{Color, Vec3};

const COLOR_MAX: f64 = 255f64;

fn write_color(file: &mut File, pixel_color: Color, gamma_scale: f64) -> std::io::Result<()> {
    let color_x = (COLOR_MAX * (pixel_color.x * gamma_scale).sqrt()) as i32;
    let color_y = (COLOR_MAX * (pixel_color.y * gamma_scale).sqrt()) as i32;
    let color_z = (COLOR_MAX * (pixel_color.z * gamma_scale).sqrt()) as i32;
    let r = color_x.clamp(0i32, 255i32);
    let g = color_y.clamp(0i32, 255i32);
    let b = color_z.clamp(0i32, 255i32);
    file.write_fmt(format_args!("{} {} {}\n", r, g, b))?;
    return Ok(());
}

#[allow(dead_code)]
fn rand_sphere() -> Sphere {
    let sphere = Sphere {
        center: vec3! {rand_f64(-1.0, 1.0), rand_f64(-0.3, 0.7), -1.0},
        // radius: rand_f64(0.2, 0.4)
        radius: 0.4,
        material: Material::Metal {
            albedo: vec3!(rand_f64(0.0, 1.0), rand_f64(0.0, 1.0), rand_f64(0.0, 1.0)),
            fuzzyness: rand_f64(0.0, 1.0),
        },
    };
    println!("{:?}", sphere);
    sphere
}

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
fn get_ray_color<T: Collidable>(ray: Ray, world: &Vec<T>, max_depth: usize) -> Color {
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
    // let t: f64 = 0.5 * (unit_direction.y + 1.0);
    let color = (1.0 - t) * vec3!(1.0, 1.0, 1.0) + t * vec3!(0.5, 0.7, 1.0);
    return color;
}

fn write_ray_tracer_image(file_name: &str, image_width: usize) -> std::io::Result<()> {
    // image specs
    let aspect_ratio = 16.0 / 9.0;
    let image_height: usize = ((image_width as f64) / aspect_ratio) as usize;
    // let samples_per_pixel: usize = 100;
    let samples_per_pixel: usize = 50;

    // maximal number of ray bounces (i.e. reflections)
    let max_depth: usize = 110;
    let normalization_factor: f64 = 1.0 / (samples_per_pixel as f64);

    // setup world
    let ground = Sphere {
        center: vec3! {0.0, -100.5, -1.0},
        radius: 100.0,
        material: Material::Lambertian {
            albedo: vec3!(0.8, 0.8, 0.0),
        },
    };

    let center_ball = Sphere {
        center: vec3! {0.0, 0.0, -1.0},
        radius: 0.5,
        material: Material::Lambertian {
            albedo: vec3!(0.1, 0.2, 0.5),
        },
    };

    let left_ball = Sphere {
        center: vec3! {-1.0, 0.0, -1.0},
        radius: 0.5,
        material: Material::Dialectric {
            refraction_index: WINDOW_GLASS_REFRACTION,
            // refraction_index: WATER_20_CELSIUS_REFRACTION,
            // refraction_index: DIAMOND_REFRACTION,
        },
    };

    let left_inner_ball = Sphere {
        center: vec3! {-1.0, 0.0, -1.0},
        radius: -0.45,
        material: Material::Dialectric {
            refraction_index: WINDOW_GLASS_REFRACTION,
        },
    };

    let right_ball = Sphere {
        center: vec3! {1.0, 0.0, -1.0},
        radius: 0.5,
        material: Material::Metal {
            albedo: vec3!(0.8, 0.6, 0.2),
            fuzzyness: 0.0,
        },
    };

    // let world = vec![center_ball, ground, left_ball, left_inner_ball, right_ball];
    let world = vec![center_ball, ground, left_ball, right_ball];

    let look_from = vec3!(-2.0, 2.0, 1.0);
    let look_at = vec3!(0.0, 0.0, -1.0);
    let up = vec3!(0.0, 1.0, 0.0);

    let distance_to_focus_plane = (look_from - look_at).norm();
    let aperture = 0.5;

    let camera = setup_camera(
        look_from,
        look_at,
        up,
        20.0,
        aspect_ratio,
        aperture,
        distance_to_focus_plane,
    );

    // render
    let mut file = File::create(file_name)?;
    file.write_fmt(format_args!("P3\n{} {}\n255\n", image_width, image_height))?;
    // println!("P3\n{} {}\n255\n", image_width, image_height);

    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let mut pixel_color: Color = vec3!(0.0, 0.0, 0.0);
            // antialise by using samples_per_pixel random points close to the actual pixels
            for _sample in 0..samples_per_pixel {
                let u = ((j as f64) + rand_f64(0.0, 0.999)) / ((image_width - 1) as f64);
                let v = ((i as f64) + rand_f64(0.0, 0.999)) / ((image_height - 1) as f64);
                // send a ray towards the current pixel
                // actually, we pick samples_per_pixel many random points close to the normalized pixel
                let ray = camera.send_ray_towards(u, v);
                // add the ray color to our pixel color
                pixel_color = pixel_color + get_ray_color(ray, &world, max_depth);
            }
            write_color(&mut file, pixel_color, normalization_factor)?;
        }
        // println!("DONE - Iteration {:?}", i);
    }

    Result::Ok(())
}

fn main() -> std::io::Result<()> {
    write_ray_tracer_image("foo.ppm", 400)
}
