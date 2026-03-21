use std::time::Duration;

use glam::Mat3;
use glam::Vec2;
use glam::Vec4;
use marmalade::audio;
use marmalade::input::Key;

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

pub const VIEW_POS: [Vec2; 4] = [
    Vec2::new(-16., -9.),
    Vec2::new(-150., -85.),
    Vec2::new(-1950., -490.),
    Vec2::new(-8000., -8000.),
];

pub const VIEW_SIZE: [Vec2; 4] = [
    Vec2::new(32., 18.),
    Vec2::new(195., 110.),
    Vec2::new(2240., 1260.),
    Vec2::new(32000., 18000.),
];

fn draw_game(canvas: &mut Canvas2d, world: &mut World, assets: &Assets) {
    canvas.camera_view_ratio(world.cam_pos, world.view_radius, ASPECT_RATIO);

    let mouse_pos = canvas.screen_to_world_pos(input::mouse_position().as_vec2());

    canvas.draw_rect(VIEW_POS[3], VIEW_SIZE[3], color::WHITE, &assets.l4);

    canvas.draw_rect(
        Vec2::new(-2800., -2250.),
        Vec2::new(4000., 4000.),
        color::WHITE,
        &assets.earth_resource,
    );

    canvas.draw_rect(VIEW_POS[2], VIEW_SIZE[2], color::WHITE, &assets.l3);

    canvas.draw_rect(VIEW_POS[1], VIEW_SIZE[1], color::WHITE, &assets.l2);

    canvas.draw_rect(VIEW_POS[0], VIEW_SIZE[0], color::WHITE, &assets.l1);

    if !input::is_button_down(Button::Left) {
        world.selected = None;
    }

    let mouse_clicked = input::is_button_pressed(Button::Left);

    for resource in &world.resources[world.get_stage() - 1] {
        let r = resource.borrow();

        let radius = r.radius
            * if r.movable && r.pos.distance(mouse_pos) < r.radius {
                if mouse_clicked {
                    world.selected = Some(resource.clone());
                }

                let color_circle: Vec4 = if r.energy > 0 {
                    color::rgba(1., 1., 0., 0.5)
                } else if r.lubrication > 0 {
                    color::rgba(0., 0., 1., 0.4)
                } else if r.sharpening > 0 {
                    color::rgba(1., 0., 0., 0.4)
                } else {
                    color::rgba(1., 1., 1., 0.)
                };

                canvas.draw_regular(
                    r.pos,
                    r.radius / 2.,
                    64,
                    color_circle,
                    &canvas.white_texture(),
                );
                1.1
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

            let screen_pos = canvas.world_to_screen_pos(s.pos);

            canvas.camera_view_ratio(Vec2::new(0.0, 0.0), 16., ASPECT_RATIO);

            let interface_pos = canvas.screen_to_world_pos(screen_pos);

            if world.scraper.current_ressource_shredded.is_none() {
                if interface_pos.distance(Vec2::new(
                    SHREDDER_POS.x + SHREDDER_SIZE.x / 2.,
                    SHREDDER_POS.y + SHREDDER_SIZE.y,
                )) < 1.
                {
                    let screen_pos = canvas.world_to_screen_pos(
                        SHREDDER_POS + Vec2::new(SHREDDER_SIZE.x / 2., SHREDDER_SIZE.y),
                    );

                    canvas.camera_view_ratio(world.cam_pos, world.view_radius, ASPECT_RATIO);

                    let world_pos = canvas.screen_to_world_pos(screen_pos);

                    s.pos = world_pos;
                    drop(s);
                    world.scraper.shred(selected.clone());

                    Some(audio::play(&assets.shreder_sound, 1.0));
                }
            }
        } else {
            world.selected = None;
        }
    }

    world.resources[world.get_stage() - 1].retain(|x| x.borrow().alive);

    canvas.camera_view_ratio(Vec2::new(0.0, 0.0), 16., ASPECT_RATIO);

    canvas.draw_rect(
        SHREDDER_POS,
        SHREDDER_SIZE,
        color::WHITE,
        &assets.shredder_box,
    );

    let wheel_pos_1 = Vec2::new(
        SHREDDER_POS.x - WHEEL_SIZE.x / 2. + 0.5,
        SHREDDER_POS.y + WHEEL_SIZE.y / 2. + 0.5,
    );

    let wheel_pos_2 = Vec2::new(
        SHREDDER_POS.x + WHEEL_SIZE.x / 2. + 0.5,
        SHREDDER_POS.y + WHEEL_SIZE.y / 2. + 0.5,
    );

    let wheel_pos_3 = Vec2::new(
        SHREDDER_POS.x + SHREDDER_SIZE.x / 2. - WHEEL_SIZE.x / 2.,
        SHREDDER_POS.y + SHREDDER_SIZE.y / 2. - WHEEL_SIZE.y / 2.,
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

    let assets = Assets::load(&mut canvas).await;

    let mut world = World::new(&assets);

    let mut tick_scheduler = TickScheduler::new(Duration::from_secs_f64(1.0 / 60.0)); // 60 HZ
    draw_scheduler::set_on_draw(move || {
        for _ in 0..tick_scheduler.tick_count() {
            if world.resources[world.get_stage() - 1].is_empty() {
                world.next_stage(&assets);
            }

            world.tick();

            canvas.fit_screen();

            canvas.clear(color::rgb(0., 0., 0.));

            draw_game(&mut canvas, &mut world, &assets);

            canvas.flush();

            input::reset_pressed();
        }
    });
}

fn main() {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async_main());
}
