//! Erzeugt beim Start alle Grafiken von Eclipse Ashes (prozedural) und packt sie in Atlas-Seiten:
//! Schrift, Figuren mit allen Animationen, Wesen aus dem Kreaturen-Rig, Kacheln der Seitenansicht,
//! Requisiten und die verkohlten Wracks aus „Throne of the Drowned“ als Kulisse.

use crate::gfx::{Frame, GfxInit, Tex, Upload};
use bevy::prelude::*;
use std::collections::HashMap;
use sw_art::atlas::{Atlas, Spr, PAGE};
use sw_art::canvas::Canvas;
use sw_art::chars;
use sw_art::props::{self as pr, PropArt};
use sw_art::side;

#[derive(Clone)]
pub struct PropDef {
    pub spr: Spr,
    /// Licht relativ zum Fuß: dx, dy, Höhe, Radius · Farbe · Flackern
    pub light: Option<([f32; 4], [f32; 3], f32)>,
}


#[derive(Clone, Default)]
pub struct CharSheet {
    /// [Richtung][Animation][Bild]
    pub f: Vec<Vec<Vec<Spr>>>,
    pub s: f32,
    /// Schattenradius (x) am Boden
    pub shadow: f32,
}
impl CharSheet {
    pub fn get(&self, dir: usize, anim: usize, frame: usize) -> Spr {
        let a = &self.f[dir.min(2)];
        let list = if a[anim].is_empty() { &a[0] } else { &a[anim] };
        list[frame % list.len()]
    }
    pub fn len(&self, anim: usize) -> usize {
        let l = self.f[0][anim].len();
        if l == 0 {
            self.f[0][0].len()
        } else {
            l
        }
    }
}

pub fn anim_index(name: &str) -> usize {
    chars::ANIMS.iter().position(|a| a.0 == name).unwrap_or(0)
}

#[derive(Resource)]
pub struct Art {
    pub map: HashMap<String, Spr>,
    pub props: HashMap<String, PropDef>,
    pub chars: HashMap<String, CharSheet>,
    pub glyphs: HashMap<char, (Spr, i32)>,
    pub player: CharSheet,
}

impl Art {
    pub fn spr(&self, name: &str) -> Spr {
        self.map.get(name).copied().unwrap_or_else(|| {
            warn!("Sprite fehlt: {name}");
            Spr { page: 0, x: 0, y: 0, w: 1, h: 1, ax: 0, ay: 0 }
        })
    }
}

/// Verkohlt ein Schiffsbild: Holz und Tuch schwarzbraun, Segel mit Brandlöchern und glühenden
/// Rändern, Glutnester im Rumpf.
fn char_ship(c: &mut Canvas, el: &str) {
    let nz = sw_art::noise::Noise::new(77 + el.len() as u32 * 13);
    let (w, h) = (c.w, c.h);
    let alpha = c.col.iter().map(|p| p[3]).collect::<Vec<_>>();
    for y in 0..h {
        for x in 0..w {
            let i = c.idx(x, y);
            if alpha[i] == 0 {
                continue;
            }
            let p = c.col[i];
            let ev = c.ev[i];
            let n = nz.fbm(x as f32 * 0.045, y as f32 * 0.045, 3);
            let lum = (p[0] as f32 * 0.3 + p[1] as f32 * 0.5 + p[2] as f32 * 0.2) / 255.0;
            // hohe, helle Flächen (Segel): Brandlöcher mit glühendem Saum
            if ev > 70.0 && lum > 0.42 {
                if n > 0.12 {
                    c.clear(x, y);
                    continue;
                }
                if n > 0.04 {
                    c.col[i] = [70, 26, 10, 255];
                    c.glow(x, y, [255, 120, 40, 255], 0.5 + (n - 0.04) * 4.0);
                    continue;
                }
            }
            // verkohlen: dunkel, warm, Ruß in Flecken
            let soot = (0.5 - n * 0.6).clamp(0.22, 0.62);
            let k = |v: u8, add: f32| ((v as f32 * soot + add) as f32).min(255.0) as u8;
            c.col[i] = [k(p[0], 10.0), k(p[1], 5.0), k(p[2], 3.0), p[3]];
            // Glutnester: glimmende Risse im Holz
            let e = nz.ridged(x as f32 * 0.09 + 3.0, y as f32 * 0.09, 2);
            if e > 0.95 && ev < 60.0 && sw_art::noise::h01(x, y, 5) < 0.5 {
                let g = (e - 0.95) * 12.0;
                c.col[i] = [150, 50, 14, 255];
                c.glow(x, y, [255, 110 + (g * 80.0) as u8, 30, 255], 0.6 + g);
            }
        }
    }
}

