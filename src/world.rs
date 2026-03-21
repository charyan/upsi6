use std::{cell::RefCell, rc::Rc};

use glam::Vec2;
use marmalade::{
    audio::{self, Audio, SoundHandle},
    render::canvas2d::TextureRect,
};

use crate::{VIEW_POS, VIEW_SIZE, assets::Assets};

const MAX_STAT: i8 = 5;
const MAX_TIME: i32 = 60;
const MIN_TIME: i32 = 12;

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
        }
    }

    pub fn tick(&mut self) {
        if let Some(resource) = &mut self.current_ressource_shredded {
            if self.current_shredding_tick > self.target_shredding_tick {
                self.lubrication += resource.borrow().lubrication;
                self.energy += resource.borrow().energy;
                self.sharpening += resource.borrow().sharpening;

                if self.lubrication < 0 {
                    self.lubrication = 0
                }

                if self.energy < 0 {
                    self.energy = 0
                }

                if self.sharpening < 0 {
                    self.sharpening = 0
                }

                resource.borrow_mut().alive = false;
                self.current_ressource_shredded = None;
            } else {
                self.current_shredding_tick += 1;

                let ratio = 1. / self.target_shredding_tick as f32;
                let radius = resource.borrow().radius;

                resource.borrow_mut().pos.y -= radius * ratio;
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

fn create_resources(assets: &Assets) -> [Vec<Rc<RefCell<Resource>>>; 4] {
    let mut res_l1 = Vec::new();

    res_l1.push(Rc::new(RefCell::new(Resource::new(
        1.,
        Vec2::new(1., 1.),
        -1,
        -1,
        2,
        assets.l1_can.clone(),
    ))));

    res_l1.push(Rc::new(RefCell::new(Resource::new(
        1.,
        Vec2::new(2., 2.),
        -1,
        2,
        -1,
        assets.l1_chair.clone(),
    ))));

    res_l1.push(Rc::new(RefCell::new(Resource::new(
        1.,
        Vec2::new(3., 3.),
        -1,
        -1,
        2,
        assets.l1_computer.clone(),
    ))));

    res_l1.push(Rc::new(RefCell::new(Resource::new(
        1.,
        Vec2::new(4., 4.),
        -1,
        -1,
        -1,
        assets.l1_desk.clone(),
    ))));

    res_l1.push(Rc::new(RefCell::new(Resource::new(
        1.,
        Vec2::new(5., 5.),
        2,
        -1,
        -1,
        assets.l1_trash.clone(),
    ))));

    let mut res_l2 = Vec::new();
    res_l2.push(Rc::new(RefCell::new(Resource::new(
        10.,
        Vec2::new(20., 20.),
        -1,
        -1,
        -1,
        assets.l2_bench.clone(),
    ))));

    res_l2.push(Rc::new(RefCell::new(Resource::new(
        20.,
        Vec2::new(30., 20.),
        -1,
        -1,
        -1,
        assets.l2_car.clone(),
    ))));

    res_l2.push(Rc::new(RefCell::new(Resource::new(
        10.,
        Vec2::new(0., 0.),
        -1,
        -1,
        -1,
        assets.l2_light.clone(),
    ))));

    res_l2.push(Rc::new(RefCell::new(Resource::new(
        10.,
        Vec2::new(-10., -10.),
        -1,
        -1,
        -1,
        assets.l2_letterbox.clone(),
    ))));

    res_l2.push(Rc::new(RefCell::new(Resource::new(
        10.,
        Vec2::new(-20., -25.),
        -1,
        -1,
        -1,
        assets.l2_manholecover.clone(),
    ))));

    res_l2.push(Rc::new(RefCell::new(Resource::new(
        10.,
        Vec2::new(-90., -20.),
        -1,
        -1,
        -1,
        assets.l2_object.clone(),
    ))));

    res_l2.push(Rc::new(RefCell::new(Resource::new(
        10.,
        Vec2::new(-10., -25.),
        -1,
        -1,
        -1,
        assets.l2_truck.clone(),
    ))));

    let mut res_l3 = Vec::new();
    res_l3.push(Rc::new(RefCell::new(Resource::new(
        100.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l3_airport.clone(),
    ))));

    res_l3.push(Rc::new(RefCell::new(Resource::new(
        100.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l3_boat.clone(),
    ))));

    res_l3.push(Rc::new(RefCell::new(Resource::new(
        100.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l3_bridge.clone(),
    ))));

    res_l3.push(Rc::new(RefCell::new(Resource::new(
        100.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l3_building.clone(),
    ))));

    res_l3.push(Rc::new(RefCell::new(Resource::new(
        100.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l3_container.clone(),
    ))));

    res_l3.push(Rc::new(RefCell::new(Resource::new(
        100.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l3_cow.clone(),
    ))));

    res_l3.push(Rc::new(RefCell::new(Resource::new(
        100.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l3_crane.clone(),
    ))));

    res_l3.push(Rc::new(RefCell::new(Resource::new(
        100.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l3_hotairbaloon.clone(),
    ))));

    res_l3.push(Rc::new(RefCell::new(Resource::new(
        100.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l3_house.clone(),
    ))));

    res_l3.push(Rc::new(RefCell::new(Resource::new(
        100.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l3_object.clone(),
    ))));

    res_l3.push(Rc::new(RefCell::new(Resource::new(
        100.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l3_tree.clone(),
    ))));

    let mut res_l4 = Vec::new();
    res_l4.push(Rc::new(RefCell::new(Resource::new(
        1000.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l4_star.clone(),
    ))));

    res_l4.push(Rc::new(RefCell::new(Resource::new(
        1000.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l4_sat.clone(),
    ))));

    res_l4.push(Rc::new(RefCell::new(Resource::new(
        1000.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l4_helmet.clone(),
    ))));

    res_l4.push(Rc::new(RefCell::new(Resource::new(
        1000.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l4_moon.clone(),
    ))));

    res_l4.push(Rc::new(RefCell::new(Resource::new(
        1000.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l4_milkyway.clone(),
    ))));

    res_l4.push(Rc::new(RefCell::new(Resource::new(
        1000.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l4_helmet.clone(),
    ))));

    res_l4.push(Rc::new(RefCell::new(Resource::new(
        1000.,
        Vec2::new(100., -25.),
        -1,
        -1,
        -1,
        assets.l4_comet.clone(),
    ))));
    let resources = [res_l1, res_l2, res_l3, res_l4];

    resources
}

pub struct World {
    pub selected: Option<Rc<RefCell<Resource>>>,
    pub view_radius: f32,
    pub cam_pos: Vec2,
    pub resources: [Vec<Rc<RefCell<Resource>>>; 4],
    pub scraper: Scraper,
    stage: usize,
    pub music_handle: Option<SoundHandle>,
}

impl World {
    pub fn new(assets: &Assets) -> World {
        let resources = create_resources(assets);

        let scraper = Scraper::new();

        World {
            stage: 1,
            selected: None,
            view_radius: VIEW_SIZE[0].x / 2.,
            cam_pos: Vec2::ZERO,
            resources,
            scraper,
            music_handle: None,
        }
    }

    pub fn handle_stage_pos(&mut self) {
        let (target_radius, previous_radius) = match self.stage {
            1 => (VIEW_SIZE[0].x / 2., VIEW_SIZE[0].x / 4.),
            2 => (VIEW_SIZE[1].x / 2., VIEW_SIZE[0].x / 2.),
            3 => (VIEW_SIZE[2].x / 2., VIEW_SIZE[1].x / 2.),
            4 => (VIEW_SIZE[3].x / 2., VIEW_SIZE[2].x / 2.),
            _ => panic!(),
        };

        if self.view_radius * 1.01 < target_radius {
            self.view_radius *= 1.01;
        }

        let (target_pos, previous_pos) = match self.stage {
            1 => (
                VIEW_POS[0] + VIEW_SIZE[0] / 2.,
                VIEW_POS[0] + VIEW_SIZE[0] / 2.,
            ),
            2 => (
                VIEW_POS[1] + VIEW_SIZE[1] / 2.,
                VIEW_POS[0] + VIEW_SIZE[0] / 2.,
            ),
            3 => (
                VIEW_POS[2] + VIEW_SIZE[2] / 2.,
                VIEW_POS[1] + VIEW_SIZE[1] / 2.,
            ),
            4 => (
                VIEW_POS[3] + VIEW_SIZE[3] / 2.,
                VIEW_POS[2] + VIEW_SIZE[2] / 2.,
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
        if let Some(music_handle) = &self.music_handle {
            music_handle.stop();
        }
        self.music_handle = Some(audio::play_loop(&assets.music_act[self.stage - 1], 1.0));
    }

    pub fn tick(&mut self) {
        self.handle_stage_pos();
        self.scraper.tick();
    }
}
