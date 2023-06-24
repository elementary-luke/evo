use crate::body::Body;
use crate::point::*;
use crate::circle::*;
use crate::muscle::*;
use macroquad::prelude::BLACK;
use macroquad::prelude::Color;
use macroquad::prelude::Rect;
use macroquad::prelude::vec2;
use macroquad::qrand as rand;
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text;
use std::fmt;

#[derive(Clone)]
pub struct TreeBody
{
    pub pos : Point,
    pub circles : Vec<Circle>,
    pub muscles : Vec<Muscle>,
    pub parent : Option<(usize, usize)>,
    pub children : Vec<(usize, usize)>,
    pub body_array_index : (usize, usize),
    pub parent_tree_body : Option<(usize, usize)>,
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
            parent_tree_body : None,
        }
    }
}

impl fmt::Debug for TreeBody
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.body_array_index)
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
            pos : Point {x : 0.0, y : 0.0},
            circles : body.circles,
            muscles : body.muscles,
            parent : body.parent,
            children : body.children,
            body_array_index : index,
            parent_tree_body : None,
        }
        
    }

    pub fn draw(&mut self, par_pos : Point)
    {
        self.muscles.iter_mut().for_each(|m| m.draw(par_pos + self.pos, self.circles[m.from].pos, self.circles[m.to].pos));
        self.circles.iter_mut().for_each(|c| c.draw(par_pos + self.pos));
        //println!("{:?} {:?}  {:?}", self.body_array_index, par_pos, self.pos);
        draw_text(&format!("Birth year: {}, Place: {}", self.body_array_index.0, self.body_array_index.1 + 1), par_pos.x + self.pos.x - 100.0, par_pos.y + self.pos.y + 200.0, 20.0, BLACK);
        if self.parent.is_some()
        {
            draw_text(&format!("Par Year: {}, Index: {}", self.parent.unwrap().0, self.parent.unwrap().1 + 1), par_pos.x + self.pos.x - 100.0, par_pos.y + self.pos.y + 250.0, 20.0, BLACK);
        }

        // draw_line(par_pos.x, par_pos.y + 200.0, par_pos.x + self.pos.x, par_pos.y + 200.0, 1.0, macroquad::prelude::GREEN);
        // draw_line(par_pos.x + self.pos.x, par_pos.y + 200.0, par_pos.x + self.pos.x, par_pos.y + self.pos.y + 200.0, 1.0, macroquad::prelude::GREEN);
    }

    pub fn mouse_on(&mut self, mouse_pos : Point) -> bool
    {
        //println!("{:?} {:?}", mouse_pos, self.pos); 
        if Rect::new(self.pos.x - 100.0, self.pos.y - 150.0, 200.0, 300.0).contains(vec2(mouse_pos.x, mouse_pos.y))
        {
            draw_rectangle(self.pos.x-100.0, self.pos.y - 150.0, 200.0, 300.0, Color::from_rgba(0, 0, 0, 61));
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