fn add_prop(a: &mut Atlas, props: &mut HashMap<String, PropDef>, pa: PropArt) {
    let spr = a.add_trim(&format!("p:{}", pa.name), &pa.c, pa.ax, pa.ay);
    props.insert(pa.name.clone(), PropDef { spr, light: pa.light });
}

fn sheet(a: &mut Atlas, key: &str, l: &chars::Look, s: f32, role: chars::Role) -> CharSheet {
    let mut f = vec![];
    for dir in chars::DIRS {
        let mut per = vec![];
        for (anim, n) in chars::ANIMS {
            let mut v = vec![];
            let wanted = chars::wants_anim(role, anim);
            if wanted && !(dir != "down" && anim == "dead") {
                for i in 0..n {
                    let c = chars::frame(l, dir, anim, i, s);
                    let ay = c.h - (4.0 * s) as i32;
                    v.push(a.add_trim(&format!("c:{key}:{dir}:{anim}:{i}"), &c, c.w / 2, ay));
                }
            }
            per.push(v);
        }
        f.push(per);
    }
    // Tod nur von vorn: für die anderen Richtungen übernehmen
    if sw_art::beasts::sea_boss(key) {
        f[0] = f[2].clone();
        f[1] = f[2].clone();
    }
    let dead = anim_index("dead");
    for d in 1..3 {
        if f[d][dead].is_empty() {
            f[d][dead] = f[0][dead].clone();
        }
    }
    CharSheet { f, s, shadow: 15.0 * s }
}

/// Bilderbogen eines Wesens aus dem Kreaturen-Rig (gleiche Richtungen und Animationen wie Figuren).
/// Große Wesen teilen sich gleiche Bilder (`beasts::canon`), See-Wesen haben nur eine Richtung.
fn beast_sheet(a: &mut Atlas, key: &str) -> CharSheet {
    use sw_art::beasts;
    let kd = beasts::kind(key);
    let mut cache: HashMap<(&str, &str, usize), Spr> = HashMap::new();
    let mut f = vec![];
    for (di, dir) in chars::DIRS.iter().enumerate() {
        let mut per = vec![];
        for (anim, n) in chars::ANIMS {
            let mut v = vec![];
            let wanted = if kd.only.is_some() {
                anim == "idle"
            } else if beasts::sea_boss(key) {
                // Seebosse: nur Seitenansicht, kein Gehen (fällt auf Stehen zurück)
                di == 2 && matches!(anim, "idle" | "atk" | "hurt")
            } else {
                matches!(anim, "idle" | "walk" | "atk" | "hurt") || (anim == "dead" && di == 0) || (anim == "shoot" && kd.shoot)
            };
            if wanted {
                for i in 0..n {
                    let ci = beasts::canon(key, anim, i);
                    let rd: &str = if anim == "dead" { "down" } else { kd.only.unwrap_or(dir) };
                    let spr = *cache.entry((rd, anim, ci)).or_insert_with(|| {
                        let c = beasts::beast_frame(key, rd, anim, ci);
                        a.add_trim(&format!("k:{key}:{rd}:{anim}:{ci}"), &c, c.w / 2, c.h - kd.foot)
                    });
                    v.push(spr);
                }
            }
            per.push(v);
        }
        f.push(per);
    }
    if beasts::sea_boss(key) {
        f[0] = f[2].clone();
        f[1] = f[2].clone();
    }
    let dead = anim_index("dead");
    for d in 1..3 {
        if f[d][dead].is_empty() {
            f[d][dead] = f[0][dead].clone();
        }
    }
    CharSheet { f, s: 1.0, shadow: kd.shadow }
}

