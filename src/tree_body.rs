use crate::body::Body;
use crate::point::*;
use crate::circle::*;
use crate::muscle::*;
use crate::settings::Settings;
use macroquad::prelude::BLACK;
use macroquad::qrand as rand;
use macroquad::text::draw_text;
use std::cmp::min;

#[derive(Clone, Debug)]
pub struct TreeBody
{
    pub pos : Point,
    pub circles : Vec<Circle>,
    pub muscles : Vec<Muscle>,
    pub parent : Option<(usize, usize)>,
    pub children : Vec<(usize, usize)>,
    pub body_array_index : (usize, usize),
}

impl Default for TreeBody
{
    fn default() -> Self
    {
        TreeBody 
        {
            pos : Point {x : 0.0, y : 0.0},
            circles : Vec::new(),
            muscles : Vec::new(),
            parent : None,
            children : Vec::new(),
            body_array_index : (4200, 4200),
        }
    }
}

impl TreeBody
{
    pub fn new() -> TreeBody // new empty body
    {
        let circles : Vec<Circle> = Vec::new();
        let muscles : Vec<Muscle> = Vec::new();
        let body = TreeBody {..Default::default()};
        body
    }

    pub fn from(body : Body, index : (usize, usize)) -> TreeBody // new body from body
    {
        TreeBody
        {
            pos : body.pos,
            circles : body.circles,
            muscles : body.muscles,
            parent : body.parent,
            children : body.children,
            body_array_index : index,
        }
        
    }

    pub fn draw(&mut self)
    {
        self.muscles.iter_mut().for_each(|m| m.draw(self.pos, self.circles[m.from].pos, self.circles[m.to].pos));
        self.circles.iter_mut().for_each(|c| c.draw(self.pos)); // TODO + parent pos
         draw_text(&format!("Gen: {}, Index: {}", self.body_array_index.0, self.body_array_index.1), self.pos.x - 100.0, self.pos.y + 200.0, 20.0, BLACK);
        if self.parent.is_some()
        {
            draw_text(&format!("Par Gen: {}, Index: {}", self.parent.unwrap().0, self.parent.unwrap().1), self.pos.x - 100.0, self.pos.y + 250.0, 20.0, BLACK);
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

