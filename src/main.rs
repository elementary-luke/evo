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
use macroquad::prelude::Rect;
use macroquad::ui::widgets::Button;
use macroquad::ui::widgets::Label;
use macroquad::window::*;
use macroquad::text::*;
use macroquad::qrand as rand;
use macroquad::ui::*;
use macroquad::math::vec2;
use macroquad::camera::*;


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
    let mut gen : i64 = 1;
    let mut show : i64 = 0;


    create(&mut bodies, 150);
    simulate(&mut bodies);
    // for _ in 0..1 // quite stable number of gens
    // {
    //     kill(&mut bodies);
    //     repopulate(&mut bodies);
    //     simulate(&mut bodies);
        
    // }
    bodies[0].pos = Point {x : screen_width()/2.0 - 100.0, y : screen_height()/2.0 - 150.0};
    bodies[0].set_start_avg();


    

    let mut rbodies = vec![bodies[0].clone()];
    
    loop {
        let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));
        time += 1.0/60.0;

        clear_background(color_u8!(	135.0, 206.0, 235.0, 1.0));

        for i in 0..rbodies.len()
        {
            rbodies[i].update(time);
            rbodies[i].draw();
            if rbodies.len() > 1
            {
                rbodies[i].set_alpha(i as f32/(bodies.len() - 1) as f32);
            }
            else 
            {
                rbodies[0].set_alpha(0.75);
            }
        }
        
        let next_gen_b = Button::new("Next Gen").position(vec2(screen_width() - 100.0, 20.0)).ui(&mut root_ui());
        let next_10_gen_b = Button::new("Next 10 Gen").position(vec2(screen_width() - 100.0, 50.0)).ui(&mut root_ui());
        let show_text : String;

        match show
        {
            0 => show_text = "Showing Best".to_string(),
            1 => show_text = "Showing Medium".to_string(),
            2 => show_text = "Showing Worst".to_string(),
            3 => show_text = "Showing All".to_string(),
            _ => show_text = "ERR".to_string(),
        }

        let show_all_b = Button::new(show_text).position(vec2(screen_width() - 100.0, 80.0)).ui(&mut root_ui());
        
        if show_all_b
        {
            time = 0.0;
            show += 1;
            if show >= 4
            {
                show = 0;
            }
            
            match show
            {
                0 => {
                    rbodies = vec![bodies[0].clone()]
                },
                1 => {
                    rbodies = vec![bodies[bodies.len() / 2].clone()]
                },
                2 => {
                    rbodies = vec![bodies[bodies.len() - 1].clone()]
                },
                3 => {
                    rbodies = bodies.clone();
                },
                _ => println!("ERR RBDOIES SET"),
            }

        }

        if next_10_gen_b
        {
            for _ in 0..10
            {
                kill(&mut bodies);
                repopulate(&mut bodies);
                simulate(&mut bodies);
            }
            time = 0.0;
            gen += 10;
            match show
            {
                0 => {
                    rbodies = vec![bodies[0].clone()]
                },
                1 => {
                    rbodies = vec![bodies[bodies.len() / 2].clone()]
                },
                2 => {
                    rbodies = vec![bodies[bodies.len() - 1].clone()]
                },
                3 => {
                    rbodies = bodies.clone();
                },
                _ => println!("ERR RBDOIES SET"),
            }
        }

        if next_gen_b
        {
            kill(&mut bodies);
            repopulate(&mut bodies);
            simulate(&mut bodies);
            time = 0.0;
            gen += 1;
            match show
            {
                0 => {
                    rbodies = vec![bodies[0].clone()]
                },
                1 => {
                    rbodies = vec![bodies[bodies.len() / 2].clone()]
                },
                2 => {
                    rbodies = vec![bodies[bodies.len() - 1].clone()]
                },
                3 => {
                    rbodies = bodies.clone();
                },
                _ => println!("ERR RBDOIES SET"),
            }
        }
        
        Label::new("Time ".to_string() + &time.to_string())
            .position(vec2(20.0, 10.0)).ui(&mut root_ui());

        Label::new("Distance ".to_string() + &rbodies[0].get_average_distance().to_string())
            .position(vec2(20.0, 30.0)).ui(&mut root_ui());
        
        Label::new("Gen ".to_string() + &gen.to_string())
            .position(vec2(20.0, 50.0)).ui(&mut root_ui());

        Label::new("Best dist ".to_string() + &bodies[0].distance.unwrap().to_string())
            .position(vec2(20.0, 70.0)).ui(&mut root_ui());

        Label::new("Avg dist ".to_string() + &(bodies.iter().map(|i| i.distance.unwrap()).sum::<f32>() / bodies.len() as f32).to_string())
            .position(vec2(20.0, 90.0)).ui(&mut root_ui());

        cam.target.x = rbodies[0].pos.x + rbodies[0].get_average_distance();
        set_camera(&cam);

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
// fn testing(bodies : &mut Vec<Body>)
// {
//     let mut body = Body::new();
//     body.circles.push(Circle {
//         pos : Point {x : -10.0, y : 10.0},
//         r: 5.0, 
//         color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
//         slip: 0.9,
//         velocity : Point {x : 0.0, y : 0.0},
//         acceleration : Point {x : 0.0, y : 0.0},
//         forces : vec![],
//         on_floor : false,
//     });
//     body.circles.push(Circle {
//         pos : Point {x : 10.0, y : 10.0},
//         r: 5.0, 
//         color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
//         slip: 0.9,
//         velocity : Point {x : 0.0, y : 0.0},
//         acceleration : Point {x : 0.0, y : 0.0},
//         forces : vec![],
//         on_floor : false,
//     });
//     body.circles.push(Circle {
//         pos : Point {x : 10.0, y : -10.0},
//         r: 5.0, 
//         color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
//         slip: 0.9,
//         velocity : Point {x : 0.0, y : 0.0},
//         acceleration : Point {x : 0.0, y : 0.0},
//         forces : vec![],
//         on_floor : false,
//     });
//     body.circles.push(Circle {
//         pos : Point {x : -10.0, y : -10.0},
//         r: 5.0, 
//         color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
//         slip: 0.9,
//         velocity : Point {x : 0.0, y : 0.0},
//         acceleration : Point {x : 0.0, y : 0.0},
//         forces : vec![],
//         on_floor : false,
//     });
//     body.muscles.push(Muscle {
//         from : 0,
//         to : 1,
//         strength : 5.0,
//         contracted_len : 80.0,
//         extended_len : 160.0,
//         contracted_time : 3.0,
//         extended_time : 0.4,
//         contracting : (true, 0.0),
//     });
//     body.muscles.push(Muscle {
//         from : 1,
//         to : 2,
//         strength : 5.0,
//         contracted_len : 80.0,
//         extended_len : 160.0,
//         contracted_time : 3.0,
//         extended_time : 0.4,
//         contracting : (true, 0.0),
//     });
//     body.muscles.push(Muscle {
//         from : 2,
//         to : 3,
//         strength : 5.0,
//         contracted_len : 80.0,
//         extended_len : 160.0,
//         contracted_time : 3.0,
//         extended_time : 0.4,
//         contracting : (true, 0.0),
//     });
//     body.muscles.push(Muscle {
//         from : 3,
//         to : 0,
//         strength : 5.0,
//         contracted_len : 80.0,
//         extended_len : 160.0,
//         contracted_time : 3.0,
//         extended_time : 0.4,
//         contracting : (true, 0.0),
//     });
//     bodies.push(body);
// }

