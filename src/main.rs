use macroquad::prelude::*;

mod atlas;
mod window;

pub async fn next_test() -> bool {
    macroquad::miniquad::window::schedule_update();

    if is_key_pressed(KeyCode::Space) {
        next_frame().await;
        return true;
    }
    return false;
}

fn window_conf() -> macroquad::conf::Conf {
    use macroquad::miniquad::conf;

    let mut config = miniquad::conf::Conf {
        window_title: "Window Conf".to_owned(),
        ..Default::default()
    };
    let metal = std::env::args().nth(1).as_deref() == Some("metal");
    config.platform.apple_gfx_api = if metal {
        conf::AppleGfxApi::Metal
    } else {
        conf::AppleGfxApi::OpenGl
    };
    let egl = std::env::args().nth(1).as_deref() == Some("egl");
    config.platform.linux_x11_gl = if egl {
        conf::LinuxX11Gl::EGLOnly
    } else {
        conf::LinuxX11Gl::GLXOnly
    };
    config.platform.blocking_event_loop = true;

    macroquad::conf::Conf {
        miniquad_conf: config,
        update_on: Some(macroquad::conf::UpdateTrigger {
            mouse_down: true,
            specific_key: Some(vec![KeyCode::Space]),
            ..Default::default()
        }),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //window::all().await;
    atlas::all().await;
}
