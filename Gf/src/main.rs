//#![windows_subsystem = "windows"]
use macroquad::prelude::*;
use macroquad::prelude::ImageFormat;

fn window_conf() -> Conf {
    Conf {
        window_title: "Niespodzianka!".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        fullscreen: false,
        sample_count: 1,
        high_dpi: false,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("OKNO SIE ZROBILO!!!");

    let kotek_bytes = include_bytes!("kotek.png");
    let kotek_texture = Texture2D::from_file_with_format(kotek_bytes, Some(ImageFormat::Png));
    println!("Kotek zaladowany");

    let mut nie_pos = vec2(450.0, 350.0);
    let mut nie_target = nie_pos;
    let mut nie_timer = 0.0;
    let mut zaakceptowano = false;
    let mut show_kotek = false;

    loop {
        clear_background(Color::from_rgba(255, 230, 235, 255));
        let (m_x, m_y) = mouse_position();
        let mouse_vec = vec2(m_x, m_y);
        let dt = get_frame_time();

        if !zaakceptowano {
            // Tekst główny
            draw_text("Czy zostaniesz moja Walentynka?", 120.0, 200.0, 45.0, BLACK);

            // Przycisk TAK
            let tak_rect = Rect::new(250.0, 350.0, 120.0, 60.0);
            draw_rectangle(tak_rect.x, tak_rect.y, tak_rect.w, tak_rect.h, GREEN);
            draw_text("TAK!", tak_rect.x + 30.0, tak_rect.y + 40.0, 30.0, WHITE);

            let dystans = mouse_vec.distance(nie_pos + vec2(60.0, 30.0));
            if dystans < 100.0 && nie_timer <= 0.0 {
                show_kotek = true;
                nie_target.x = rand::gen_range(50.0, screen_width() - 150.0);
                nie_target.y = rand::gen_range(100.0, screen_height() - 100.0);
                nie_timer = 0.5;
            } else {
                show_kotek = false;
            }

            if nie_timer > 0.0 {
                let speed = 600.0;
                let dir = nie_target - nie_pos;
                if dir.length() < speed * dt {
                    nie_pos = nie_target;
                    nie_timer = 0.0;
                } else {
                    nie_pos += dir.normalize() * speed * dt;
                    nie_timer -= dt;
                }
            }

            draw_rectangle(nie_pos.x, nie_pos.y, 120.0, 60.0, RED);
            draw_text("Nie", nie_pos.x + 35.0, nie_pos.y + 40.0, 30.0, WHITE);

            if show_kotek {
                draw_texture_ex(
                    &kotek_texture,
                    m_x - 50.0,
                    m_y - 50.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(100.0, 100.0)),
                        ..Default::default()
                    },
                );
            }

            if is_mouse_button_pressed(MouseButton::Left) && tak_rect.contains(mouse_vec) {
                zaakceptowano = true;
            }
        } else {
            draw_text("Hura! <3", 320.0, 250.0, 60.0, RED);
            draw_text("Gratulacje, zostalas moja Walentynka!", 130.0, 350.0, 40.0, BLACK);
        }

        next_frame().await
    }
}
