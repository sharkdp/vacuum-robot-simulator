use super::vector::Vector;
// use super::ray::Ray;
// use super::point::Point;
// use super::target::Target;

pub struct Line {
    pub start: Vector,
    pub end:   Vector
}

impl Line {
    pub fn new(start: Vector, end: Vector) -> Line {
        Line { start, end }
    }
}

// impl Target for Line {
//     fn intersect(&self, ray: &Ray) -> Option<Point> {
//         let p = ray.origin;
//         let r = ray.direction;
//         let q = self.origin;
//         let s = self.direction;

//         let d = r.cross(s);

//         if d == 0.0 {
//             None
//         } else {
//             let u = (q - p).cross(r) / d;
//             let hit = q + (s * u);

//             Some(Point::from_vector(hit))
//         }
//     }
// }
