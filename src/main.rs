use std::time::Duration;

use glam::Mat3;
use glam::Vec2;
use glam::Vec4;
use marmalade::audio;
use marmalade::console;
use marmalade::input::Key;
use marmalade::render::canvas2d::TextureRect;

use crate::assets::Assets;
use crate::world::World;
use marmalade::dom_stack;
use marmalade::draw_scheduler;
use marmalade::input;
use marmalade::input::Button;
use marmalade::loading;
use marmalade::render::canvas2d::Canvas2d;
use marmalade::render::canvas2d::DrawTarget2d;
use marmalade::render::color;
use marmalade::tick_scheduler::TickScheduler;

mod assets;
mod world;

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const WORLD_SIZE: Vec2 = Vec2::new(16., 9.);

pub const SHREDDER_POS: Vec2 = Vec2::new(-15., -9.);
pub const SHREDDER_SIZE: Vec2 = Vec2::new(4., 4.);
pub const WHEEL_SIZE: Vec2 = Vec2::new(3., 3.);

const TARGET_PRESS: i32 = 10;
const VALUE_MIN: i8 = 2;

pub const VIEW_POS: [Vec2; 5] = [
    Vec2::new(-16., -9.),
    Vec2::new(-16., -9.),
    Vec2::new(-150., -85.),
    Vec2::new(-1950., -490.),
    Vec2::new(-8000., -8000.),
];

pub const VIEW_SIZE: [Vec2; 5] = [
    Vec2::new(32., 18.),
    Vec2::new(32., 18.),
    Vec2::new(195., 110.),
    Vec2::new(2240., 1260.),
    Vec2::new(32000., 18000.),
];

pub enum WorldState {
    EMAIL,
    PLAY,
    MOVING,
    END,
    FINAL,
    BYE,
}

fn draw_shredder(
    world: &World,
    canvas: &mut Canvas2d,
    assets: &Assets,
    text: &TextureRect,
    pos: Vec2,
) {
    canvas.camera_view_ratio(Vec2::new(0.0, 0.0), 16., ASPECT_RATIO);

    canvas.draw_rect(pos, SHREDDER_SIZE, color::WHITE, text);

    let wheel_pos_1 = Vec2::new(
        pos.x - WHEEL_SIZE.x / 2. + 0.5,
        pos.y + WHEEL_SIZE.y / 2. + 2.,
    );

    let wheel_pos_2 = Vec2::new(
        pos.x + WHEEL_SIZE.x / 2. + 0.5,
        pos.y + WHEEL_SIZE.y / 2. + 2.,
    );

    let wheel_pos_3 = Vec2::new(
        pos.x + SHREDDER_SIZE.x / 2. - WHEEL_SIZE.x / 2.,
        pos.y + SHREDDER_SIZE.y / 2. - WHEEL_SIZE.y / 2. + 1.5,
    );

    draw_wheel(
        canvas,
        wheel_pos_1,
        ((world.scraper.current_shredding_tick % 5) as f32) * 72.,
        assets,
    );
    draw_wheel(
        canvas,
        wheel_pos_2,
        -((world.scraper.current_shredding_tick % 5) as f32) * 72.,
        assets,
    );
    draw_wheel(
        canvas,
        wheel_pos_3,
        -((world.scraper.current_shredding_tick % 5) as f32) * 72.,
        assets,
    );

    if world.scraper.lubrication <= VALUE_MIN {
        canvas.draw_rect(
            Vec2::new(pos.x + 0.5, pos.y + 0.25),
            Vec2::new(1., 1.),
            color::WHITE,
            &assets.oil,
        );
    }

    if world.scraper.energy <= VALUE_MIN {
        canvas.draw_rect(
            Vec2::new(pos.x + 1.5, pos.y + 0.25),
            Vec2::new(1., 1.),
            color::WHITE,
            &assets.energy,
        );
    }

    if world.scraper.sharpening <= VALUE_MIN {
        canvas.draw_rect(
            Vec2::new(pos.x + 2.5, pos.y + 0.25),
            Vec2::new(1., 1.),
            color::WHITE,
            &assets.gears,
        );
    }
}

