use macroquad::prelude::*;
pub struct Dot
{
    pub x : f32,
    pub y : f32,
    pub r : f32,
    pub color : Color,
    pub friction : f32,
}

impl Dot
{
    pub fn draw(&mut self)
    {
        draw_circle(self.x, self.y, self.r, self.color)
    }
}

pub struct Muscle
{
    pub from : Dot,
    pub to : Dot,
    pub extended_len : f32,
    pub contracted_len : f32,
    pub strength : f32,
}

impl Muscle
{
    pub fn draw(&mut self)
    {
        
    }
}

pub struct Body
{
    pub circles : Vec<Dot>,
    pub muscles : Vec<Muscle>,
}

impl Body
{
    pub fn new() -> Body // new empty body
    {
        let mut circles : Vec<Dot> = Vec::new();
        let mut muscles : Vec<Muscle> = Vec::new();
        let mut body : Body = Body {circles, muscles};
        body
    }
    pub fn new_random() -> Body
    {
        let mut circles : Vec<Dot> = Vec::new();
        let mut muscles : Vec<Muscle> = Vec::new();
        let mut body : Body = Body {circles, muscles};
        body
    }
    pub fn draw(&mut self)
    {
        self.circles.iter_mut().for_each(|i| i.draw());
        self.muscles.iter_mut().for_each(|i| i.draw());
    }
}


// pub enum ObjTypes
// {
//     Dot(Dot),
//     Muscle(Muscle),
//     Body(Body),
// }