use macroquad::prelude::*;
use macroquad::color::colors;

#[macroquad::main("InputKeys")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let mut circle_color = WHITE;

    loop {
        clear_background(LIGHTGRAY);

        if is_key_down(KeyCode::Right) {
            x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            x -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 1.0;
        }
        if is_key_down(KeyCode::R) {
            circle_color = RED;
        }
        if is_key_down(KeyCode::G) {
            circle_color = GREEN;
        }
        if is_key_down(KeyCode::B) {
            circle_color = BLUE;
        }
        if is_key_down(KeyCode::Q) {
            // stop loop
            break;
        }

        draw_circle(x, y, 15.0, circle_color);
        draw_text("Move the ball with arrow keys. Press Q to exit.", 20.0, 20.0, 20.0, DARKGRAY);
        println!("frame x: {} y: {}", x, y);
        next_frame().await
     
    }
}