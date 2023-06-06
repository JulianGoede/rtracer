use crate::ray::Ray;
use crate::rng::{rand_f64, rand_unit_vec};
use crate::vec::{Color, Vec3};
use crate::vec3;

#[allow(dead_code)]
pub const VACUUM_REFRACTION: f64 = 1.0;
#[allow(dead_code)]
pub const WINDOW_GLASS_REFRACTION: f64 = 1.52;
#[allow(dead_code)]
pub const WATER_20_CELSIUS_REFRACTION: f64 = 1.333;
#[allow(dead_code)]
pub const DIAMOND_REFRACTION: f64 = 2.417;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Material {
    // albedo is a latin word
    // it is a measure for the amount of light reflexion
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzzyness: f64 },
    // glass, diamond etc
    Dialectric { refraction_index: f64 },
}

fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    return *v - 2.0 * v.dot(normal) * *normal;
}

fn refract(v: &Vec3, normal: &Vec3, refraction_ratio: f64) -> Vec3 {
    let cos_theta_1 = (-v.dot(normal)).min(1.0);
    let cos_theta_2 = (1.0
        - refraction_ratio * refraction_ratio * (1.0 - cos_theta_1 * cos_theta_1))
        .max(0.0)
        .sqrt();
    return refraction_ratio * *v + (refraction_ratio * cos_theta_1 - cos_theta_2) * *normal;
}

pub fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
    // Schlick approximation
    assert_ne!(refraction_ratio, -1.0);
    let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    r0 *= r0;
    return r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);
}

pub trait Reflectable {
    fn scatter(
        &self,
        input_ray: &Ray,
        reflection_point: &Vec3,
        reflection_normal: &Vec3,
        ray_is_inside: bool,
        source_material: &Material,
    ) -> Option<(Ray, Color)>;
}

impl Reflectable for Material {
    fn scatter(
        &self,
        input_ray: &Ray,
        reflection_point: &Vec3,
        reflection_normal: &Vec3,
        ray_is_inside: bool,
        source_material: &Material,
    ) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian { albedo } => {
                let random_unit_vec = rand_unit_vec();
                let mut scatter_direction = if reflection_normal.dot(&random_unit_vec) > 0.0 {
                    // unit vec points from collision outwards
                    *reflection_normal + random_unit_vec
                } else {
                    // unit vec points from collision inwards, hence we subtract to point outwards
                    *reflection_normal - random_unit_vec
                };
                if scatter_direction.almost_zero() {
                    scatter_direction = *reflection_normal;
                }
                let scattered_ray = Ray {
                    origin: *reflection_point,
                    direction: scatter_direction,
                };
                return Some((scattered_ray, albedo.clone()));
            }
            Material::Metal { albedo, fuzzyness } => {
                // normalized input direction =: v
                let v = input_ray.direction.to_unit_vec();
                let reflection = reflect(&v, &reflection_normal);

                let fuzzy_random_unit_vec: Vec3 = fuzzyness.min(1.0) * rand_unit_vec();
                let scatter_direction = if reflection_normal.dot(&fuzzy_random_unit_vec) > 0.0 {
                    // unit vec points from collision outwards
                    reflection + fuzzy_random_unit_vec
                } else {
                    // unit vec points from collision inwards, hence we subtract to point outwards
                    reflection - fuzzy_random_unit_vec
                };

                let scattered_ray = Ray {
                    origin: *reflection_point,
                    direction: scatter_direction,
                };
                if scattered_ray.direction.dot(&reflection_normal) > 0.0 {
                    return Some((scattered_ray, albedo.clone()));
                }
                return None;
            }
            Material::Dialectric { refraction_index } => {
                let attenuation: Color = vec3!(1.0, 1.0, 1.0);

                let source_refraction = VACUUM_REFRACTION;

                let refraction_ratio = if ray_is_inside {
                    *refraction_index / source_refraction
                } else {
                    source_refraction / refraction_index
                };

                let unit_direction = input_ray.direction.to_unit_vec();
                let cos_theta = (-unit_direction.dot(&reflection_normal)).min(1.0);
                // 1 = cos(theta)^2 + sin(theta)^2 iff sin(theta) = sqrt(1-cos(theta)^2)
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let reflection_coefficient = reflectance(cos_theta, refraction_ratio);

                let should_reflect = cannot_refract || reflection_coefficient > rand_f64(0.0, 1.0);
                // let should_reflect = cannot_refract;

                let direction = if should_reflect {
                    reflect(&unit_direction, reflection_normal)
                } else {
                    refract(&unit_direction, reflection_normal, refraction_ratio)
                };
                Some((
                    Ray {
                        origin: *reflection_point,
                        direction,
                    },
                    attenuation,
                ))
            }
        }
    }
}

macro_rules! test_dialectric_refraction_angle {
    ($($name: ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name(){
                let (theta_1, refraction_index, expected) = $value;
                let v = vec3!(theta_1.cos(), theta_1.sin(), 0.0);
                let n = vec3!(-1.0, 0.0, 0.0);

                { // sanity checks
                    assert_eq!(v.norm(), 1.0);
                    assert_eq!(n.norm(), 1.0);
                    // should point in different directions
                    assert_eq!(v.dot(&n).signum(), -1.0);
                    let angle = v.dot(&(-n)).min(1.0).acos().to_degrees();
                    assert!((angle - theta_1.to_degrees()).abs() < 1e-4, "angle = {:?} != {:?} = theta", angle, theta_1);
                }


                let refracted = refract(&v, &n, 1.0/refraction_index);
                let actual = (refracted.dot(&(-n)).min(1.0).acos() / refracted.norm()).to_degrees();
                assert!((actual - expected).abs() < 1e-3, "Actual = {:?} != {:?} = Expected", actual, expected);
            }
        )*
    }
}

test_dialectric_refraction_angle! {
    vacuum_to_glass: (30f64.to_radians(), WINDOW_GLASS_REFRACTION, 19.2049),
    glass_to_vacuum: (25f64.to_radians(), 1.0/WINDOW_GLASS_REFRACTION, 39.9695),
    vacuum_to_water: (27f64.to_radians(), WATER_20_CELSIUS_REFRACTION, 19.9121),
}