pub fn build(frame: &mut Frame) -> Art {
    let t0 = bevy::platform::time::Instant::now();
    let mut a = Atlas::new();
    let mut props: HashMap<String, PropDef> = HashMap::new();
    let mut c = Canvas::new(4, 4);
    c.rect(0, 0, 4, 4, sw_art::color::hex(0xFFFFFF));
    a.add("white", &c, 0, 0);

    // ---------------------------------------------------------------- Schrift
    let mut glyphs = HashMap::new();
    for (ch, _) in sw_art::font::GLYPHS {
        if let Some((g, w)) = sw_art::font::glyph(*ch) {
            let s = a.add(&format!("f:{}", *ch as u32), &g, 0, 0);
            glyphs.insert(*ch, (s, w));
        }
    }
    a.add("ui:logo", &sw_art::ui::logo("ECLIPSE ASHES"), 0, 0);

    // ---------------------------------------------------------------- Kacheln der Seitenansicht
    for v in 0..3 {
        a.add(&format!("s:rock{v}"), &side::rock(v, false, false), 0, 0);
        a.add(&format!("s:top{v}"), &side::rock(v, true, false), 0, 0);
        a.add(&format!("s:back{v}"), &side::rock(v, false, true), 0, 0);
        a.add(&format!("s:plank{v}"), &side::plank(v), 0, 0);
    }
    a.add("s:ladder", &side::ladder(false), 0, 0);
    a.add("s:chain", &side::ladder(true), 0, 0);
    for (n, w) in [("s:winch0", false), ("s:winch1", true)] {
        let c = side::winch(w);
        a.add(n, &c, c.w / 2, c.h);
    }

    // ---------------------------------------------------------------- Requisiten
    let mut list = vec![pr::saberfire(), pr::torch(), pr::skull_pile(), pr::barrels(), pr::crates(), pr::cannon(), pr::anchor_deco()];
    for i in 0..3 {
        list.push(sw_art::structures::sword_grave(i));
        list.push(sw_art::structures::candles(i));
    }
    for pa in list {
        add_prop(&mut a, &mut props, pa);
    }
    // gestrandete Galeonen aus „Throne of the Drowned“: verkohlt, in die Ascheklippen gerammt
    for (el, k) in [("fire", 6usize), ("earth", 2)] {
        let (_, h, foot) = sw_art::ships::ship_size(el);
        let mut c = sw_art::ships::ship_frame(el, k, Default::default());
        char_ship(&mut c, el);
        a.add_trim(&format!("hw:{el}"), &c, c.w / 2, h - foot);
    }

    // ---------------------------------------------------------------- Figuren und Wesen
    let mut chs = HashMap::new();
    for name in ["ashSkeleton", "skelBrute"] {
        let (l, s) = chars::preset(name);
        chs.insert(name.to_string(), sheet(&mut a, name, &l, s, chars::Role::Foe));
    }
    // Kapitän Barnaby: der Ork-Kapitän, anderthalbmal so groß
    let (l, s) = chars::preset("orcCaptain");
    chs.insert("barnaby".to_string(), sheet(&mut a, "barnaby", &l, s * 1.5, chars::Role::Foe));
    chs.insert("crab".to_string(), beast_sheet(&mut a, "crab"));
    // Miniboss: der Musketen-Skelett-Schütze, größer
    let (l, s) = chars::preset("skelGunner");
    chs.insert("gunner".to_string(), sheet(&mut a, "gunner", &l, s * 1.4, chars::Role::Foe));
    let player = sheet(&mut a, "player", &chars::player_look(&[0, 1, 1, 5, 0, 0, 4, 0], "cutlass"), 1.0, chars::Role::Player);

    let pages = a.pages.len() as u32;
    info!("Grafik erzeugt in {:?} · {} Atlasseiten", t0.elapsed(), pages);
    // die Wassermaske des Renderers bleibt ungenutzt (keine See mehr): kleinste Größe
    frame.init = Some(GfxInit { atlas_pages: pages, mask_w: 8, mask_h: 8 });
    // Seiten in den Upload verschieben statt kopieren (Wasm-Speicher wächst nur)
    let map = std::mem::take(&mut a.map);
    for (i, pg) in a.pages.into_iter().enumerate() {
        frame.uploads.push(Upload { tex: Tex::AtlasCol, layer: i as u32, x: 0, y: 0, w: PAGE as u32, h: PAGE as u32, data: pg.col });
        frame.uploads.push(Upload { tex: Tex::AtlasNrm, layer: i as u32, x: 0, y: 0, w: PAGE as u32, h: PAGE as u32, data: pg.nrm });
    }
    Art { map, props, chars: chs, glyphs, player }
}
