//use normal_rand as rand;
mod muscle;
mod circle;
mod point;
mod force;
mod body;
use crate::body::*;
use crate::point::*;
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
    let mut bodies : Vec<Body> = Vec::new();
    bodies.push(Body::new_random(200.0 , 200.0));
    bodies[0].pos = Point {x : screen_width()/2.0, y : screen_height()/2.0};
    println!("circles: {}", bodies[0].circles.len());

    println!("muscles: {}", bodies[0].muscles.len());
    for i in 0..bodies[0].muscles.len()
    {
        println!("{}, {}", bodies[0].muscles[i].from, bodies[0].muscles[i].to);
    }
    loop {
        time += get_frame_time();
        clear_background(color_u8!(	135.0, 206.0, 235.0, 1.0));
        

        for i in 0..bodies.len()
        {
            bodies[i].update(time);
            bodies[i].draw();
        }
        

        draw_text(&time.to_string(), 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }

}