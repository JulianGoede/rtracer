use crate::vec::{ZERO, Vec3};

static mut RNG_STATE: i32 = 44;
const RNG_A: i32 = 8121;
const RNG_C: i32 = 28411;
const RNG_M: i32 = 134456;

pub fn rand_f64(t_min: f64, t_max: f64) -> f64 {
    // unsafe is fine as we don't rely on a deterministic random number
    debug_assert!(t_min <= t_max);
    let mut t = 0.0;
    unsafe {
        RNG_STATE = (RNG_A * RNG_STATE + RNG_C) % RNG_M;
        t += (RNG_STATE as f64) / ((RNG_M - 1) as f64);
    }
    (t_max - t_min) * t + t_min * t
}

pub fn rand_vec(min_val: f64, max_val: f64) -> Vec3 {
    Vec3 {
        x: rand_f64(min_val, max_val),
        y: rand_f64(min_val, max_val),
        z: rand_f64(min_val, max_val),
    }
}

pub fn rand_unit_vec() -> Vec3 {
    loop {
        let v = rand_vec(-10.0, 10.0);
        if v != ZERO {
            return v.to_unit_vec();
        }
    }
}

#[test]
fn test_rand_unit_vec_has_norm_one() {
    let v = rand_unit_vec();
    let actual = v.norm();
    let expected = 1.0;

    assert!(f64::abs(actual - expected) < f64::EPSILON);
    assert_eq!(actual, expected);
}
