use macroquad::prelude::*;

use crate::next_test;

async fn test1() {
    next_frame().await;
    clear_background(WHITE);
    draw_text("Should be 3 red squares", 20.0, 20.0, 16., BLACK);
    let image_data = Image::gen_image_color(50, 50, RED);
    let texture = Texture2D::from_image(&image_data);
    macroquad::texture::build_textures_atlas();
    draw_texture_ex(
        &texture,
        0.0,
        100.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(100., 100.0)),
            ..Default::default()
        },
    );
    let image_data = Image::gen_image_color(1050, 1050, RED);
    let texture2 = Texture2D::from_image(&image_data);
    draw_texture_ex(
        &texture2,
        110.0,
        100.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(100., 100.0)),
            ..Default::default()
        },
    );
    macroquad::texture::build_textures_atlas();
    draw_texture_ex(
        &texture,
        220.0,
        100.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(100., 100.0)),
            ..Default::default()
        },
    );
    next_frame().await;

    unsafe {
        macroquad::texture::reset_textures_atlas();
    }
}

async fn test2() {
    let font = load_ttf_font("assets/DancingScriptRegular.ttf")
        .await
        .unwrap();

    while !next_test().await {
        const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let text =
            CHARSET
            .chars()
            .nth(rand::gen_range(0i32, CHARSET.len() as i32) as usize)
            .unwrap();
        let font_size = rand::gen_range(1i32, 100i32) as u16;
        draw_text_ex(
            &text.to_string(),
            screen_width() / 4.0 * 2.0,
            screen_height() / 3.0 * 2.0,
            TextParams {
                font_size,
                font: Some(&font),
                rotation: 0.0,
                ..Default::default()
            },
        );
        next_frame().await;
    }
}
pub async fn all() {
    //test1().await;
    test2().await;
}
