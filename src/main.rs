use macroquad::{prelude::*};

const MOVEMENT_SPEED: f32 = 200.0;
const RADIUS: f32 = 16.0;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

#[macroquad::main("Macro Game")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut squares: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: RADIUS,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    loop {

        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size: size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0), 
                y: -size,
            });
        }
        clear_background(DARKPURPLE);

        let delta_time = get_frame_time();
        
        if is_key_down(KeyCode::Right) {
            circle.x += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= circle.speed * delta_time;
        }

        circle.x = clamp(circle.x, 0.0 + RADIUS / 2.0, screen_width() - RADIUS / 2.0);
        circle.y = clamp(circle.y, 0.0 + RADIUS / 2.0, screen_height() - RADIUS / 2.0);
        draw_circle(circle.x, circle.y, circle.size, YELLOW);

        for square in &mut squares {
            square.y += square.speed * delta_time;
        }

        squares.retain(|square| square.y < screen_height() + square.size);

        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size, GREEN,
            );
        }

        next_frame().await;
    }
}
