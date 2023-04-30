//use normal_rand as rand;
mod muscle;
mod circle;
mod point;
mod force;
mod body;
mod settings;
use crate::body::*;
use crate::point::*;
use crate::circle::*;
use crate::muscle::*;
use crate::settings::*;
use macroquad::color::*;
use macroquad::color_u8;
use macroquad::window::*;
use macroquad::text::*;
use macroquad::qrand as rand;
use macroquad::ui::*;
use macroquad::ui::widgets::Button;
use macroquad::math::vec2;
use std::f32::NAN;


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
    rand::srand(macroquad::miniquad::date::now() as u64);
    let mut time : f32 = 0.0;
    let mut bodies : Vec<Body> = Vec::new();

    bodies.push(Body::new_random(Settings::X_BOUND, Settings::Y_BOUND));
    create(&mut bodies, 100);
    simulate(&mut bodies);
    for _ in 0..10
    {
        kill(&mut bodies);
        repopulate(&mut bodies);
        simulate(&mut bodies);
    }

    // testing(&mut bodies);

    //bodies.push(Body::new_random(Settings:X_BOUND, Settings:Y_BOUND));
    //bodies.push(Body::new());
    //testing(&mut bodies);
    bodies[0].pos = Point {x : screen_width()/2.0 - 100.0, y : screen_height()/2.0 - 150.0};
    bodies[0].set_start_avg();
    println!("{}", bodies[0].distance.unwrap());
    
    loop {
        //time += get_frame_time();
        time += 1.0/60.0;

        clear_background(color_u8!(	135.0, 206.0, 235.0, 1.0));
        bodies[0].update(time);
        bodies[0].draw();

        let next_gen = Button::new("Next Gen")
            .position(vec2(40.0, 50.0))
            .size(vec2(40.0, 50.0))
            .ui(&mut root_ui())
            ;
        draw_text(&time.to_string(), 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(&bodies[0].get_average_distance().to_string(), 20.0, 40.0, 30.0, DARKGRAY);
        next_frame().await
    }

}
fn create (bodies : &mut Vec<Body>, n : usize)
{
    for _ in 0..n
    {
        bodies.push(Body::new_random(Settings::X_BOUND, Settings::Y_BOUND));
    }
}
fn simulate(bodies : &mut Vec<Body>)
{
    for i in 0..bodies.len()
    {
        if bodies[i].distance.is_some()
        {
            continue;
        }
        bodies[i].pos = Point {x : screen_width()/2.0 - 100.0, y : screen_height()/2.0 - 150.0};
        bodies[i].set_start_avg();
        let temp = bodies[i].clone();

        let dist = bodies[i].simulate();
        bodies[i] = temp;

        //if it went backwards flip it
        if dist < 0.0
        {
            bodies[i].flip();
        }
        bodies[i].distance = Some(dist.abs());
    }
    bodies.retain(|b| !b.distance.unwrap().is_nan());
    bodies.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
}

fn kill(bodies : &mut Vec<Body>)
{
    let mut to_kill : Vec<usize> = Vec::new();
    for i in 0..(bodies.len() / 2)
    {
        let f : f32 = i as f32 / bodies.len() as f32;
        let r : f32 = (rand::gen_range(-1.0 as f32, 1.0 as f32).powf(3.0) + 1.0) / 2.0;
        if f > r
        {
            to_kill.push(i);
        }
        else 
        {
            to_kill.push(bodies.len() - 1 - i);
        }
    }
    
    to_kill.sort();
    to_kill.reverse();
    for i in to_kill
    {
        bodies.remove(i);
    }
}

fn repopulate(bodies : &mut Vec<Body>)
{
    let mut new_bodies : Vec<Body> = Vec::new();
    for i in 0..bodies.len()
    {
        let mut new_body = bodies[i].clone();
        new_body.distance = None;
        new_body.mutate();
        new_bodies.push(new_body);
    }
    bodies.append(&mut new_bodies);
}

