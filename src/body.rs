use crate::point::*;
use crate::circle::*;
use crate::muscle::*;
use macroquad::qrand as rand;
use std::cmp::min;

pub struct Body
{
    pub pos : Point,
    pub circles : Vec<Circle>,
    pub muscles : Vec<Muscle>,
}

impl Body
{
    pub fn new() -> Body // new empty body
    {
        let circles : Vec<Circle> = Vec::new();
        let muscles : Vec<Muscle> = Vec::new();
        let body : Body = Body {pos : Point {x : 0.0, y : 0.0}, circles, muscles};
        body
    }
    pub fn new_random(x_bound : f32, y_bound : f32) -> Body
    {
        rand::srand(macroquad::miniquad::date::now() as u64);
        let circles : Vec<Circle> = Vec::new();
        let muscles : Vec<Muscle> = Vec::new();
        let mut body : Body = Body {pos : Point {x : 0.0, y : 0.0}, circles, muscles};
        
        for _ in 0..rand::gen_range(2, 10) //REVERT TO 2, 10
        {
            let x = rand::gen_range(-x_bound / 2.0, x_bound / 2.0);
            let y = rand::gen_range(-y_bound / 2.0, y_bound / 2.0);

            //was to make sure Circles were overlapping but not needed
            // loop    
            // {
            //     x = rand::gen_range(-x_bound / 2.0, x_bound / 2.0);
            //     y = rand::gen_range(-y_bound / 2.0, y_bound / 2.0);
            //     if body.circles.iter().any(|c| (c.x - x).powi(2) + (c.y - y).powi(2) < (c.r + c.r).powi(2))
            //     {
            //         continue;
            //     }
            //     break;
            // }
            
            body.circles.push(Circle::new_random(Point {x, y})); 
        }
        
        // make sure every circle is connected
        let mut connected : Vec<usize> = Vec::new();
        connected.push(rand::gen_range(0, body.circles.len()));
        for i in 0..body.circles.len()
        {
            if connected.iter().any(|x| *x == i)
            {
                continue;
            }
            body.muscles.push( Muscle::new_random( i, connected[rand::gen_range(0, connected.len())] ) );
            connected.push(i);
        }

        //add a couple more random muscles between random circles but no repeats
        let max_connections = factorial(body.circles.len()) / (2 * factorial(body.circles.len()-2));
        if body.muscles.len() >= max_connections // dont continue if no more possible muscle additions are possible
        {
            return body;
        }
        for _ in 0..rand::gen_range(0, min(max_connections - body.muscles.len(), 5)) 
        {
            //might be too rng and take too long so might have to change VV
            loop
            {
                let from = rand::gen_range(0, body.circles.len());
                let to = rand::gen_range(0, body.circles.len());

                if from == to || body.muscles.iter().any(|m| (m.from == from && m.to == to) || (m.from == to && m.to == from))
                {
                    continue;
                }

                body.muscles.push(Muscle::new_random(from, to));
                break;
            }
        }
        body
    }

    pub fn draw(&mut self)
    {
        self.muscles.iter_mut().for_each(|m| m.draw(self.pos, self.circles[m.from].pos, self.circles[m.to].pos));
        self.circles.iter_mut().for_each(|c| c.draw(self.pos));
    }
    pub fn update(&mut self, time : f32)
    {
        self.muscles.iter_mut().for_each(|m| m.update(time, &mut self.circles));
        self.circles.iter_mut().for_each(|c| c.update(self.pos));
    }

}

fn factorial(n : usize) -> usize
{
    (1..=n).product()
}