fn draw_game(canvas: &mut Canvas2d, world: &mut World, assets: &mut Assets) {
    canvas.camera_view_ratio(world.cam_pos, world.view_radius, ASPECT_RATIO);

    let mouse_pos = canvas.screen_to_world_pos(input::mouse_position().as_vec2());

    canvas.draw_rect(VIEW_POS[4], VIEW_SIZE[4], color::WHITE, &assets.l4);

    if world.get_stage() <= 3 {
        canvas.draw_rect(VIEW_POS[3], VIEW_SIZE[3], color::WHITE, &assets.l3);
    }

    if world.get_stage() <= 2 {
        canvas.draw_rect(VIEW_POS[2], VIEW_SIZE[2], color::WHITE, &assets.l2);
    }

    if world.get_stage() <= 1 {
        canvas.draw_rect(VIEW_POS[1], VIEW_SIZE[1], color::WHITE, &assets.l1);
    }

    if !input::is_button_down(Button::Left) {
        world.selected = None;
    }

    /*     if input::is_button_pressed(Button::Left) {
        console::log(&format!("x = {}, y = {}", mouse_pos.x, mouse_pos.y));
    } */

    let mouse_clicked = input::is_button_pressed(Button::Left);

    match world.state {
        WorldState::EMAIL => {
            canvas.draw_rect(
                Vec2::new(-8., -9.),
                Vec2::new(16., 18.),
                color::WHITE,
                &assets.gui_intro_email,
            );

            canvas.draw_rect(
                Vec2::new(0., -8.5),
                Vec2::new(6., 4.),
                color::WHITE,
                &assets.gui_intro_button_instructions,
            );

            if mouse_clicked
                && mouse_pos.x < 6.
                && mouse_pos.x > 0.
                && mouse_pos.y < -4.5
                && mouse_pos.y > -8.5
            {
                world.music_handle = Some(audio::play_loop(&mut assets.music_act[0], 1.));
                world.state = WorldState::PLAY;
            }
        }
        WorldState::PLAY => {
            for resource in &world.resources[world.get_stage()] {
                let r = resource.borrow();

                let radius = r.radius
                    * if r.movable && r.pos.distance(mouse_pos) < (r.radius / 2.) {
                        if mouse_clicked {
                            world.selected = Some(resource.clone());
                            audio::play(&mut assets.pickup_sound, 0.5);
                        }

                        if world.transition_running {
                            1.0
                        } else {
                            let color_circle: Vec4 = if r.energy > 0 {
                                color::rgba(1., 1., 0., 0.5)
                            } else if r.lubrication > 0 {
                                color::rgba(1., 0., 0., 0.4)
                            } else if r.sharpening > 0 {
                                color::rgba(0., 0., 1., 0.4)
                            } else {
                                color::rgba(0., 0., 0., 0.1)
                            };

                            canvas.draw_regular(
                                r.pos,
                                r.radius / 2.,
                                64,
                                color_circle,
                                &canvas.white_texture(),
                            );
                            1.1
                        }
                    } else {
                        1.0
                    };

                let size = Vec2::new(radius, radius);
                canvas.draw_rect(r.pos - size / 2., size, color::WHITE, &r.texture);
            }

            if let Some(selected) = &world.selected {
                if selected.borrow().movable {
                    let mut s = selected.borrow_mut();

                    let dist = mouse_pos - s.pos;

                    s.pos += dist * 0.1;

                    let view_pos = VIEW_POS[world.get_stage()];
                    let view_size = VIEW_SIZE[world.get_stage()];

                    if s.pos.x < view_pos.x {
                        s.pos.x = view_pos.x;
                    }
                    if s.pos.x > view_pos.x + view_size.x {
                        s.pos.x = view_pos.x + view_size.x;
                    }

                    if s.pos.y < view_pos.y {
                        s.pos.y = view_pos.y;
                    }
                    if s.pos.y > view_pos.y + view_size.y {
                        s.pos.y = view_pos.y + view_size.y;
                    }

                    let screen_pos = canvas.world_to_screen_pos(s.pos);

                    canvas.camera_view_ratio(Vec2::new(0.0, 0.0), 16., ASPECT_RATIO);

                    let interface_pos = canvas.screen_to_world_pos(screen_pos);

                    if world.scraper.current_ressource_shredded.is_none()
                        && world.scraper.waiting_key.is_none()
                    {
                        if interface_pos.distance(Vec2::new(
                            SHREDDER_POS.x + SHREDDER_SIZE.x / 2.,
                            SHREDDER_POS.y + SHREDDER_SIZE.y,
                        )) < 4.
                        {
                            let screen_pos = canvas.world_to_screen_pos(
                                SHREDDER_POS + Vec2::new(SHREDDER_SIZE.x / 2., SHREDDER_SIZE.y),
                            );

                            canvas.camera_view_ratio(
                                world.cam_pos,
                                world.view_radius,
                                ASPECT_RATIO,
                            );

                            let world_pos = canvas.screen_to_world_pos(screen_pos);

                            s.pos = world_pos;
                            drop(s);
                            world.scraper.shred(selected.clone());

                            Some(audio::play(&assets.shreder_sound, 0.2));
                        }
                    }
                } else {
                    world.selected = None;
                }
            }

            world.resources[world.get_stage()].retain(|x| x.borrow().alive);

            canvas.camera_view_ratio(Vec2::new(0.0, 0.0), 16., ASPECT_RATIO);

            let mut panel_text: &TextureRect = &assets.shredder_panel_ok;

            if let Some(key) = world.scraper.waiting_key {
                let pos = Vec2::new(SHREDDER_POS.x - 0.5, SHREDDER_POS.y + SHREDDER_SIZE.y + 3.0);
                let text = if world.scraper.key_down {
                    panel_text = &assets.shredder_panel_nok_1;
                    &assets.gui_key_down
                } else {
                    panel_text = &assets.shredder_panel_nok_2;
                    &assets.gui_key_up
                };

                canvas.draw_rect(pos, Vec2::new(4., 4.), color::WHITE, &text);

                canvas.draw_text(
                    pos + 1.75,
                    1.,
                    key.0,
                    &mut assets.font,
                    color::rgb(0., 0., 0.),
                    &canvas.white_texture(),
                );

                canvas.draw_rect(
                    Vec2::new(pos.x + 3.75, pos.y),
                    Vec2::new(
                        0.5,
                        world.scraper.current_press as f32 / TARGET_PRESS as f32 * 4.,
                    ),
                    color::rgb(0., 1., 0.),
                    &canvas.white_texture(),
                );

                canvas.draw_rect(
                    Vec2::new(pos.x + 3.5, pos.y),
                    Vec2::new(1., 4.),
                    color::rgb(0., 0., 0.),
                    &assets.gui_gauge,
                );
            }

            draw_shredder(world, canvas, assets, panel_text, SHREDDER_POS);

            canvas.draw_rect(
                Vec2::new(12., 6.),
                Vec2::new(4., 4.),
                color::WHITE,
                &assets.gui_time,
            );

            let timer = world.timer / 20; // Scale to reasonable speed

            let timer_minutes = timer % 60;
            let timer_hours = timer / 60 + 8; // start work at height
            canvas.draw_text(
                Vec2::new(14.0, 8.15),
                0.7,
                &format!("{timer_hours:02}h{timer_minutes:02}"),
                &mut assets.font,
                color::rgb(0., 0., 0.),
                &canvas.white_texture(),
            );
        }

        WorldState::MOVING => {
            canvas.camera_view_ratio(Vec2::new(0.0, 0.0), 16., ASPECT_RATIO);
            draw_shredder(
                world,
                canvas,
                assets,
                &assets.shredder_panel_ok,
                SHREDDER_POS + Vec2::new(0.1, 0.05) * world.end_tick as f32,
            );

            if world.end_tick > 120 {
                world.state = WorldState::FINAL;
            }
        }

        WorldState::FINAL => {
            canvas.camera_view_ratio(Vec2::new(0.0, 0.0), 16., ASPECT_RATIO);

            let mouse_pos = canvas.screen_to_world_pos(input::mouse_position().as_vec2());
            let pos = SHREDDER_POS + Vec2::new(12., 6.);
            let dist = mouse_pos.distance(Vec2::new(
                pos.x + SHREDDER_SIZE.x / 2.,
                pos.y + SHREDDER_SIZE.y,
            ));

            console::log(&format!("{dist}"));

            if dist < 1. {
                world.scraper.last_shred();
                world.state = WorldState::END;
            }
            draw_shredder(world, canvas, assets, &assets.shredder_panel_ok, pos);
        }

        WorldState::END => {
            canvas.camera_view_ratio(Vec2::new(0.0, 0.0), 16., ASPECT_RATIO);

            let pos = SHREDDER_POS + Vec2::new(12., 6.);
            draw_shredder(world, canvas, assets, &assets.shredder_panel_ok, pos);
            if !world.scraper.shredding_hand {
                world.state = WorldState::BYE;
            }
        }

        WorldState::BYE => {
            canvas.camera_view_ratio(Vec2::new(0.0, 0.0), 16., ASPECT_RATIO);

            canvas.draw_text(
                Vec2::new(-6., 0.),
                2.,
                &format!("Thank you for playing !"),
                &mut assets.font,
                color::WHITE,
                &canvas.white_texture(),
            );

            let timer = world.timer / 20; // Scale to reasonable speed

            let timer_minutes = timer % 60;
            let timer_hours = timer / 60 + 8;

            canvas.draw_text(
                Vec2::new(-2., -2.5),
                2.,
                &format!("Time : {timer_hours:2}h{timer_minutes}"),
                &mut assets.font,
                color::WHITE,
                &canvas.white_texture(),
            );
        }
    }

    world.resources[world.get_stage()].retain(|x| x.borrow().alive);

    canvas.camera_view_ratio(Vec2::new(0.0, 0.0), 16., ASPECT_RATIO);

    let mut panel_text: &TextureRect = &assets.shredder_panel_ok;

    if let Some(key) = world.scraper.waiting_key {
        let pos = Vec2::new(SHREDDER_POS.x - 0.5, SHREDDER_POS.y + SHREDDER_SIZE.y + 3.0);
        let text = if world.scraper.key_down {
            panel_text = &assets.shredder_panel_nok_1;
            &assets.gui_key_down
        } else {
            panel_text = &assets.shredder_panel_nok_2;
            &assets.gui_key_up
        };

        canvas.draw_rect(pos, Vec2::new(4., 4.), color::WHITE, &text);

        canvas.draw_text(
            pos + 1.75,
            1.,
            key.0,
            &mut assets.font,
            color::rgb(0., 0., 0.),
            &canvas.white_texture(),
        );

        canvas.draw_rect(
            Vec2::new(pos.x + 3.75, pos.y),
            Vec2::new(
                0.5,
                world.scraper.current_press as f32 / TARGET_PRESS as f32 * 4.,
            ),
            color::rgb(0., 1., 0.),
            &canvas.white_texture(),
        );

        canvas.draw_rect(
            Vec2::new(pos.x + 3.5, pos.y),
            Vec2::new(1., 4.),
            color::rgb(0., 0., 0.),
            &assets.gui_gauge,
        );
    }

    canvas.draw_rect(SHREDDER_POS, SHREDDER_SIZE, color::WHITE, &panel_text);

    let wheel_pos_1 = Vec2::new(
        SHREDDER_POS.x - WHEEL_SIZE.x / 2. + 0.5,
        SHREDDER_POS.y + WHEEL_SIZE.y / 2. + 2.,
    );

    let wheel_pos_2 = Vec2::new(
        SHREDDER_POS.x + WHEEL_SIZE.x / 2. + 0.5,
        SHREDDER_POS.y + WHEEL_SIZE.y / 2. + 2.,
    );

    let wheel_pos_3 = Vec2::new(
        SHREDDER_POS.x + SHREDDER_SIZE.x / 2. - WHEEL_SIZE.x / 2.,
        SHREDDER_POS.y + SHREDDER_SIZE.y / 2. - WHEEL_SIZE.y / 2. + 1.5,
    );

    draw_wheel(
        canvas,
        wheel_pos_1,
        ((world.scraper.current_shredding_tick % 5) as f32) * 72.,
        assets,
    );
    draw_wheel(
        canvas,
        wheel_pos_2,
        -((world.scraper.current_shredding_tick % 5) as f32) * 72.,
        assets,
    );
    draw_wheel(
        canvas,
        wheel_pos_3,
        -((world.scraper.current_shredding_tick % 5) as f32) * 72.,
        assets,
    );

    if world.scraper.lubrication <= VALUE_MIN {
        canvas.draw_rect(
            Vec2::new(SHREDDER_POS.x + 0.5, SHREDDER_POS.y + 0.25),
            Vec2::new(1., 1.),
            color::WHITE,
            &assets.oil,
        );
    }

    if world.scraper.energy <= VALUE_MIN {
        canvas.draw_rect(
            Vec2::new(SHREDDER_POS.x + 1.5, SHREDDER_POS.y + 0.25),
            Vec2::new(1., 1.),
            color::WHITE,
            &assets.energy,
        );
    }

    if world.scraper.sharpening <= VALUE_MIN {
        canvas.draw_rect(
            Vec2::new(SHREDDER_POS.x + 2.5, SHREDDER_POS.y + 0.25),
            Vec2::new(1., 1.),
            color::WHITE,
            &assets.gears,
        );
    }
}

