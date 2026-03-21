use std::{cell::RefCell, rc::Rc};

use glam::Vec2;
use marmalade::console;

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
}

impl Resource {
    pub fn new(radius: f32, pos: Vec2, lubrication: i8, sharpening: i8, energy: i8) -> Resource {
        Resource {
            radius,
            pos,
            lubrication,
            sharpening,
            energy,
            alive: true,
            movable: true,
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

pub struct World {
    pub selected: Option<Rc<RefCell<Resource>>>,
    pub view_radius: f32,
    pub resources: Vec<Rc<RefCell<Resource>>>,
    pub scraper: Scraper,
}

impl World {
    pub fn new() -> World {
        let mut resources = Vec::new();

        for x in 0..3 {
            for y in 0..3 {
                resources.push(Rc::new(RefCell::new(Resource::new(
                    0.5,
                    Vec2::new((x - 1) as f32 * 2.0, (y - 1) as f32 * 2.0),
                    -1,
                    -1,
                    -1,
                ))));
            }
        }
        let scraper = Scraper::new();

        World {
            selected: None,
            view_radius: 4.,
            resources,
            scraper,
        }
    }

    pub fn tick(&mut self) {
        self.view_radius *= 1.0;
        self.scraper.tick();
    }
}
