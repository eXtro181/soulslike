//! Pixelschrift (5×7 mit Umlauten) für die Oberfläche in UI-Pixeln.

use crate::art::Art;
use crate::gfx::{Frame, SprOpt};

pub const GAP: f32 = 1.0;

fn adv(art: &Art, ch: char) -> f32 {
    if ch == ' ' {
        return 3.0;
    }
    match art.glyphs.get(&ch).or_else(|| art.glyphs.get(&fallback(ch))) {
        Some((_, w)) => *w as f32 + GAP,
        None => 4.0,
    }
}
fn fallback(ch: char) -> char {
    match ch {
        '“' | '”' | '«' | '»' => '"',
        '‹' => '<',
        '›' => '>',
        '–' | '—' => '-',
        '’' | '‘' => '\'',
        'é' | 'è' => 'e',
        _ => '?',
    }
}

pub fn width(art: &Art, s: &str) -> f32 {
    let w: f32 = s.chars().map(|c| adv(art, c)).sum();
    (w - GAP).max(0.0)
}

#[derive(Clone, Copy, PartialEq)]
pub enum Al {
    L,
    C,
    R,
}

/// Text zeichnen; `y` = Oberkante. Farbe in sRGB-Bytes.
pub fn draw(f: &mut Frame, art: &Art, s: &str, x: f32, y: f32, col: [u8; 4], al: Al, scale: f32, shadow: bool) {
    let w = width(art, s) * scale;
    let mut cx = match al {
        Al::L => x,
        Al::C => x - w / 2.0,
        Al::R => x - w,
    }
    .round();
    let y = y.round();
    for ch in s.chars() {
        if ch != ' ' {
            if let Some((g, _)) = art.glyphs.get(&ch).or_else(|| art.glyphs.get(&fallback(ch))) {
                if shadow {
                    f.ui_spr(g, cx + scale, y + scale, SprOpt { tint: [0, 0, 0, (col[3] as f32 * 0.75) as u8], scale, ..Default::default() });
                }
                f.ui_spr(g, cx, y, SprOpt { tint: col, scale, ..Default::default() });
            }
        }
        cx += adv(art, ch) * scale;
    }
}

pub fn tc(f: &mut Frame, art: &Art, s: &str, x: f32, y: f32, col: [u8; 4]) {
    draw(f, art, s, x, y, col, Al::C, 1.0, true)
}


pub mod col {
    pub const BONE: [u8; 4] = [216, 207, 184, 255];
    pub const BONE_DIM: [u8; 4] = [216, 207, 184, 180];
    pub const GOLD: [u8; 4] = [232, 164, 67, 255];
    pub const SOUL: [u8; 4] = [95, 242, 200, 255];
    pub const BLOOD_L: [u8; 4] = [194, 80, 46, 255];
    pub const INK: [u8; 4] = [11, 16, 22, 255];
    pub const MIST: [u8; 4] = [143, 163, 168, 255];
    pub const GREY: [u8; 4] = [150, 150, 150, 130];
    pub const RED: [u8; 4] = [230, 90, 76, 255];
}
