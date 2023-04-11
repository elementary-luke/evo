mod creations;
use crate::creations::*;
use macroquad::color::*;
use macroquad::color_u8;
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
    // could use hashmap to store all objects but probs dont need to store objects anyway
    let mut bodies : Vec<Body> = Vec::new();
    bodies.push(Body::new_random(200.0 , 200.0));
    bodies[0].x = screen_width()/2.0;
    bodies[0].y = screen_height()/2.0;
    println!("{}", bodies[0].circles.len());
        for i in 0..bodies[0].muscles.len()
        {
            println!("{}, {}", bodies[0].muscles[i].from, bodies[0].muscles[i].to);
        }
    loop {
        clear_background(color_u8!(	135.0, 206.0, 235.0, 1.0));
        

        for i in 0..bodies.len()
        {
            bodies[i].draw();
        }
        

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}