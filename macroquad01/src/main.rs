use macroquad::prelude::*;

#[macroquad::main("InputKeys")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

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
        if is_key_down(KeyCode::Q) {
            // stop loop
            break;
        }

        draw_circle(x, y, 15.0, YELLOW);
        draw_text("Move the ball with arrow keys. Press Q to exit.", 20.0, 20.0, 20.0, DARKGRAY);
        println!("frame x: {} y: {}", x, y);
        next_frame().await
     
    }
}