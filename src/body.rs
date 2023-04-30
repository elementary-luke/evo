use crate::point::*;
use crate::circle::*;
use crate::muscle::*;
use macroquad::qrand as rand;
use std::cmp::min;

#[derive(Clone, Debug)]
pub struct Body
{
    pub start_avg_x : f32,
    pub pos : Point,
    pub circles : Vec<Circle>,
    pub muscles : Vec<Muscle>,
    pub distance : Option<f32>,
}

impl Body
{
    pub fn new() -> Body // new empty body
    {
        let circles : Vec<Circle> = Vec::new();
        let muscles : Vec<Muscle> = Vec::new();
        let body : Body = Body {pos : Point {x : 0.0, y : 0.0}, circles, muscles, start_avg_x : 0.0, distance : None};
        body
    }
    pub fn new_random(x_bound : f32, y_bound : f32) -> Body
    {
        rand::srand(macroquad::miniquad::date::now() as u64);
        let circles : Vec<Circle> = Vec::new();
        let muscles : Vec<Muscle> = Vec::new();
        let mut body : Body = Body {pos : Point {x : 0.0, y : 0.0}, circles, muscles, start_avg_x : 0.0, distance : None};
        
        for _ in 0..rand::gen_range(5, 6) //REVERT TO 2, 10
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

        body.start_avg_x = body.circles.iter().map(|c| c.pos.x).sum::<f32>() / body.circles.len() as f32;
        //add a couple more random muscles between random circles but no repeats
        let max_connections = factorial(body.circles.len()) / (2 * factorial(body.circles.len()-2));
        if body.muscles.len() >= max_connections // dont continue if no more possible muscle additions are possible
        {   
            return body;
        }
        for _ in 0..(rand::gen_range(0, (max_connections - body.muscles.len() + 1)) as usize)
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
    pub fn set_start_avg(&mut self)
    {
        self.start_avg_x = self.circles.iter().map(|c| c.pos.x).sum::<f32>() / self.circles.len() as f32;
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
    pub fn get_average_distance(&self) -> f32
    {
        self.circles.iter().map(|c| c.pos.x).sum::<f32>() / self.circles.len() as f32 - self.start_avg_x
    }
    pub fn simulate(&mut self) -> f32
    {
        let mut time = 0.0;
        loop
        {
            time += 1.0/60.0;
            self.update(time);
            if self.circles.iter().all(|c| c.on_floor)
            {
                return self.get_average_distance();
            }
            if time > 10.0
            {
                //TODO make absolute
                return self.get_average_distance();
            }
            
        }
        
    }
    pub fn mutate(&mut self)
    {
        if  rand::gen_range(1, 5) == 1
        {
            if rand::gen_range(0, 2) == 1
            {
                // add circle
                self.circles.push(Circle::new_random(Point {x : rand::gen_range(-200.0, 200.0), y : rand::gen_range(-100.0, 100.0)}));
                for _ in 1..rand::gen_range(1, 2)
                {
                    self.muscles.push( Muscle::new_random(self.circles.len() - 1, rand::gen_range(0, self.circles.len() - 1)) );
                }
            }
            else
            {
                //remove circle
                if self.circles.len() == 0
                {
                    return;
                }
                let index = rand::gen_range(0, self.circles.len() - 1);
                for i in 0..self.muscles.len()
                {
                    if self.muscles[i].from > index
                    {
                        self.muscles[i].from -= 1;
                    }
                    if self.muscles[i].to > index
                    {
                        self.muscles[i].to -= 1;
                    }
                }
                self.muscles.retain(|x| x.from != index && x.to != index);
                self.circles.remove(index);
            }
            self.set_start_avg();
        }
        else 
        {
            //mutate component
            if rand::gen_range(0, 2) == 1
            {
                //mutate muscle
                if self.muscles.len() == 0
                {
                    return;
                }
                let index = rand::gen_range(0, self.muscles.len() - 1);
                self.muscles[index].mutate();
            }
            else
            {
                //mutate circle
                if self.circles.len() == 0
                {
                    return;
                }
                let index = rand::gen_range(0, self.circles.len() - 1);
                self.circles[index].mutate();
            }
        }
    }


}

fn factorial(n : usize) -> usize
{
    (1..=n).product()
}