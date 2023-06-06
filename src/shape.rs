use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{Vec3, ZERO};
use crate::vec3;

#[derive(Debug, PartialEq)]
pub struct Collision {
    // position of shape that intersects ray
    pub pos: Vec3,
    pub normal: Vec3,
    pub ray_is_inside: bool,
    // scalar value for which ray R(t):= R.origin + t*R.direction = pos
    pub t: f64,
    pub material: Material,
}

pub trait Collidable {
    // return scalar value t (if any) at which ray.origin + t*ray.direction
    // first intersects collidable body
    fn collide(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision>;
}

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! sphere {
    () => {
        Sphere {
            center: crate::vec3!(),
            radius: 1f64,
            material: Material::Lambertian {
                albedo: crate::vec3!(),
            },
        }
    };
}

impl Collidable for Sphere {
    fn collide(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        let delta: Vec3 = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let half_b = delta.dot(&ray.direction);
        let c = delta.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // find the nearest root t within an acceptable range
        // s.t. ray(t) intersect sphere != empty
        let discriminant_root = discriminant.sqrt();
        let mut root = (-half_b - discriminant_root) / a;
        if root < t_min || root > t_max {
            root = (-half_b + discriminant_root) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        // println!("t={:?}; t_min = {:?}", root, t_min);

        // compute the angle between ray and intersection point
        // to compute a normal that always points towards the ray
        let outward_normal: Vec3 = (ray.at(root) - self.center) * (1.0 / self.radius);
        let ray_is_inside_sphere: bool = ray.direction.dot(&outward_normal) >= 0.0;
        // let the normal point towards the ray
        let normal = match ray_is_inside_sphere {
            true => {
                // println!("Ray is inside");
                -outward_normal
            }
            false => {
                // println!("Ray is outside");
                outward_normal
            }
        };

        return Some(Collision {
            pos: ray.at(root),
            normal,
            ray_is_inside: ray_is_inside_sphere,
            t: root,
            material: self.material,
        });
    }
}

#[test]
fn test_sphere_macro() {
    let actual = sphere!();
    let expected = Sphere {
        center: crate::vec3!(),
        radius: 1f64,
        material: Material::Lambertian {
            albedo: crate::vec3!(),
        },
    };
    assert_eq!(actual, expected);
}

#[test]
fn test_ray_collides_sphere() {
    let material = Material::Dialectric {
        refraction_index: 1.0,
    };
    let sphere = Sphere {
        center: vec3!(0.0, 0.0, -2.0),
        radius: 1.0,
        material,
    };
    let ray = Ray {
        origin: ZERO,
        direction: vec3!(0.0, 0.0, -1.0),
    };
    let actual = sphere.collide(&ray, 0.0, 10.0);

    let expected = Some(Collision {
        pos: vec3!(0.0, 0.0, -1.0),
        normal: vec3!(0.0, 0.0, 1.0),
        ray_is_inside: false,
        t: 1.0,
        material,
    });

    assert_eq!(actual, expected);
}

#[test]
fn test_ray_collides_inside_sphere() {
    let material = Material::Dialectric {
        refraction_index: 1.0,
    };
    let sphere = Sphere {
        center: vec3!(0.0, 0.0, -2.0),
        radius: 1.0,
        material,
    };

    let inside_sphere_pos: Vec3 = vec3!(0.0, 0.0, -1.5);
    let ray = Ray {
        origin: inside_sphere_pos,
        direction: vec3!(0.0, 0.0, -1.0),
    };
    let actual = sphere.collide(&ray, 0.0, 10.0);

    let expected = Some(Collision {
        pos: vec3!(0.0, 0.0, -3.0),
        normal: vec3!(0.0, 0.0, 1.0),
        ray_is_inside: true,
        t: 1.5,
        material,
    });

    assert_eq!(actual, expected);
}

#[test]
fn test_ray_starting_on_boundary_collides_sphere() {
    let material = Material::Dialectric {
        refraction_index: 1.0,
    };
    let sphere = Sphere {
        center: vec3!(0.0, 0.0, -2.0),
        radius: 1.0,
        material,
    };

    let sphere_boundary: Vec3 = vec3!(0.0, 0.0, -1.0);
    let ray = Ray {
        origin: sphere_boundary,
        direction: vec3!(0.0, 0.0, -1.0),
    };
    let t_min = 0.01; // important we enforce t >= 0 here!
    let actual = sphere.collide(&ray, t_min, 10.0);

    let expected = Some(Collision {
        pos: vec3!(0.0, 0.0, -3.0),
        normal: vec3!(0.0, 0.0, 1.0),
        ray_is_inside: true,
        t: 2.0,
        material,
    });

    assert_eq!(actual, expected);
}
