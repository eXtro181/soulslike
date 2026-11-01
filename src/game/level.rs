//! Zone 01, Die Aschenen Klippen von Calamita: Kachelkarte (32 px), Kollision mit Plattformen und Leitern, Abkürzung und
//! Nebeltor, Zeichnen der Kacheln und der Parallax-Kulisse mit der gestrandeten Galeone.

use crate::art::Art;
use crate::gfx::{flags, modes, Frame, LightDef, SprOpt};
use sw_art::noise::h01;

pub const TS: f32 = 32.0;

/// Legende: `#` Fels · ` ` offene Luft (Kulisse sichtbar) · `.` Höhle (Rückwand) · `=` Planke
/// (von unten durchlässig) · `H` Leiter · `L` Kettenleiter der Abkürzung (erst nach der Winde)
/// · `W` Winde · `S` Ascheschrein · `P` Start · `D` Nebeltor · `B` Barnaby · `m` Aschen-Marodeur
/// · `g` Rost-Skelett · `c` Schlick-Krabbler (an der Decke) · `G` Gallions-Schütze (Miniboss) · `1`/`2` Aschenkünste · `t` Fackel.
pub const MAP: [&str; 40] = [
    "####################################################",
    "####################################################",
    "####                ###########                    #",
    "####                ###########                    #",
    "####                ###########                    #",
    "####         ====   ###########                    #",
    "####                ###########                    #",
    "####                ###########                    #",
    "####    =====       ###########    ====    ====    #",
    "##                  .....c.....D                   #",
    "##                  ...........D                   #",
    "##   W    t   G     ..t.....m.tD t          B    t #",
    "##L#####H###########################################",
    "##L#....H............###############################",
    "##L#....H............###############################",
    "##L#....H............###############################",
    "##L#....H............###############################",
    "##L#....H.m....m...t.###############################",
    "##L##############H##################################",
    "##L#..........c..H...###############################",
    "##L#.............H...###############################",
    "##L#........====.H...###############################",
    "##L#.............H...###############################",
    "##L#t......g.....H..2###############################",
    "##L###H#############################################",
    "##L#..H..............###############################",
    "##L#..H..............###############################",
    "##L#..H..====........###############################",
    "##L#..H..............###############################",
    "##L#.1H.....m......t.###############################",
    "##L###############H#################################",
    "##L...............H..###############################",
    "##L...............H..###############################",
    "##L...............H..###############################",
    "##L...............H..###############################",
    "##L...............H..###############################",
    "##L.t.S.P...t..m..H..###############################",
    "####################################################",
    "####################################################",
    "####################################################",
];

pub struct Level {
    pub w: i32,
    pub h: i32,
    cells: Vec<u8>,
    pub shortcut: bool,
    pub gate_closed: bool,
}

/// Ein Körper mit Fußpunkt (x, y) und Kasten w×h.
#[derive(Clone, Copy, Default)]
pub struct Body {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub vx: f32,
    pub vy: f32,
    pub ground: bool,
    /// durch Planken nach unten fallen
    pub drop: bool,
}

