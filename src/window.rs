use macroquad::prelude::*;

use crate::next_test;

async fn test1() {
    while !next_test().await {
        clear_background(RED);
        draw_text("Press space for next test", 20.0, 20.0, 16., BLACK);
        next_frame().await;
    }
}

async fn test2() {
    set_fullscreen(true);
    while !next_test().await {
        clear_background(WHITE);
        draw_text("Should be fullscreen", 20.0, 20.0, 16., BLACK);
        next_frame().await;
    }

    set_fullscreen(false);
    macroquad::miniquad::window::schedule_update();
    next_frame().await;
    while !next_test().await {
        clear_background(WHITE);
        draw_text("Should be windowed again", 20.0, 20.0, 16., BLACK);
        next_frame().await;
    }
}

async fn test3() {
    request_new_screen_size(350., 50.);
    macroquad::miniquad::window::schedule_update();
    while !next_test().await {
        clear_background(WHITE);
        draw_text("Should be a 350, 50 window", 20.0, 20.0, 16., BLACK);
        next_frame().await;
    }
    request_new_screen_size(800., 600.);
    macroquad::miniquad::window::schedule_update();
    next_frame().await;
    while !next_test().await {
        clear_background(WHITE);
        draw_text("Should be a 800, 600 window", 20.0, 20.0, 16., BLACK);
        next_frame().await;
    }
}

async fn test4() {
    use macroquad::prelude::*;

    while !next_test().await {
        clear_background(DARKGRAY);

        draw_text("Should be a red square", 20.0, 20.0, 16., BLACK);

        {
            let image_data = Image::gen_image_color(50, 50, RED);
            let image = Texture2D::from_image(&image_data);
            draw_texture(&image, 20.0, 50.0, WHITE);
        }
        next_frame().await;
    }
}

pub async fn all() {
    test1().await;
    test2().await;
    test3().await;
    test4().await;
}
