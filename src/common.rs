// 
use piston_window::*;
use std::path::PathBuf;
use find_folder;
use music;

// shock/horror
static mut SOUND_IS_ON: bool = false;

#[derive(Copy, Clone)]
pub struct TargetBrick {
    pub x: f64,
    pub y: f64,
    pub brick_id: usize,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Sound {
    Fire,
    ShipExplode,
    SpiderExplode,
    TakeBrick,
    DepositBrick,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Music {}

pub const SCREEN_WIDTH: f64 = 600.0;
pub const SCREEN_HEIGHT: f64 = 700.0;

pub struct ScreenObjectArea {
    tl_x: f64,
    tl_y: f64,
    br_x: f64,
    br_y: f64,
}

impl ScreenObjectArea {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> ScreenObjectArea {
        ScreenObjectArea {tl_x: x, tl_y: y, br_x: x + w, br_y: y + h}
    }

    pub fn collides(&self, col_area: ScreenObjectArea) -> bool {
        col_area.br_x > self.tl_x && col_area.tl_x < self.br_x &&
        col_area.br_y > self.tl_y && col_area.tl_y < self.br_y
    }
}

pub fn find_asset(file_name: &str) -> PathBuf {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    assets.join(file_name)
}

pub fn win_image(window: &mut PistonWindow, file_name: &str) -> G2dTexture {
    let image_path = find_asset(&format!("image/{}", file_name));
    Texture::from_path(
        &mut window.factory,
        &image_path,
        Flip::None,
        &TextureSettings::new()
    ).unwrap()
}

pub fn play_sound(sound: &Sound) {
    unsafe {
        if SOUND_IS_ON {
            music::play_sound(sound, music::Repeat::Times(0), music::MAX_VOLUME);
        }
    }
}

pub fn sound_on() {
    unsafe {
        SOUND_IS_ON = true;
    }
}

pub fn sound_off() {
    unsafe {
        SOUND_IS_ON = false;
    }
}

