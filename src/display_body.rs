use crate::body::Body;
use crate::point::*;
use crate::circle::*;
use crate::muscle::*;
use crate::settings::Settings;
use macroquad::prelude::BLACK;
use macroquad::qrand as rand;
use macroquad::shapes::draw_line;
use macroquad::text::draw_text;
use std::cmp::min;
use std::fmt;

#[derive(Clone)]
pub struct DisplayBody
{
    pub pos : Point,
    pub circles : Vec<Circle>,
    pub muscles : Vec<Muscle>,
    pub body_array_index : (usize, usize),
}

impl Default for DisplayBody
{
    fn default() -> Self
    {
        DisplayBody 
        {
            pos : Point {x : 0.0, y : 0.0},
            circles : Vec::new(),
            muscles : Vec::new(),
            body_array_index : (4200, 4200),
        }
    }
}

impl fmt::Debug for DisplayBody
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.body_array_index)
    }
}

impl DisplayBody
{
    pub fn new() -> DisplayBody // new empty body
    {
        let circles : Vec<Circle> = Vec::new();
        let muscles : Vec<Muscle> = Vec::new();
        let body = DisplayBody {..Default::default()};
        body
    }

    pub fn from(body : Body, index : (usize, usize)) -> DisplayBody // new body from body
    {
        DisplayBody
        {
            pos : Point {x : 0.0, y : 0.0},
            circles : body.circles,
            muscles : body.muscles,
            body_array_index : index,
        }
        
    }

    pub fn draw(&mut self, par_pos : Point)
    {
        self.muscles.iter_mut().for_each(|m| m.draw(par_pos + self.pos, self.circles[m.from].pos, self.circles[m.to].pos));
        self.circles.iter_mut().for_each(|c| c.draw(par_pos + self.pos));
        //println!("{:?} {:?}  {:?}", self.body_array_index, par_pos, self.pos);
        draw_text(&format!("Place: {}", self.body_array_index.1 + 1), par_pos.x + self.pos.x - 100.0, par_pos.y + self.pos.y + 200.0, 20.0, BLACK);
    }

    pub fn mouse_on(&mut self, mouse_pos : Point) -> bool
    {
        //println!("{:?} {:?}", mouse_pos, self.pos); 
        if (mouse_pos.x - self.pos.x).abs() < 100.0 && (mouse_pos.y - self.pos.y).abs() < 150.0
        {
            return true;
        }
        else 
        {
            return false;
        }
    }
}

fn factorial(n : usize) -> usize
{
    (1..=n).product()
}

fn rand_biased(min : i32, max : i32, p : f32) -> i32
{
    // the higher p is the more biased towards min it is https://gamedev.stackexchange.com/questions/116832/random-number-in-a-range-biased-toward-the-low-end-of-the-range/116875#116875
    (min as f32 + (max - min) as f32 * (rand::gen_range(-1.0, 1.0) as f32).powf(p).abs()).round() as i32
}

