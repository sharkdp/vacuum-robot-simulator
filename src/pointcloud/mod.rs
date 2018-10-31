use geometry::Point;
use std::slice::{Iter, IterMut};

pub struct PointCloud {
    points: Vec<Point>,
}

impl PointCloud {
    pub fn new(points: Vec<Point>) -> PointCloud {
        PointCloud { points }
    }

    pub fn empty() -> PointCloud {
        PointCloud { points: vec![] }
    }

    pub fn size(&self) -> usize {
        self.points.len()
    }

    pub fn add(&mut self, p: Point) {
        self.points.push(p);
    }

    pub fn iter(&self) -> Iter<Point> {
        self.points.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Point> {
        self.points.iter_mut()
    }
}

pub fn raycast() -> PointCloud {
    let mut cloud = PointCloud::empty();

    cloud.add(Point::new(0.0, 0.0));
    cloud.add(Point::new(3.0, 4.0));
    cloud.add(Point::new(4.0, 5.0));

    cloud
}
