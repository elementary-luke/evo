mod creations;
use crate::creations::*;
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
    


async fn main() {
    // could use hashmap to store all objects but probs dont need to store objects anyway
    let mut bodies : Vec<Body> = Vec::new();
    loop {
        clear_background(LIGHTGRAY);
        bodies.push(Body::new());
        bodies.into_iter().for_each(|i| i.draw());

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}