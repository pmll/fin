// for animation sequences that are not informed in any way by game state -
// we just want to set them going and forget about them

use piston_window::*;

type TextRenderFn = Box<Fn(u32, Context, &mut G2d, &mut Glyphs)>;
type NonTextRenderFn = Box<Fn(u32, Context, &mut G2d)>;

enum RenderFn {
    Text(TextRenderFn),
    NonText(NonTextRenderFn),
}


pub struct Animation {
    from_frame: u32,
    frames: u32,
    render: RenderFn,
}

impl Animation {
    fn finished(&self, frame: u32) -> bool {
        frame - self.from_frame >= self.frames
    }
}

pub struct Animations {
    animation: Vec<Animation>,
    frame: u32,
}

impl Animations {
    pub fn new() -> Animations {
        Animations {animation: Vec::new(), frame: 0}
    }

    fn unregister_finished(&mut self) {
        let frame = self.frame;
        self.animation.retain(|a| ! a.finished(frame));
    }

    pub fn register(&mut self, render: NonTextRenderFn, frames: u32) {
        self.animation.push(Animation {from_frame: self.frame, frames, render: RenderFn::NonText(render)});
    }

    pub fn register_text(&mut self, render: TextRenderFn, frames: u32) {
        self.animation.push(Animation {from_frame: self.frame, frames, render: RenderFn::Text(render)});
    }

    pub fn render(&self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
        for a in &self.animation {
            match a.render {
                RenderFn::Text(ref r) => {(r)(self.frame - a.from_frame, c, g, glyphs)},
                RenderFn::NonText(ref r) => {(r)(self.frame - a.from_frame, c, g)},
            }
        }
    }

    pub fn update(&mut self) {
        self.frame += 1;
        self.unregister_finished();
    }

    pub fn in_progress(&self) -> bool {
        self.animation.len() > 0
    }
}

