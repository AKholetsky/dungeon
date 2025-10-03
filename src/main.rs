use macroquad::{color::RED, window::{clear_background, next_frame}};


#[macroquad::main("dungeon")]
async fn main() {
    loop {
        clear_background(RED);
        next_frame().await
    }
}
