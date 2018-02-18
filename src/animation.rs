// for animation sequences that are not informed in any way by game state -
// we just want to set them going and forget about them

use piston_window::*;

type RenderFn = Box<Fn(u32, Context, &mut G2d, &mut Glyphs)>;

pub struct Animation {
    from_frame: u32,
    frames: u32,
    render: RenderFn,
}

impl Animation {
    pub fn new(render: RenderFn, frame_no: u32, frames: u32) -> Animation {
        Animation {from_frame: frame_no, frames, render}
    }

    fn finished(&self, frame_no: u32) -> bool {
        frame_no - self.from_frame >= self.frames
    }
}

pub struct Animations {
    animation: Vec<Animation>,
}

impl Animations {
    pub fn new() -> Animations {
        Animations {animation: Vec::new()}
    }

    pub fn register(&mut self, a: Animation) {
        self.animation.push(a);
    }

    pub fn render(&self, c: Context, g: &mut G2d, glyphs: &mut Glyphs, frame_no: u32) {
        for a in &self.animation {
            (a.render)(frame_no - a.from_frame, c, g, glyphs);
        }
    }

    pub fn unregister_finished(&mut self, frame_no: u32) {
        self.animation.retain(|a| ! a.finished(frame_no));
    }

    pub fn in_progress(&self) -> bool {
        self.animation.len() > 0
    }
}

