use crate::ray::Ray;
use crate::rng::rand_f64;
use crate::vec::Vec3;
use crate::vec3;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,

    // camera coordinate system
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

pub fn setup_camera(
    look_from: Vec3,
    look_at: Vec3,
    up: Vec3, // ortorgonal to view direction look_from -> look_at
    field_of_view: f64,
    aspect_ratio: f64,
    aperture: f64, // control deblurring
    focus_distance: f64,
) -> Camera {
    let theta = field_of_view.to_radians();
    let h = (theta / 2.0).tan();

    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    // define the virtual hyperplanes horizontal, vertical etc.
    let w = (look_from - look_at).to_unit_vec();
    // get normal (perpendicular) vector w.r.t. to viewing direction
    // and "up". Flip, to let u point upwards aswell
    let u = -(w.cross(&up)).to_unit_vec();
    // get normal (perpendicular) w.r.t. w and viewing direction
    let v = w.cross(&u);

    // in total we have a new linear basis lin(cam):=lin{w, u, v} which are the camera coordinates
    let origin = look_from;
    let horizontal = focus_distance * viewport_width * u;
    let vertical = focus_distance * viewport_height * v;
    // lower left corner of the canvas
    let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - focus_distance * w;

    let lens_radius = aperture / 2.0;

    Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical,
        u,
        v,
        w,
        lens_radius,
    }
}

// Return the ray starting from camera origin and moving through the
// normalized image pixle coordinates (x, y)
impl Camera {
    pub fn send_ray_towards(&self, x: f64, y: f64) -> Ray {
        let random_xy_unit_vec = vec3!(rand_f64(-1.0, 1.0), rand_f64(-1.0, 1.0), 0.0).to_unit_vec();
        let random_direction = self.lens_radius * random_xy_unit_vec;
        let offset: Vec3 = self.u * random_direction.x + self.v * random_direction.y;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + x * self.horizontal + y * self.vertical
                - (self.origin + offset),
        }
    }
}
