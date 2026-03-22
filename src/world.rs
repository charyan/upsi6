use std::{cell::RefCell, rc::Rc};

use glam::Vec2;
use marmalade::{
    audio::{self, Audio, SoundHandle},
    input, rand,
    render::canvas2d::TextureRect,
};

use crate::{TARGET_PRESS, VIEW_POS, VIEW_SIZE, WorldState, assets::Assets};

const MAX_STAT: i8 = 5;
const MAX_TIME: i32 = 60;
const MIN_TIME: i32 = 12;
const TICK_PRESS: i32 = 12;

pub const RANDOM_KEYS: &[(&'static str, input::Key)] = &[
    ("A", input::Key::A),
    ("X", input::Key::X),
    ("K", input::Key::K),
    ("Q", input::Key::Q),
    ("P", input::Key::P),
    ("T", input::Key::T),
    ("L", input::Key::L),
    ("O", input::Key::O),
    ("H", input::Key::H),
    ("M", input::Key::M),
];

fn random_key() -> (&'static str, input::Key) {
    let index = rand::rand_range(0., RANDOM_KEYS.len() as f64) as usize;
    if let Some(value) = RANDOM_KEYS.get(index) {
        *value
    } else {
        panic!()
    }
}

pub struct Resource {
    pub radius: f32,
    pub pos: Vec2,
    pub lubrication: i8,
    pub sharpening: i8,
    pub energy: i8,
    pub alive: bool,
    pub movable: bool,
    pub texture: TextureRect,
}

impl Resource {
    pub fn new(
        radius: f32,
        pos: Vec2,
        lubrication: i8,
        sharpening: i8,
        energy: i8,
        texture: TextureRect,
    ) -> Resource {
        Resource {
            radius,
            pos,
            lubrication,
            sharpening,
            energy,
            alive: true,
            movable: true,
            texture,
        }
    }
}

pub struct Scraper {
    pub lubrication: i8,
    pub sharpening: i8,
    pub energy: i8,
    pub current_ressource_shredded: Option<Rc<RefCell<Resource>>>,
    pub current_shredding_tick: i32,
    pub target_shredding_tick: i32,
    pub waiting_key: Option<(&'static str, input::Key)>,
    pub current_press: i32,
    pub tick_press: i32,
    pub key_down: bool,
    pub shredding_hand: bool,
}

impl Scraper {
    pub fn new() -> Self {
        Scraper {
            lubrication: MAX_STAT,
            sharpening: MAX_STAT,
            energy: MAX_STAT,
            current_ressource_shredded: None,
            current_shredding_tick: 0,
            target_shredding_tick: 0,
            waiting_key: None,
            current_press: 0,
            tick_press: 0,
            key_down: false,
            shredding_hand: false,
        }
    }

    pub fn last_shred(&mut self) {
        self.target_shredding_tick = 180;
        self.shredding_hand = true;
    }

    pub fn tick(&mut self, assets: &mut Assets) {
        if let Some(key) = self.waiting_key {
            if self.tick_press > TICK_PRESS {
                self.tick_press = 0;
                self.key_down = !self.key_down;

                if self.current_press > 0 {
                    self.current_press -= 1;
                }
            } else {
                self.tick_press += 1;
            }

            if input::is_key_pressed(key.1) {
                self.current_press += 2;
            }

            if self.current_press > TARGET_PRESS {
                self.waiting_key = None;
                self.current_press = 0;
            }
        }

        if let Some(resource) = &mut self.current_ressource_shredded {
            if self.current_shredding_tick > self.target_shredding_tick {
                self.lubrication += resource.borrow().lubrication;
                self.energy += resource.borrow().energy;
                self.sharpening += resource.borrow().sharpening;

                if self.lubrication < 0 {
                    self.lubrication = 0;
                    self.waiting_key = Some(random_key());
                }

                if self.energy < 0 {
                    self.energy = 0;
                    self.waiting_key = Some(random_key());
                }

                if self.sharpening < 0 {
                    self.sharpening = 0;
                    self.waiting_key = Some(random_key());
                }

                if self.waiting_key.is_some() {
                    audio::play(&mut assets.shreder_break, 1.);
                }

                resource.borrow_mut().alive = false;
                self.current_ressource_shredded = None;
            } else {
                self.current_shredding_tick += 1;

                let ratio = 1. / self.target_shredding_tick as f32;
                let radius = resource.borrow().radius;

                resource.borrow_mut().pos.y -= radius * ratio;
            }
        } else if self.shredding_hand {
            if self.current_shredding_tick > self.target_shredding_tick {
                self.shredding_hand = false;
            } else {
                self.current_shredding_tick += 1;
            }
        }
    }

    pub fn get_speed(&self) -> i32 {
        let percentage: f32 =
            (self.energy + self.lubrication + self.sharpening) as f32 / (MAX_STAT as f32 * 3.0);

        let ticks = if percentage == 0. {
            MAX_TIME
        } else {
            (MIN_TIME as f32 / percentage) as i32
        };

        ticks
    }

    pub fn shred(&mut self, ressource: Rc<RefCell<Resource>>) {
        self.current_shredding_tick = 0;
        self.target_shredding_tick = self.get_speed();
        ressource.borrow_mut().movable = false;
        self.current_ressource_shredded = Some(ressource);
    }
}

fn create_resources(assets: &Assets) -> [Vec<Rc<RefCell<Resource>>>; 5] {
    let make_res =
        |val, pos, a, b, c, asset| Rc::new(RefCell::new(Resource::new(val, pos, a, b, c, asset)));

    let l0_data = [(
        18.,
        Vec2::new(0., 0.),
        0,
        0,
        0,
        assets.gui_intro_instructions.clone(),
    )];

    let l1_data = [
        (6., Vec2::new(2.3, -2.), -1, 2, -1, assets.l1_chair.clone()),
        (6., Vec2::new(9.7, 6.), -1, 2, -1, assets.l1_chair.clone()),
        (3., Vec2::new(-4.3, -6.4), 0, 2, -1, assets.l1_trash.clone()),
        (3., Vec2::new(5.8, 1.24), 0, 2, -1, assets.l1_trash.clone()),
        (9., Vec2::new(-3.5, 1.5), 2, -1, -1, assets.l1_desk.clone()),
        (9., Vec2::new(5.7, 8.5), 2, -1, -1, assets.l1_desk.clone()),
        (
            2.,
            Vec2::new(-3.3, 2.7),
            2,
            -1,
            -1,
            assets.l1_computer.clone(),
        ),
        (
            2.,
            Vec2::new(12., -4.6),
            2,
            -1,
            -1,
            assets.l1_computer.clone(),
        ),
        (1., Vec2::new(-0.23, 3.9), -1, -1, 2, assets.l1_can.clone()),
        (1., Vec2::new(-0., 3.), -1, -1, 2, assets.l1_can.clone()),
        (1., Vec2::new(5., 7.5), -1, -1, 2, assets.l1_can.clone()),
        (1., Vec2::new(4.5, 7.9), -1, -1, 2, assets.l1_can.clone()),
        (1., Vec2::new(-12., 5.6), -1, -1, 2, assets.l1_can.clone()),
        (1., Vec2::new(-14., 0.66), -1, -1, 2, assets.l1_can.clone()),
    ];

    let l2_data = [
        (
            15.,
            Vec2::new(-106.176, -61.72),
            0,
            -1,
            1,
            assets.l2_light.clone(),
        ),
        (
            15.,
            Vec2::new(-70.9, -80.9),
            0,
            -1,
            1,
            assets.l2_light.clone(),
        ),
        (
            15.,
            Vec2::new(-118.0, 0.87),
            0,
            -1,
            1,
            assets.l2_light.clone(),
        ),
        (
            15.,
            Vec2::new(-85.3, -20.5),
            0,
            -1,
            1,
            assets.l2_light.clone(),
        ),
        (
            15.,
            Vec2::new(-45.4, -41.1),
            0,
            -1,
            1,
            assets.l2_light.clone(),
        ),
        (
            15.,
            Vec2::new(-10.9, -64.0),
            0,
            -1,
            1,
            assets.l2_light.clone(),
        ),
        (
            10.,
            Vec2::new(-105.4, -37.57949),
            -1,
            2,
            -1,
            assets.l2_manholecover.clone(),
        ),
        (
            10.,
            Vec2::new(-55.5, -68.0),
            -1,
            2,
            -1,
            assets.l2_manholecover.clone(),
        ),
        (
            20.,
            Vec2::new(-130.6, -24.13),
            3,
            -1,
            -1,
            assets.l2_car.clone(),
        ),
        (
            40.,
            Vec2::new(-80.0, -54.7),
            3,
            -1,
            -1,
            assets.l2_truck.clone(),
        ),
        (
            6.,
            Vec2::new(4.7, -66.78),
            -1,
            3,
            -1,
            assets.l2_letterbox.clone(),
        ),
        (
            10.,
            Vec2::new(-86.5, 14.75),
            -1,
            1,
            -1,
            assets.l2_bench.clone(),
        ),
        (
            10.,
            Vec2::new(-85.79, -0.71),
            -1,
            1,
            -1,
            assets.l2_bench.clone(),
        ),
        (
            10.,
            Vec2::new(-61.2, 3.6),
            -1,
            1,
            -1,
            assets.l2_bench.clone(),
        ),
    ];

    let l3_data = [
        (
            500.,
            Vec2::new(-929.39, 273.9),
            -1,
            3,
            -1,
            assets.l3_bridge.clone(),
        ),
        (
            100.,
            Vec2::new(0.73, -27.29),
            -1,
            -1,
            1,
            assets.l3_l2_object.clone(),
        ),
        (
            100.,
            Vec2::new(-148.21, 58.76),
            -1,
            -1,
            1,
            assets.l3_l2_object.clone(),
        ),
        (
            500.,
            Vec2::new(-1541.75, 429.49),
            -1,
            3,
            -1,
            assets.l3_airport.clone(),
        ),
        (
            200.,
            Vec2::new(197.68, 611.54),
            -1,
            -1,
            3,
            assets.l3_hotairbaloon.clone(),
        ),
        (
            150.,
            Vec2::new(-982.35, -22.32),
            -1,
            2,
            -1,
            assets.l3_boat.clone(),
        ),
        (
            150.,
            Vec2::new(-719.20, -341.74),
            -1,
            2,
            -1,
            assets.l3_boat.clone(),
        ),
        (
            90.,
            Vec2::new(-1316.6667, -47.15),
            1,
            -1,
            -1,
            assets.l3_tree.clone(),
        ),
        (
            90.,
            Vec2::new(-1415.96, -275.54),
            1,
            -1,
            -1,
            assets.l3_tree.clone(),
        ),
        (
            90.,
            Vec2::new(-1190.88, -244.1),
            1,
            -1,
            -1,
            assets.l3_tree.clone(),
        ),
        (
            300.,
            Vec2::new(-598.38, -45.49),
            -1,
            -1,
            3,
            assets.l3_crane.clone(),
        ),
        (
            100.,
            Vec2::new(-461.01, -164.65),
            -1,
            1,
            -1,
            assets.l3_container.clone(),
        ),
        (
            100.,
            Vec2::new(-341.85, -139.83),
            -1,
            1,
            -1,
            assets.l3_container.clone(),
        ),
        (
            100.,
            Vec2::new(-413.02, -263.96),
            -1,
            1,
            -1,
            assets.l3_container.clone(),
        ),
        (
            100.,
            Vec2::new(-1442.44, -91.83),
            2,
            -1,
            -1,
            assets.l3_cow.clone(),
        ),
        (
            100.,
            Vec2::new(-1334.87, -196.10),
            2,
            -1,
            -1,
            assets.l3_cow.clone(),
        ),
        (
            150.,
            Vec2::new(-240.90, 263.99),
            -1,
            -1,
            1,
            assets.l3_building.clone(),
        ),
        (
            100.,
            Vec2::new(-535.49, 297.09),
            -1,
            -1,
            1,
            assets.l3_building.clone(),
        ),
        (
            100.,
            Vec2::new(-479.22, -396.36),
            2,
            -1,
            0,
            assets.l3_whale.clone(),
        ),
        (
            100.,
            Vec2::new(-343.51, -432.77),
            2,
            -1,
            0,
            assets.l3_whale.clone(),
        ),
    ];

    let l4_data = [
        (
            10000.,
            Vec2::new(18483.207, 759.7441),
            -1,
            -1,
            1,
            assets.l4_sun.clone(),
        ),
        (
            1000.,
            Vec2::new(-5955.55, -264.4925),
            1,
            -1,
            -1,
            assets.l4_moon.clone(),
        ),
        (
            1500.,
            Vec2::new(-3383.0498, 7595.926),
            -1,
            -1,
            1,
            assets.l4_comet.clone(),
        ),
        (
            1000.,
            Vec2::new(2190.7021, 7691.2036),
            -1,
            3,
            -1,
            assets.l4_star.clone(),
        ),
        (
            1000.,
            Vec2::new(-381.79883, 8620.162),
            -1,
            3,
            -1,
            assets.l4_star.clone(),
        ),
        (
            600.,
            Vec2::new(1523.7578, -407.4095),
            -1,
            1,
            -1,
            assets.l4_sat.clone(),
        ),
        (
            1200.,
            Vec2::new(13624.037, 6690.7866),
            -1,
            -1,
            1,
            assets.l4_milkyway.clone(),
        ),
        (
            1600.,
            Vec2::new(6811.676, -4456.7153),
            -1,
            -1,
            1,
            assets.l4_milkyway.clone(),
        ),
        (
            4000.,
            Vec2::new(7145.1475, 2260.3691),
            1,
            -1,
            -1,
            assets.l4_helmet.clone(),
        ),
        (
            4200.,
            Vec2::new(-810.5488, -312.1314),
            1,
            -1,
            -1,
            assets.l3_object.clone(),
        ),
    ];

    let res_l0 = l0_data
        .into_iter()
        .map(|(v, p, a, b, c, asset)| make_res(v, p, a, b, c, asset))
        .collect();

    let res_l1 = l1_data
        .into_iter()
        .map(|(v, p, a, b, c, asset)| make_res(v, p, a, b, c, asset))
        .collect();

    let res_l2 = l2_data
        .into_iter()
        .map(|(v, p, a, b, c, asset)| make_res(v, p, a, b, c, asset))
        .collect();

    let res_l3 = l3_data
        .into_iter()
        .map(|(v, p, a, b, c, asset)| make_res(v, p, a, b, c, asset))
        .collect();

    let res_l4 = l4_data
        .into_iter()
        .map(|(v, p, a, b, c, asset)| make_res(v, p, a, b, c, asset))
        .collect();

    [res_l0, res_l1, res_l2, res_l3, res_l4]
}

pub struct World {
    pub selected: Option<Rc<RefCell<Resource>>>,
    pub view_radius: f32,
    pub cam_pos: Vec2,
    pub resources: [Vec<Rc<RefCell<Resource>>>; 5],
    pub scraper: Scraper,
    stage: usize,
    pub state: WorldState,
    pub music_handle: Option<SoundHandle>,
    pub background_handle: Option<SoundHandle>,
    pub timer: u64,
    pub end_tick: u64,
    pub transition_running: bool
}

impl World {
    pub fn new(assets: &Assets) -> World {
        let resources = create_resources(assets);

        let scraper = Scraper::new();

        World {
            stage: 0,
            selected: None,
            view_radius: VIEW_SIZE[0].x / 2.,
            cam_pos: Vec2::ZERO,
            resources,
            scraper,
            state: WorldState::EMAIL,
            music_handle: None,
            background_handle: None,
            timer: 0,
            end_tick: 0,
            transition_running: false
        }
    }

    pub fn handle_stage_pos(&mut self) {
        let (target_radius, previous_radius) = match self.stage {
            0 => (VIEW_SIZE[0].x / 2., VIEW_SIZE[0].x / 4.),
            1 => (VIEW_SIZE[1].x / 2., VIEW_SIZE[0].x / 4.),
            2 => (VIEW_SIZE[2].x / 2., VIEW_SIZE[1].x / 2.),
            3 => (VIEW_SIZE[3].x / 2., VIEW_SIZE[2].x / 2.),
            4 => (VIEW_SIZE[4].x / 2., VIEW_SIZE[3].x / 2.),

            _ => panic!(),
        };

        if self.view_radius * 1.01 < target_radius {
            self.view_radius *= 1.01;
        } else {
            self.transition_running = false;
        }

        let (target_pos, previous_pos) = match self.stage {
            0 => (
                VIEW_POS[0] + VIEW_SIZE[0] / 2.,
                VIEW_POS[0] + VIEW_SIZE[0] / 2.,
            ),
            1 => (
                VIEW_POS[1] + VIEW_SIZE[1] / 2.,
                VIEW_POS[0] + VIEW_SIZE[0] / 2.,
            ),
            2 => (
                VIEW_POS[2] + VIEW_SIZE[2] / 2.,
                VIEW_POS[0] + VIEW_SIZE[0] / 2.,
            ),
            3 => (
                VIEW_POS[3] + VIEW_SIZE[3] / 2.,
                VIEW_POS[2] + VIEW_SIZE[2] / 2.,
            ),
            4 => (
                VIEW_POS[4] + VIEW_SIZE[4] / 2.,
                VIEW_POS[3] + VIEW_SIZE[3] / 2.,
            ),
            _ => panic!(),
        };

        let ratio = (self.view_radius - previous_radius) / (target_radius - previous_radius);
        self.cam_pos = target_pos * ratio + (1. - ratio) * previous_pos;
    }

    pub fn get_stage(&self) -> usize {
        self.stage
    }

    pub fn next_stage(&mut self, assets: &Assets) {
        self.stage += 1;
        self.transition_running = true;
        if let Some(music_handle) = &self.music_handle {
            music_handle.stop();
            self.music_handle = None;
        }
        if let Some(background_handle) = &self.background_handle {
            background_handle.stop();
            self.background_handle = None;
        }


        self.music_handle = Some(audio::play_loop(&assets.music_act[self.stage.saturating_sub(1)], 0.5));
        if let Some(background) = &assets.background_act[self.stage.saturating_sub(1)] {
            self.background_handle = Some(audio::play_loop(background, 1.))
        } 
    }

    pub fn tick(&mut self, assets: &mut Assets) {
        self.handle_stage_pos();
        self.scraper.tick(assets);
        if let WorldState::PLAY = self.state {
            self.timer += 1;
        }
        if let WorldState::MOVING = self.state {
            self.end_tick += 1
        }
    }
}
