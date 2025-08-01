use macroquad::{prelude::*};
use std::fs;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

impl Shape {
    fn collide_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0, 
            w: self.size, 
            h: self.size, 
        }
    }
}

#[macroquad::main("Macro Game")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    const RADIUS: f32 = 16.0;
    rand::srand(miniquad::date::now() as u64);

    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);
    let mut game_state = GameState::MainMenu;
    let mut squares: Vec<Shape> = vec![];
    let mut bullets: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: RADIUS,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    };

    loop {

        clear_background(DARKPURPLE);

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape){
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space){
                    squares.clear();
                    bullets.clear();
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    score = 0;
                    game_state = GameState::Playing;
                }
                let text = "Press space";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE);
            },
            GameState::Playing => {
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

                if is_key_pressed(KeyCode::Space) {
                    bullets.push(Shape { 
                            size: 5.0, 
                            speed: circle.speed * 2.0, 
                            x: circle.x, 
                            y: circle.y, 
                            collided: false
                        });
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                circle.x = clamp(circle.x, 0.0 + RADIUS / 2.0, screen_width() - RADIUS / 2.0);
                circle.y = clamp(circle.y, 0.0 + RADIUS / 2.0, screen_height() - RADIUS / 2.0);


                if rand::gen_range(0, 99) >= 95 {
                    let size = rand::gen_range(16.0, 64.0);
                    squares.push(Shape {
                        size: size,
                        speed: rand::gen_range(50.0, 150.0),
                        x: rand::gen_range(size / 2.0, screen_width() - size / 2.0), 
                        y: -size,
                        collided: false,
                    });
                }

                for square in &mut squares {
                    square.y += square.speed * delta_time;
                }

                for bullet in &mut bullets {
                    bullet.y -= bullet.speed * delta_time;
                }

                squares.retain(|square| square.y < screen_height() + square.size);   
                bullets.retain(|bullet| bullet.y > 0.0 + bullet.size);
                squares.retain(|square| !square.collided);   
                bullets.retain(|bullet| !bullet.collided);   

                if squares.iter().any(|square| circle.collide_with(square)) {
                    if score == high_score {
                        fs::write("highscore.dat", high_score.to_string()).ok();
                    }
                    game_state = GameState::GameOver;
                }
                for square in squares.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if bullet.collide_with(square) {
                            bullet.collided = true;
                            square.collided = true;
                            score += square.size.round() as u32;
                            high_score = high_score.max(score);
                        }
                    }
                }
                for bullet in &bullets {
                    draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
                }
                draw_circle(circle.x, circle.y, circle.size, YELLOW);
                for square in &squares {
                    draw_rectangle(
                        square.x - square.size / 2.0,
                        square.y - square.size / 2.0,
                        square.size,
                        square.size, GREEN,
                    );
                }

                draw_text(
                    format!("Score: {}", score).as_str(),
                    10.0,
                    35.0,
                    25.0,
                    WHITE);
                let highscore_text = format!("High score: {}", high_score);
                let text_dimensions = measure_text(&highscore_text.as_str(), None, 25, 1.0);
                draw_text(
                    &highscore_text.as_str(), 
                    screen_width() - text_dimensions.width - 10.0, 
                    35.0, 
                    25.0, 
                    WHITE);
            },
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
                let text = "Paused";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE);
            },
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                let text = "Game Over!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE);
            },
        }
        
        // if gameover {
        //     let text = "GAME OVER";
        //     let text_dimensions = measure_text(text, None, 50, 1.0);
        //     draw_text(
        //         text,
        //         screen_width() / 2.0 - text_dimensions.width / 2.0,
        //         screen_height() / 2.0,
        //         50.0,
        //         RED);
        // }

        // if gameover && is_key_pressed(KeyCode::Space) {

        // }

        next_frame().await;
    }
}
