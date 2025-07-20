use macroquad::prelude::*;

#[macroquad::main("Macro Game")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await;
    }
}