impl Level {
    pub fn new() -> Self {
        let w = MAP[0].len() as i32;
        // Selbstprüfung der Karte: rechteckig, je genau ein Start, Schrein, Winde und Boss
        assert!(MAP.iter().all(|r| r.len() == w as usize), "Kartenzeilen ungleich lang");
        for ch in ['P', 'S', 'W', 'B'] {
            assert_eq!(MAP.iter().map(|r| r.matches(ch).count()).sum::<usize>(), 1, "Karte: {ch} muss genau einmal vorkommen");
        }
        Level { w, h: MAP.len() as i32, cells: MAP.iter().flat_map(|r| r.bytes()).collect(), shortcut: false, gate_closed: false }
    }
    pub fn cell(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            b'#'
        } else {
            self.cells[(y * self.w + x) as usize]
        }
    }
    /// alle Zellen mit diesem Zeichen (Kachel x, y)
    pub fn find(&self, ch: u8) -> Vec<(i32, i32)> {
        (0..self.cells.len() as i32).filter(|i| self.cells[*i as usize] == ch).map(|i| (i % self.w, i / self.w)).collect()
    }
    pub fn solid(&self, x: i32, y: i32) -> bool {
        match self.cell(x, y) {
            b'#' => true,
            b'D' => self.gate_closed,
            _ => false,
        }
    }
    pub fn ladder(&self, x: i32, y: i32) -> bool {
        match self.cell(x, y) {
            b'H' => true,
            b'L' => self.shortcut,
            _ => false,
        }
    }
    /// trägt von oben: Planke oder oberstes Leiterstück
    pub fn oneway(&self, x: i32, y: i32) -> bool {
        self.cell(x, y) == b'=' || (self.ladder(x, y) && !self.ladder(x, y - 1))
    }
    pub fn size(&self) -> (f32, f32) {
        (self.w as f32 * TS, self.h as f32 * TS)
    }

    fn box_hits(&self, x0: f32, y0: f32, x1: f32, y1: f32) -> bool {
        let (tx0, tx1) = ((x0 / TS).floor() as i32, ((x1 - 0.01) / TS).floor() as i32);
        let (ty0, ty1) = ((y0 / TS).floor() as i32, ((y1 - 0.01) / TS).floor() as i32);
        (ty0..=ty1).any(|ty| (tx0..=tx1).any(|tx| self.solid(tx, ty)))
    }
    pub fn blocked_at(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.box_hits(x - w * 0.5, y - h, x + w * 0.5, y)
    }

    /// Bewegen mit Schwerkraft schon in vy; löst erst x, dann y auf. Gibt true zurück, wenn eine Wand in x getroffen wurde.
    pub fn step(&self, b: &mut Body, dt: f32) -> bool {
        let mut wall = false;
        let nx = b.x + b.vx * dt;
        if self.blocked_at(nx, b.y, b.w, b.h) {
            // an die Wand heranschieben
            let dir = b.vx.signum();
            let mut x = b.x;
            for _ in 0..32 {
                if self.blocked_at(x + dir, b.y, b.w, b.h) {
                    break;
                }
                x += dir;
            }
            b.x = x;
            b.vx = 0.0;
            wall = true;
        } else {
            b.x = nx;
        }
        let ny = b.y + b.vy * dt;
        b.ground = false;
        if b.vy >= 0.0 {
            // fallen: auf Fels oder (ohne Durchfallen) auf Planken und Leiterköpfen landen
            let (tx0, tx1) = (((b.x - b.w * 0.5) / TS).floor() as i32, ((b.x + b.w * 0.5 - 0.01) / TS).floor() as i32);
            let (r0, r1) = ((b.y / TS).floor() as i32, (ny / TS).floor() as i32);
            for ty in r0..=r1 {
                let top = ty as f32 * TS;
                if top < b.y - 0.01 || top > ny {
                    continue;
                }
                let hit = (tx0..=tx1).any(|tx| self.solid(tx, ty) || (!b.drop && self.oneway(tx, ty)));
                if hit {
                    b.y = top;
                    b.vy = 0.0;
                    b.ground = true;
                    return wall;
                }
            }
            b.y = ny;
        } else if self.blocked_at(b.x, ny, b.w, b.h) {
            b.y = ((ny - b.h) / TS).floor() * TS + TS + b.h;
            b.vy = 0.0;
        } else {
            b.y = ny;
        }
        wall
    }

    // ------------------------------------------------------------------ Zeichnen
    /// Kulisse hinter der Karte: Himmel mit Aschesonne, ferne Klippen, die Galeone, nahe Klippen.
    pub fn draw_back(&self, art: &Art, f: &mut Frame, t: f32) {
        let (vx, vy, vw, vh) = f.view();
        // Himmel: harte Farbstufen von Pechschwarz zu glühendem Aschegrau
        let bands: [[u8; 3]; 6] = [[34, 28, 38], [56, 42, 52], [84, 58, 62], [118, 76, 70], [156, 96, 78], [196, 120, 86]];
        for (i, c) in bands.iter().enumerate() {
            let y = vy + vh * i as f32 / 6.0;
            f.world_q(-10.0, Frame::quad_rect(vx - 2.0, y, vw + 4.0, vh / 6.0 + 1.0, [c[0], c[1], c[2], 255], modes::SOLID, flags::GLOW));
        }
        // verfinsterte Sonne als Pixelscheibe in der Kulisse (so verdeckt der Fels sie):
        // Glutkranz in zwei Stufen, davor die schwarze Scheibe
        let (sx, sy) = (vx + vw * 0.78 - vx * 0.02, vy + vh * 0.2 - vy * 0.02);
        let pulse = 1.0 + (t * 0.8).sin() * 0.04;
        for (r, c) in [(52.0 * pulse, [236u8, 120, 70]), (42.0, [255, 170, 110]), (34.0, [10, 8, 12])] {
            let mut y = -r;
            while y < r {
                let w = (r * r - (y + 1.0) * (y + 1.0)).max(0.0).sqrt();
                f.world_q(-9.8, Frame::quad_rect(sx - w, sy + y, w * 2.0, 2.0, [c[0], c[1], c[2], 255], modes::SOLID, flags::GLOW));
                y += 2.0;
            }
        }
        // Klippen in drei Tiefen (Faktor = wie stark sie der Kamera folgen)
        for (layer, k, col, step, amp) in [(0, 0.12, [112u8, 78, 74], 48.0, 140.0), (1, 0.3, [74, 52, 54], 40.0, 110.0)] {
            self.cliffs(f, -9.0 + layer as f32 * 0.5, k, col, step, amp, layer);
        }
        // die gestrandete Galeone, schräg in der Ascheklippe
        let k = 0.35;
        let (gx, gy) = (vx * k + 900.0, vy * k + 400.0);
        let s = art.spr("hw:fire");
        f.world_q(-8.2, Frame::quad_spr(&s, gx, gy, &SprOpt { scale: 1.6, rot: -0.12, tint: [190, 160, 150, 255], flags: flags::GLOW, ..Default::default() }));
        let s2 = art.spr("hw:earth");
        f.world_q(-8.3, Frame::quad_spr(&s2, gx - 760.0, gy + 60.0, &SprOpt { scale: 1.1, rot: 0.2, flip: true, tint: [150, 130, 128, 255], flags: flags::GLOW, ..Default::default() }));
        f.light(LightDef { x: gx, y: gy - 60.0, z: 60.0, r: 260.0, col: [1.4, 0.5, 0.18], flicker: 0.4 });
        self.cliffs(f, -8.0, 0.5, [40, 30, 32], 36.0, 70.0, 2);
    }

    fn cliffs(&self, f: &mut Frame, sort: f32, k: f32, col: [u8; 3], step: f32, amp: f32, seed: i32) {
        let (vx, vy, vw, vh) = f.view();
        let base = vy * k + 430.0 - seed as f32 * 30.0;
        let x0 = ((vx - vx * k) / step).floor() as i32 - 1;
        let n = (vw / step) as i32 + 3;
        for i in x0..x0 + n {
            let hgt = (h01(i, seed, 17) * 0.6 + h01(i / 3, seed, 5) * 0.4) * amp + 30.0;
            let x = i as f32 * step + vx * k;
            let top = base - hgt;
            if top < vy + vh {
                f.world_q(sort, Frame::quad_rect(x, top, step + 1.0, vy + vh - top + 2.0, [col[0], col[1], col[2], 255], modes::SOLID, flags::GLOW));
            }
        }
    }

    /// Kacheln im Bild: Fels, Rückwand, Planken, Leitern, Winde.
    pub fn draw_tiles(&self, art: &Art, f: &mut Frame) {
        let (vx, vy, vw, vh) = f.view();
        let (tx0, ty0) = ((vx / TS).floor() as i32, (vy / TS).floor() as i32);
        let (tx1, ty1) = (((vx + vw) / TS).ceil() as i32, ((vy + vh) / TS).ceil() as i32);
        for ty in ty0..=ty1 {
            for tx in tx0..=tx1 {
                let (x, y) = (tx as f32 * TS, ty as f32 * TS);
                let v = (h01(tx, ty, 3) * 3.0) as i32 % 3;
                let c = self.cell(tx, ty);
                let tile = |f: &mut Frame, name: &str, sort: f32| f.world_q(sort, Frame::quad_spr(&art.spr(name), x, y, &SprOpt::default()));
                match c {
                    b'#' => {
                        let top = !self.solid(tx, ty - 1);
                        tile(f, &format!("s:{}{v}", if top { "top" } else { "rock" }), 0.0);
                    }
                    b' ' => {}
                    _ => {
                        // Rückwand hinter Höhlen (auch hinter Leitern und Dingen in Höhlen)
                        let cave = c == b'.' || [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().any(|(dx, dy)| self.cell(tx + dx, ty + dy) == b'.');
                        if cave {
                            tile(f, &format!("s:back{v}"), -1.0);
                        }
                        match c {
                            b'=' => tile(f, &format!("s:plank{v}"), 0.1),
                            b'H' => tile(f, "s:ladder", 0.1),
                            // Kettenleiter: zusammengerollt oben, bis die Winde gedreht ist
                            b'L' if self.shortcut || self.cell(tx, ty - 1) != b'L' => tile(f, "s:chain", 0.1),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
