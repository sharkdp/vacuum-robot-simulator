use super::{Point, Ray};

pub trait Target {
    fn intersect(&self, ray: &Ray) -> Vec<Point>;
}
