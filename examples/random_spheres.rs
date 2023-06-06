use std::{
    array,
    fs::File,
    io::{Result, Write},
};

extern crate rtracer;

use rtracer::rng::rand_f64;
use rtracer::shape::Sphere;
use rtracer::vec::{Color, Vec3};
use rtracer::{camera::setup_camera, vec3};
use rtracer::{get_ray_color, write_color};
use rtracer::{
    material::{Material, WINDOW_GLASS_REFRACTION},
    rng,
};

#[allow(dead_code)]
fn rand_sphere() -> Sphere {
    let sphere = Sphere {
        center: rtracer::vec3! {rand_f64(-1.0, 1.0), rand_f64(-0.3, 0.7), -1.0},
        // radius: rand_f64(0.2, 0.4)
        radius: 0.4,
        material: Material::Metal {
            albedo: rtracer::vec3!(rand_f64(0.0, 1.0), rand_f64(0.0, 1.0), rand_f64(0.0, 1.0)),
            fuzzyness: rand_f64(0.0, 1.0),
        },
    };
    println!("{:?}", sphere);
    sphere
}

fn random_world() -> Vec<Sphere> {
    let mut world: Vec<Sphere> = vec![];
    let ground = Sphere {
        center: vec3!(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian {
            albedo: vec3!(0.5, 0.5, 0.5),
        },
    };
    world.push(ground);

    for a in -11..11 {
        for b in -11..11 {
            let random_material = rand_f64(0.0, 1.0);
            let center = vec3!(
                a as f64 + 0.9 * rand_f64(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * rand_f64(0.0, 1.0)
            );

            if (center - vec3!(4.0, 0.2, 0.0)).norm() > 0.9 {
                if random_material < 0.8 {
                    let albedo = rng::rand_vec(0.0, 1.0);
                    world.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Lambertian { albedo },
                    });
                } else if random_material < 0.95 {
                    let albedo = rng::rand_vec(0.5, 1.0);
                    let fuzzyness = rand_f64(0.5, 1.0);
                    world.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal { albedo, fuzzyness },
                    });
                } else {
                    world.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dialectric {
                            refraction_index: WINDOW_GLASS_REFRACTION,
                        },
                    });
                }
            }
        }
    }
    world.push(Sphere {
        center: vec3!(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dialectric {
            refraction_index: WINDOW_GLASS_REFRACTION,
        },
    });
    world.push(Sphere {
        center: vec3!(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian {
            albedo: vec3!(0.4, 0.2, 0.1),
        },
    });
    world.push(Sphere {
        center: vec3!(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal {
            albedo: vec3!(0.7, 0.6, 0.5),
            fuzzyness: 0.5,
        },
    });

    return world;
}

fn write_ray_tracer_image(file_name: &str, image_width: usize) -> std::io::Result<()> {
    // image specs
    let aspect_ratio = 3.0 / 2.0;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;
    let samples_per_pixel = 200;
    let max_depth = 20;

    let normalization_factor = 1.0 / samples_per_pixel as f64;

    let world: Vec<Sphere> = random_world();

    let look_from = rtracer::vec3!(13.0, 2.0, 3.0);
    let look_at = rtracer::vec3!(0.0, 0.0, 0.0);
    let up = rtracer::vec3!(0.0, 1.0, 0.0);

    let distance_to_focus_plane = 10.0;
    let aperture = 0.1;

    let camera = setup_camera(
        look_from,
        look_at,
        up,
        45.0,
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
            let mut pixel_color: Color = rtracer::vec3!(0.0, 0.0, 0.0);
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
        println!("row = {:?};", image_height-1-i);
    }

    Result::Ok(())
}

fn main() -> std::io::Result<()> {
    write_ray_tracer_image("random_spheres.ppm", 1200)
}
