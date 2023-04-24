//use normal_rand as rand;
mod muscle;
mod circle;
mod point;
mod force;
mod body;
use crate::body::*;
use crate::point::*;
use crate::circle::*;
use crate::muscle::*;
use macroquad::color::*;
use macroquad::color_u8;
use macroquad::time::get_frame_time;
use macroquad::window::*;
use macroquad::text::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "My game".to_owned(),
        //fullscreen: true,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
    
}



#[macroquad::main(window_conf())]
async fn main() {
    let mut time : f32 = 0.0;
    let mut bodies : Vec<(Body, f32)> = Vec::new();
    //simulate(100, &mut bodies);
    bodies.push((Body::new_random(200.0 , 200.0), 10000.0));
    //testing(&mut bodies);
    bodies[0].0.pos = Point {x : screen_width()/2.0 - 100.0, y : screen_height()/2.0};
    bodies[0].0.set_start_avg();
    
    loop {
        time += get_frame_time();
        clear_background(color_u8!(	135.0, 206.0, 235.0, 1.0));
        bodies[0].0.update(time);
        bodies[0].0.draw();
        

        draw_text(&time.to_string(), 20.0, 20.0, 30.0, DARKGRAY);
        if time > 10.0
        {
            println!("{}", bodies[0].0.get_average_distance());
        }

        next_frame().await
    }

}
fn simulate(n : usize, bodies : &mut Vec<(Body, f32)>)
{
    for i in 0..n
    {
        bodies.push((Body::new_random(200.0 , 200.0), 10000.0));
        bodies[i].0.pos = Point {x : screen_width()/2.0 - 100.0, y : screen_height()/2.0};
        bodies[i].0.set_start_avg();
        let temp = bodies[i].0.clone();

        bodies[i].1 = bodies[i].0.simulate();
        bodies[i].0 = temp;
    }
    bodies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("{} {}", bodies[0].1, bodies[bodies.len() - 1].1);
}

fn testing(bodies : &mut Vec<(Body, f32)>)
{
    bodies.push((Body::new(), 100000.0));
    bodies[0].0.circles.push(Circle {
        pos : Point {x : -10.0, y : 0.0},
        r: 5.0, 
        color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
        slip: 0.0,
        velocity : Point {x : 0.0, y : 0.0},
        acceleration : Point {x : 0.0, y : 0.0},
        forces : vec![],
        on_floor : false,
    });
    bodies[0].0.circles.push(Circle {
        pos : Point {x : 10.0, y : 0.0},
        r: 5.0, 
        color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
        slip: 0.0,
        velocity : Point {x : 0.0, y : 0.0},
        acceleration : Point {x : 0.0, y : 0.0},
        forces : vec![],
        on_floor : false,
    });
    bodies[0].0.circles.push(Circle {
        pos : Point {x : 0.0, y : -10.0},
        r: 5.0, 
        color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
        slip: 0.5,
        velocity : Point {x : 0.0, y : 0.0},
        acceleration : Point {x : 0.0, y : 0.0},
        forces : vec![],
        on_floor : false,
    });
    bodies[0].0.muscles.push(Muscle {
        from : 0,
        to : 2,
        strength : 0.6,
        contracted_len : 80.0,
        extended_len : 160.0,
        contracted_time : 0.8,
        extended_time : 0.8,
        contracting : (false, 0.0),
    });
    bodies[0].0.muscles.push(Muscle {
        from : 1,
        to : 2,
        strength : 0.6,
        contracted_len : 80.0,
        extended_len : 160.0,
        contracted_time : 0.8,
        extended_time : 0.8,
        contracting : (false, 0.0),
    });
}