fn draw_wheel(canvas: &mut Canvas2d, wheel_pos: Vec2, angle: f32, assets: &Assets) {
    let previous = canvas.get_view_matrix();

    let m1 = Mat3::from_translation(wheel_pos + Vec2::new(WHEEL_SIZE.x, WHEEL_SIZE.y) / 2.);
    let m2 = Mat3::from_rotation_z(angle.to_radians());
    let m3 = Mat3::from_translation(-wheel_pos - Vec2::new(WHEEL_SIZE.x, WHEEL_SIZE.y) / 2.);

    canvas.set_view_matrix(previous * m1 * m2 * m3);

    canvas.draw_rect(wheel_pos, WHEEL_SIZE, color::WHITE, &assets.shredder_wheel);

    canvas.set_view_matrix(previous);
}

async fn async_main() {
    dom_stack::set_title("UPSI 6");

    let main_canvas = dom_stack::create_full_screen_canvas();

    dom_stack::stack_node(&main_canvas);

    let mut canvas = Canvas2d::new(&main_canvas);

    loading::loading(&mut canvas, |_| async {}).await;

    let mut assets = Assets::load(&mut canvas).await;

    let mut world = World::new(&assets);

    let mut tick_scheduler = TickScheduler::new(Duration::from_secs_f64(1.0 / 60.0)); // 60 HZ
    draw_scheduler::set_on_draw(move || {
        for _ in 0..tick_scheduler.tick_count() {
            if world.resources[world.get_stage()].is_empty()
                && let WorldState::PLAY = world.state
            {
                if world.get_stage() != 4 {
                    world.next_stage(&assets);
                } else {
                    world.state = WorldState::MOVING;
                }
            }

            world.tick(&mut assets);

            canvas.fit_screen();

            canvas.clear(color::rgb(0., 0., 0.));

            draw_game(&mut canvas, &mut world, &mut assets);

            canvas.camera_view_ratio(Vec2::ZERO, 16., 16./9.);
            
            let mouse_texture = if input::is_button_down(Button::Left) {
                &assets.hand_close
            } else  {
                &assets.hand_open
            };
            
            canvas.draw_rect(Vec2::new(-16., -9.-36.), Vec2::new(64., 36.), color::rgb(0., 0.,0.), &canvas.white_texture());
            canvas.draw_rect(Vec2::new(-16., 9.), Vec2::new(64., 36.), color::rgb(0., 0.,0.), &canvas.white_texture());
            canvas.draw_rect(Vec2::new(-16.-64., -9.), Vec2::new(64., 36.), color::rgb(0., 0.,0.), &canvas.white_texture());
            canvas.draw_rect(Vec2::new(16., -9.), Vec2::new(64., 36.), color::rgb(0., 0.,0.), &canvas.white_texture());
            
            canvas.draw_rect(canvas.screen_to_world_pos(input::mouse_position().as_vec2())-Vec2::new(0.5, 0.5), Vec2::new(1., 1.), color::WHITE, mouse_texture);
            
            input::reset_pressed();
            canvas.flush();
        }
    });
}

fn main() {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async_main());
}