//triangler
// fn testing(bodies : &mut Vec<Body>)
// {
//     bodies.push(Body::new());
//     bodies[0].circles.push(Circle {
//         pos : Point {x : -10.0, y : 0.0},
//         r: 5.0, 
//         color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
//         slip: 0.5,
//         velocity : Point {x : 0.0, y : 0.0},
//         acceleration : Point {x : 0.0, y : 0.0},
//         forces : vec![],
//         on_floor : false,
//     });
//     bodies[0].circles.push(Circle {
//         pos : Point {x : 10.0, y : 0.0},
//         r: 5.0, 
//         color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
//         slip: 0.5,
//         velocity : Point {x : 0.0, y : 0.0},
//         acceleration : Point {x : 0.0, y : 0.0},
//         forces : vec![],
//         on_floor : false,
//     });
//     bodies[0].circles.push(Circle {
//         pos : Point {x : 20.0, y : -10.0},
//         r: 5.0, 
//         color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
//         slip: 0.5,
//         velocity : Point {x : 0.0, y : 0.0},
//         acceleration : Point {x : 0.0, y : 0.0},
//         forces : vec![],
//         on_floor : false,
//     });
//     bodies[0].muscles.push(Muscle {
//         from : 0,
//         to : 2,
//         strength : 0.6,
//         contracted_len : 80.0,
//         extended_len : 160.0,
//         contracted_time : 0.8, //revert to 0.8
//         extended_time : 0.4,
//         contracting : (true, 0.0),
//     });
//     bodies[0].muscles.push(Muscle {
//         from : 1,
//         to : 2,
//         strength : 0.6,
//         contracted_len : 80.0,
//         extended_len : 160.0,
//         contracted_time : 0.4,
//         extended_time : 0.4,
//         contracting : (true, 0.0),
//     });
//     bodies[0].muscles.push(Muscle {
//         from : 0,
//         to : 1,
//         strength : 0.6,
//         contracted_len : 80.0,
//         extended_len : 160.0,
//         contracted_time : 0.4,
//         extended_time : 0.4,
//         contracting : (true, 0.0),
//     });
// }

//square
fn testing(bodies : &mut Vec<Body>)
{
    let mut body = Body::new();
    body.circles.push(Circle {
        pos : Point {x : -10.0, y : 10.0},
        r: 5.0, 
        color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
        slip: 0.5,
        velocity : Point {x : 0.0, y : 0.0},
        acceleration : Point {x : 0.0, y : 0.0},
        forces : vec![],
        on_floor : false,
    });
    body.circles.push(Circle {
        pos : Point {x : 10.0, y : 10.0},
        r: 5.0, 
        color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
        slip: 0.5,
        velocity : Point {x : 0.0, y : 0.0},
        acceleration : Point {x : 0.0, y : 0.0},
        forces : vec![],
        on_floor : false,
    });
    body.circles.push(Circle {
        pos : Point {x : 10.0, y : -10.0},
        r: 5.0, 
        color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
        slip: 0.5,
        velocity : Point {x : 0.0, y : 0.0},
        acceleration : Point {x : 0.0, y : 0.0},
        forces : vec![],
        on_floor : false,
    });
    body.circles.push(Circle {
        pos : Point {x : -10.0, y : -10.0},
        r: 5.0, 
        color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
        slip: 0.5,
        velocity : Point {x : 0.0, y : 0.0},
        acceleration : Point {x : 0.0, y : 0.0},
        forces : vec![],
        on_floor : false,
    });
    body.muscles.push(Muscle {
        from : 0,
        to : 1,
        strength : 0.6,
        contracted_len : 80.0,
        extended_len : 160.0,
        contracted_time : 20.0,
        extended_time : 0.4,
        contracting : (true, 0.0),
    });
    body.muscles.push(Muscle {
        from : 1,
        to : 2,
        strength : 0.6,
        contracted_len : 80.0,
        extended_len : 160.0,
        contracted_time : 20.0,
        extended_time : 0.4,
        contracting : (true, 0.0),
    });
    body.muscles.push(Muscle {
        from : 2,
        to : 3,
        strength : 0.6,
        contracted_len : 80.0,
        extended_len : 160.0,
        contracted_time : 20.0,
        extended_time : 0.4,
        contracting : (true, 0.0),
    });
    body.muscles.push(Muscle {
        from : 3,
        to : 0,
        strength : 0.6,
        contracted_len : 80.0,
        extended_len : 160.0,
        contracted_time : 20.0,
        extended_time : 0.4,
        contracting : (true, 0.0),
    });
    bodies.push(body);
}
