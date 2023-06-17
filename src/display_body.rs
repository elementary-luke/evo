use crate::body::Body;
use crate::point::*;
use crate::circle::*;
use crate::muscle::*;
use crate::settings::Settings;
use macroquad::color;
use macroquad::prelude::BLACK;
use macroquad::prelude::Color;
use macroquad::prelude::DARKGRAY;
use macroquad::prelude::Rect;
use macroquad::prelude::vec2;
use macroquad::qrand as rand;
use macroquad::shapes::draw_line;
use macroquad::shapes::draw_rectangle;
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
    pub dist : f32,
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
            dist : 0.0,
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
            dist : body.distance.unwrap(),
        }
        
    }

    pub fn draw(&mut self, par_pos : Point)
    {
        self.muscles.iter_mut().for_each(|m| m.draw(par_pos + self.pos, self.circles[m.from].pos, self.circles[m.to].pos));
        self.circles.iter_mut().for_each(|c| c.draw(par_pos + self.pos));
        //println!("{:?} {:?}  {:?}", self.body_array_index, par_pos, self.pos);
        draw_text(&format!("Place: {}", self.body_array_index.1 + 1), par_pos.x + self.pos.x - 100.0, par_pos.y + self.pos.y + 200.0, 40.0, BLACK);
        draw_text(&format!("Distance: {:.1}", self.dist), par_pos.x + self.pos.x - 100.0, par_pos.y + self.pos.y + 250.0, 30.0, DARKGRAY);
    }

    pub fn mouse_on(&mut self, mouse_pos : Point) -> bool
    {
        //println!("{:?} {:?}", mouse_pos, self.pos); 
        
        if Rect::new(self.pos.x - 100.0, self.pos.y - 150.0, 200.0, 300.0).contains(vec2(mouse_pos.x, mouse_pos.y))
        {
            draw_rectangle(self.pos.x-100.0, self.pos.y - 150.0, 200.0, 300.0, Color::from_rgba(0, 0, 0, 122));
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

