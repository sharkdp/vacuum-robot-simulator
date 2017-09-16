use std::fmt;
use std::ops;
use std::cmp;
use std::slice::{Iter, IterMut};

type Scalar = f64;

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: Scalar,
    y: Scalar
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::Mul<Scalar> for Vector {
    type Output = Vector;

    fn mul(self, s: Scalar) -> Vector {
        Vector::new(self.x * s, self.y * s)
    }
}

impl cmp::PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Vector {
    fn new(x: Scalar, y: Scalar) -> Vector {
        Vector { x, y }
    }

    fn length(&self) -> Scalar {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn cross(&self, q: Vector) -> Scalar {
        self.x * q.y - q.x * self.y
    }

    fn angle(&self) -> Scalar {
        self.y.atan2(self.x)
    }
}

// ------------------------------------------

#[derive(Debug, Clone)]
struct Point {
    pos: Vector
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.pos.x, self.pos.y)
    }
}

impl cmp::PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.pos == other.pos
    }
}

impl Point {
    fn new(x: Scalar, y: Scalar) -> Point {
        Point { pos: Vector::new(x, y) }
    }

    fn from_vector(pos: Vector) -> Point {
        Point { pos }
    }

    fn distance(&self) -> Scalar {
        self.pos.length()
    }
}

// ------------------------------------------

struct PointCloud {
    points: Vec<Point>
}

impl PointCloud {
    fn new(points: Vec<Point>) -> PointCloud {
        PointCloud { points }
    }

    fn empty() -> PointCloud {
        PointCloud { points: vec!() }
    }

    fn size(&self) -> usize {
        self.points.len()
    }

    fn add(&mut self, p: Point) {
        self.points.push(p);
    }

    fn iter(&self) -> Iter<Point> {
        self.points.iter()
    }

    fn iter_mut(&mut self) -> IterMut<Point> {
        self.points.iter_mut()
    }
}

// ------------------------------------------

struct Ray {
    origin: Vector,
    direction: Vector
}

impl Ray {
    fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
}

trait Target {
    fn intersect(&self, ray: &Ray) -> Option<Point>;
}

// ------------------------------------------

struct Line {
    origin: Vector,
    direction: Vector
}

impl Line {
    fn new(origin: Vector, direction: Vector) -> Line {
        Line { origin, direction }
    }
}

impl Target for Line {
    fn intersect(&self, ray: &Ray) -> Option<Point> {
        let p = ray.origin;
        let r = ray.direction;
        let q = self.origin;
        let s = self.direction;

        let d = r.cross(s);

        if d == 0.0 {
            None
        } else {
            let u = (q - p).cross(r) / d;
            let hit = q + (s * u);

            Some(Point::from_vector(hit))
        }
    }
}

// ------------------------------------------

fn raycast() -> PointCloud {
    let mut cloud = PointCloud::empty();

    cloud.add(Point::new(0.0, 0.0));
    cloud.add(Point::new(3.0, 4.0));
    cloud.add(Point::new(4.0, 5.0));

    cloud
}

// ------------------------------------------

fn main() {
    let p1 = Point::new(3.0, 4.0);
    let p2 = Point::new(2.0, 10.0);

    println!("p1 = {}", p1);
    println!("p2 = {}", p2);
    println!("p1 != p2 = {}", p1 != p2);
    println!("p1.distance() = {}", p1.distance());
    println!("p2.distance() = {}", p2.distance());

    let pc = raycast();

    for p in pc.iter().filter(|p| p.distance() <= 6.0) {
        println!("* {}", p);
    }

    let ray = Ray::new(Vector::new(0.0, 0.0), Vector::new(-1.0, 0.0));
    let line = Line::new(Vector::new(10.0, 10.0), Vector::new(-1.0, 1.0));
    println!("intersection = {:?}", line.intersect(&ray));
}
