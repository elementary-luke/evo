use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::AddAssign;
use std::ops::MulAssign;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Point
{
    pub x : f32,
    pub y : f32,
}

impl Point
{
    pub fn from(tuple : (f32, f32)) -> Point
    {
        Point {x : tuple.0, y : tuple.1}
    }
}

impl Point 
{
    pub fn magnitude(&self) -> f32
    {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    pub fn normalised(&self) -> Point
    {
        let x = self.x / self.magnitude();
        let y = self.y / self.magnitude();
        Point {x, y}
    }
}

impl Add for Point
{
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

impl AddAssign for Point
{
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}


impl Sub for Point
{
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Mul for Point
{
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {x: self.x * other.x, y: self.y * other.y}
    }
}

impl MulAssign for Point
{
    fn mul_assign(&mut self, other: Point) {
        self.x *= other.x;
        self.y *= other.y;
    }
}