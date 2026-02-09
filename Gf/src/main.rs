//#![windows_subsystem = "windows"]
use macroquad::prelude::*;
use macroquad::prelude::ImageFormat;

struct Serduszko {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    size: f32,
    rotation: f32,
}

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

fn obsluguj_czasteczki(czasteczki: &mut Vec<Serduszko>, mouse_vec: Vec2) {
    if czasteczki.len() < 50 {
        czasteczki.push(Serduszko {
            pos: mouse_vec,
            vel: vec2(rand::gen_range(-1.5, 1.5), rand::gen_range(-1.5, 1.5)),
            life: 1.0,
            size: rand::gen_range(15.0, 30.0),
            rotation: rand::gen_range(0.0, 6.28),
        });
    }

    for i in (0..czasteczki.len()).rev() {
        let s = &mut czasteczki[i];
        s.life -= 0.02;
        s.pos += s.vel;

        if s.life <= 0.0 {
            czasteczki.remove(i);
        } else {
            let color = Color::new(1.0, 0.4, 0.6, s.life);
            draw_text_ex("<3", s.pos.x, s.pos.y, TextParams {
                font_size: s.size as u16,
                color,
                rotation: s.rotation,
                ..Default::default()
            });
        }
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let kotek_bytes = include_bytes!("kotek.png");
    let kotek_texture = Texture2D::from_file_with_format(kotek_bytes, Some(ImageFormat::Png));
    let mut czasteczki: Vec<Serduszko> = Vec::new();

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
                    nie_pos.x + 10.0,
                    nie_pos.y -110.0,
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

        obsluguj_czasteczki(&mut czasteczki, mouse_vec);

        next_frame().await
    }
}
