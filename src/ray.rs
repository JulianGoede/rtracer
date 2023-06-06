use crate::vec::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! ray {
    () => {
        Ray {
            origin: crate::vec3!(),
            direction: crate::vec3!(1.0, 0.0, 0.0),
        }
    };
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        t * self.direction + self.origin
    }
}

#[test]
fn test_ray_macro() {
    let actual = ray!();
    let expected = Ray {
        origin: crate::vec3!(),
        direction: crate::vec3!(1.0, 0.0, 0.0),
    };
    assert_eq!(actual, expected);
}
