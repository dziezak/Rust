use macroquad::audio::{load_sound, play_sound_once};
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

struct AppState {
    asking: bool,
    progress: f32,
    see_final: bool,
}

impl AppState {
    fn new() -> AppState {
        Self{
            asking: true,
            progress: 0.0,
            see_final: false,
        }
    }
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

fn obsluguj_czasteczki(czasteczki: &mut Vec<Serduszko>, mouse_vec: Vec2, czy_zaakceptowano: bool) {
    if !czy_zaakceptowano && czasteczki.len() < 50 {
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

        if czy_zaakceptowano {
            s.vel.y += 0.05;
            s.rotation += 0.05;
        }

        s.life -= 0.01;
        s.pos += s.vel;

        if s.life <= 0.0 {
            czasteczki.remove(i);
        } else {
            let color = if czy_zaakceptowano {
                Color::new(rand::gen_range(0.8, 1.0), 0.2, rand::gen_range(0.4, 0.7), s.life.min(1.0))
            } else {
                Color::new(1.0, 0.4, 0.6, s.life)
            };

            draw_text_ex("<3", s.pos.x, s.pos.y, TextParams {
                font_size: s.size as u16,
                color,
                rotation: s.rotation,
                ..Default::default()
            });
        }
    }
}


fn stworz_konfetti(czasteczki: &mut Vec<Serduszko>) {
    if czasteczki.len() < 300 {
        czasteczki.push(Serduszko {
            pos: vec2(rand::gen_range(0.0, screen_width()), -20.0),
            vel: vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(2.0, 5.0)),
            life: 2.0,
            size: rand::gen_range(15.0, 35.0),
            rotation: rand::gen_range(0.0, 6.28),
        });
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let kotek_bytes = include_bytes!("kotek.png");
    let kotek_texture = Texture2D::from_file_with_format(kotek_bytes, Some(ImageFormat::Png));
    let mut czasteczki: Vec<Serduszko> = Vec::new();
    let mut state = AppState::new();
    // Wewnątrz main, przed loop
    let sound_boink = load_sound("boink.wav").await.expect("Nie znaleziono boink.wav");
    let sound_success = load_sound("success.wav").await.expect("Nie znaleziono success.wav");
    let sound_yippee = load_sound("yippee.wav").await.expect("Nie znaleziono success.wav");

    let mut nie_pos = vec2(450.0, 350.0);
    let mut nie_target = nie_pos;
    let mut nie_timer = 0.0;
    let mut show_kotek = false;
    let mut should_be_played = true;

    loop {
        clear_background(Color::from_rgba(255, 230, 235, 255));
        let (m_x, m_y) = mouse_position();
        let mouse_vec = vec2(m_x, m_y);
        let dt = get_frame_time();

        if state.asking {
            draw_text("Czy zostaniesz moja Walentynka?", 120.0, 200.0, 45.0, BLACK);

            let puls = (get_time() as f32 * 6.0).sin();
            let scale = 1.0 + puls * 0.1;

            let tak_w = 120.0 * scale;
            let tak_h = 60.0 * scale;
            let tak_x = 250.0 - (tak_w - 120.0) / 2.0;
            let tak_y = 350.0 - (tak_h - 60.0) / 2.0;

            let tak_rect = Rect::new(tak_x, tak_y, tak_w, tak_h);

            for i in 1..=3 {
                let glow_offset = i as f32 * 10.0 * scale;
                let alpha = 0.3 / (i as f32);

                draw_rectangle(
                    tak_rect.x - glow_offset / 2.0,
                    tak_rect.y - glow_offset / 2.0,
                    tak_rect.w + glow_offset,
                    tak_rect.h + glow_offset,
                    Color::new(1.0, 0.4, 0.7, alpha)
                );
            }

            draw_rectangle(tak_rect.x, tak_rect.y, tak_rect.w, tak_rect.h, PINK);
            draw_text("TAK!", tak_rect.x + (tak_w * 0.25), tak_rect.y + (tak_h * 0.65), 30.0 * scale, WHITE);

            let dystans = mouse_vec.distance(nie_pos + vec2(60.0, 30.0));
            if dystans < 100.0 && nie_timer <= 0.0 {
                show_kotek = true;
                nie_target.x = rand::gen_range(50.0, screen_width() - 150.0);
                nie_target.y = rand::gen_range(100.0, screen_height() - 100.0);
                nie_timer = 0.5;
                play_sound_once(&sound_boink);
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
            } else {
                show_kotek = false;
            }

            draw_rectangle(nie_pos.x, nie_pos.y, 120.0, 60.0, RED);
            draw_text("NIE", nie_pos.x + 35.0, nie_pos.y + 40.0, 30.0, WHITE);

            if show_kotek {
                draw_texture_ex(
                    &kotek_texture,
                    nie_pos.x + 10.0,
                    nie_pos.y + 70.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(100.0, 100.0)),
                        ..Default::default()
                    },
                );
            }

            if is_mouse_button_pressed(MouseButton::Left) && tak_rect.contains(mouse_vec) {
                state.asking = false;
                play_sound_once(&sound_success);
            }
        }
        else if !state.see_final {
            state.progress += 0.005;

            let bar_width = 400.0;
            let bar_height = 40.0;
            let bar_x = (screen_width() - bar_width) / 2.0;
            let bar_y = screen_height() / 2.0;

            draw_text("Wysylanie milosci...", bar_x, bar_y - 20.0, 30.0, BLACK);

            draw_rectangle(bar_x, bar_y, bar_width, bar_height, Color::new(0.8, 0.8, 0.8, 1.0));
            draw_rectangle(bar_x, bar_y, bar_width * state.progress, bar_height, RED);
            draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 3.0, BLACK);

            if state.progress >= 1.0 {
                state.see_final = true;
            }
        }
        else{

            if should_be_played {
                play_sound_once(&sound_yippee);
                should_be_played = false;
            }
            draw_text("Hura! <3", 320.0, 250.0, 60.0, RED);
            draw_text("Gratulacje, zostalas moja Walentynka!", 130.0, 350.0, 40.0, BLACK);

            stworz_konfetti(&mut czasteczki);
        }

        obsluguj_czasteczki(&mut czasteczki, mouse_vec, !state.asking);

        next_frame().await
    }
}