// fn testing(bodies : &mut Vec<Body>)
// {
//     let mut body = Body::new();
//     body.circles.push(Circle {
//         pos : Point {x : 0.0, y : 0.0},
//         r: 5.0, 
//         color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
//         slip: 0.5,
//         velocity : Point {x : 0.0, y : 0.0},
//         acceleration : Point {x : 0.0, y : 0.0},
//         forces : vec![],
//         on_floor : false,
//     });
//     body.circles.push(Circle {
//         pos : Point {x : 100.0, y : 100.0},
//         r: 5.0, 
//         color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
//         slip: 0.5,
//         velocity : Point {x : 0.0, y : 0.0},
//         acceleration : Point {x : 0.0, y : 0.0},
//         forces : vec![],
//         on_floor : false,
//     });
//     body.circles.push(Circle {
//         pos : Point {x : 100.0, y : -100.0},
//         r: 5.0, 
//         color: Color { r: 1.0, g: 1.0, b: 1.0, a : 1.0}, 
//         slip: 0.5,
//         velocity : Point {x : 0.0, y : 0.0},
//         acceleration : Point {x : 0.0, y : 0.0},
//         forces : vec![],
//         on_floor : false,
//     });
//     body.muscles.push(Muscle {
//         from : 0,
//         to : 1,
//         strength : 10.0,
//         contracted_len : 20.0,
//         extended_len : 80.0,
//         contracted_time : 2.0,
//         extended_time : 2.0,
//         contracting : (false, 0.0),
//     });
//     body.muscles.push(Muscle {
//         from : 1,
//         to : 2,
//         strength : 10.0,
//         contracted_len : 20.0,
//         extended_len : 80.0,
//         contracted_time : 2.0,
//         extended_time : 2.0,
//         contracting : (false, 0.0),
//     });
//     bodies.push(body);
// }