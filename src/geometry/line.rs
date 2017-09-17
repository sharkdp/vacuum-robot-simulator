use super::{Vector, Target, Ray, Point};

pub struct Line {
    pub start: Vector,
    pub end:   Vector
}

impl Line {
    pub fn new(start: Vector, end: Vector) -> Line {
        Line { start, end }
    }
}

impl Target for Line {
    fn intersect(&self, ray: &Ray) -> Vec<Point> {
        let p = ray.origin;
        let r = ray.direction;
        let q = self.start;
        let s = self.end - self.start;

        let d = r.cross(s);

        if d == 0.0 {
            vec!()
        } else {
            let u = (q - p).cross(r) / d;
            let hit = q + (s * u);

            vec!(Point::from_vector(hit))
        }
    }
}
