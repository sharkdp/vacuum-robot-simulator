use super::point::Point;
use super::ray::Ray;

pub trait Target {
    fn intersect(&self, ray: &Ray) -> Option<Point>;
}
