//! Eclipse Ashes: Spielablauf. Titel, Spiel, Pause und Schrein-Menü; Ascheschrein (Rasten,
//! Katha: Stufen, Corvus: Aschenkunst und Prägung), Tod und Aschefleck, Winden-Abkürzung,
//! Nebeltor und Bosskampf, Kamera, Spielstand im Browser.

pub mod env;
pub mod fight;
pub mod fx;
mod hud;
pub mod input;
mod level;
#[cfg(test)]
mod tests;

use crate::art::{anim_index, Art};
use crate::gfx::{flags, Frame, LightDef, SprOpt};
use fight::*;
use fx::{rnd, BurstOpt, Fx};
use input::{Act, Input};
use level::{Body, Level, TS};
use serde::{Deserialize, Serialize};

const SAVE_KEY: &str = "eclipse-ashes";

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Save {
    pub vit: i32,
    pub end: i32,
    pub str: i32,
    pub ash: i64,
    /// freigeschaltete Aschenkünste (Schlüssel) und die gewählte
    pub arts: Vec<String>,
    pub art: usize,
    /// Prägung der Waffe (Index in `AFFS`)
    pub aff: usize,
    pub flasks: i32,
    pub shortcut: bool,
    pub boss: bool,
    pub miniboss: bool,
    /// aufgehobene Dinge (Kartenzeichen)
    pub picked: String,
    /// Aschefleck: x, y, Menge
    pub drop: Option<(f32, f32, i64)>,
    pub deaths: i32,
    /// Spielzeit in Sekunden
    pub time: f64,
}
impl Default for Save {
    fn default() -> Self {
        Save { vit: 0, end: 0, str: 0, ash: 0, arts: vec!["wirbel".into()], art: 0, aff: 0, flasks: 3, shortcut: false, boss: false, miniboss: false, picked: String::new(), drop: None, deaths: 0, time: 0.0 }
    }
}
impl Save {
    pub fn level(&self) -> i32 {
        1 + self.vit + self.end + self.str
    }
    pub fn level_cost(&self) -> i64 {
        (80.0 * 1.12f64.powi(self.level() - 1)).round() as i64
    }
}

fn store_get() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()?.local_storage().ok()??.get_item(SAVE_KEY).ok()?
    }
    #[cfg(not(target_arch = "wasm32"))]
    None
}
fn store_set(v: &str) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let s = web_sys::window().and_then(|w| w.local_storage().ok().flatten()).ok_or("Kein Speicher im Browser verfügbar.")?;
        s.set_item(SAVE_KEY, v).map_err(|_| "Speichern fehlgeschlagen (Speicher voll oder privat).".to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = v;
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mode {
    Title,
    Play,
    Pause,
    Controls,
    Shrine,
    /// Abschluss nach dem Sieg über den Zonenboss
    End,
}

pub struct Game {
    pub mode: Mode,
    pub sel: usize,
    pub lv: Level,
    pub save: Save,
    pub p: Player,
    pub foes: Vec<Foe>,
    pub boss: Boss,
    pub shots: Vec<Shot>,
    /// Dinge zum Aufheben: Kartenzeichen, x, y
    pub pickups: Vec<(char, f32, f32)>,
    pub shrine: (f32, f32),
    pub winch: (f32, f32),
    pub gate_x: f32,
    pub torches: Vec<(f32, f32)>,
    pub cam: (f32, f32),
    pub shake: f32,
    pub hitstop: f32,
    pub t: f32,
    pub input: Input,
    pub fx: Fx,
    pub sfx_q: Vec<&'static str>,
    pub mood: &'static str,
    pub banner: Option<(String, String, f32)>,
    pub toast: Option<(String, f32)>,
    pub death_t: Option<f32>,
    pub has_save: bool,
    /// Zeit seit dem Bosssieg (bis zum Abschluss-Bildschirm)
    pub end_t: Option<f32>,
    /// Modus beim letzten Zeichnen der Oberfläche und wie viele Bilder er schon steht: ein Menü
    /// nimmt erst ab dem zweiten Bild Bestätigungen an (sonst schließt das E vom Rasten es sofort)
    pub ui_mode: Mode,
    pub ui_frames: u32,
    env: env::Env,
}

impl Game {
    pub fn new() -> Self {
        let lv = Level::new();
        let at = |ch: u8| lv.find(ch).first().map(|&(x, y)| (x as f32 * TS + TS * 0.5, (y + 1) as f32 * TS)).unwrap_or((0.0, 0.0));
        let (shrine, winch, start, boss_at) = (at(b'S'), at(b'W'), at(b'P'), at(b'B'));
        let gate_x = lv.find(b'D').first().map(|&(x, _)| x as f32 * TS).unwrap_or(0.0);
        let torches = lv.find(b't').iter().map(|&(x, y)| (x as f32 * TS + TS * 0.5, (y + 1) as f32 * TS)).collect();
        let save = Save::default();
        let mut g = Game {
            mode: Mode::Title,
            sel: 0,
            lv,
            p: new_player(start, &save),
            save,
            foes: vec![],
            boss: Boss::new(boss_at.0, boss_at.1),
            shots: vec![],
            pickups: vec![],
            shrine,
            winch,
            gate_x,
            torches,
            cam: shrine,
            shake: 0.0,
            hitstop: 0.0,
            t: 0.0,
            input: Input::new(),
            fx: Fx::default(),
            sfx_q: vec![],
            mood: "title",
            banner: None,
            toast: None,
            death_t: None,
            has_save: store_get().is_some(),
            end_t: None,
            ui_mode: Mode::Title,
            ui_frames: 0,
            env: env::ash(),
        };
        g.reset_zone();
        g
    }

    /// Spiel beginnen: neu oder mit gespeichertem Stand
    pub fn start(&mut self, cont: bool) {
        self.save = if cont { store_get().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default() } else { Save::default() };
        if self.save.arts.is_empty() {
            self.save.arts = vec!["wirbel".into()];
        }
        self.lv.shortcut = self.save.shortcut;
        let start = if cont { self.shrine } else { self.lv.find(b'P').first().map(|&(x, y)| (x as f32 * TS + TS * 0.5, (y + 1) as f32 * TS)).unwrap_or(self.shrine) };
        self.p = new_player(start, &self.save);
        self.reset_zone();
        self.cam = start;
        self.mode = Mode::Play;
        self.death_t = None;
        self.mood = "glut";
        if !cont {
            self.banner("DIE ASCHENEN KLIPPEN VON CALAMITA", "Wo einst das Meer war, liegt nur noch Asche", 4.5);
        }
    }

    /// Gegner, Boss und Dinge auf den Anfang setzen (Rasten, Tod)
    fn reset_zone(&mut self) {
        self.foes.clear();
        for (ch, kind) in [(b'm', Kind::Marauder), (b'g', Kind::Grabber), (b'c', Kind::Crawler), (b'G', Kind::Gunner)] {
            if kind == Kind::Gunner && self.save.miniboss {
                continue;
            }
            for (x, y) in self.lv.find(ch) {
                self.foes.push(Foe::new(kind, x as f32 * TS + TS * 0.5, (y + 1) as f32 * TS));
            }
        }
        let home = self.boss.home;
        self.boss = Boss::new(home.0, home.1);
        if self.save.boss {
            self.boss.state = BS::Dead;
        }
        self.lv.gate_closed = false;
        self.shots.clear();
        self.pickups = ['1', '2'].iter().filter(|c| !self.save.picked.contains(**c)).flat_map(|c| self.lv.find(*c as u8).into_iter().map(move |(x, y)| (*c, x as f32 * TS + TS * 0.5, (y + 1) as f32 * TS))).collect();
    }

    pub fn persist(&mut self) {
        self.save.shortcut = self.lv.shortcut;
        match serde_json::to_string(&self.save).map_err(|e| e.to_string()).and_then(|s| store_set(&s)) {
            Ok(()) => self.has_save = true,
            Err(e) => self.toast(&format!("Speichern fehlgeschlagen: {e}")),
        }
    }
    pub fn gain_ash(&mut self, n: i64) {
        self.save.ash += n;
    }
    pub fn toast(&mut self, s: &str) {
        self.toast = Some((s.to_string(), 2.4));
    }
    pub fn banner(&mut self, a: &str, b: &str, secs: f32) {
        self.banner = Some((a.to_string(), b.to_string(), secs));
    }

    pub fn player_died(&mut self) {
        if self.death_t.is_some() {
            return;
        }
        self.p.hp = 0.0;
        self.p.state = PS::Dead;
        self.save.deaths += 1;
        // der alte Fleck verweht, die neue Asche bleibt hier liegen
        self.save.drop = if self.save.ash > 0 { Some((self.p.b.x, self.p.b.y, self.save.ash)) } else { None };
        self.save.ash = 0;
        self.death_t = Some(0.0);
        self.mood = "death";
        self.banner("ZU ASCHE ZERFALLEN", "", 3.2);
        self.sfx_q.push("death");
    }
    fn respawn(&mut self) {
        self.death_t = None;
        self.p = new_player(self.shrine, &self.save);
        self.reset_zone();
        self.cam = self.shrine;
        self.mood = "glut";
        self.persist();
    }
    fn rest(&mut self) {
        self.p.hp = self.max_hp();
        self.p.st = self.max_st();
        self.p.fp = self.max_fp();
        self.p.flasks = self.save.flasks;
        self.reset_zone();
        self.persist();
        self.mode = Mode::Shrine;
        self.sel = 0;
        self.sfx_q.push("kindle");
    }

    /// Was mit E gerade möglich ist (Text für den Hinweis)
    pub fn interaction(&self) -> Option<&'static str> {
        let (x, y) = (self.p.b.x, self.p.b.y);
        let near = |q: (f32, f32), r: f32| (q.0 - x).abs() < r && (q.1 - y).abs() < 20.0;
        if self.p.state != PS::Free {
            return None;
        }
        if near(self.shrine, 40.0) {
            Some("E  Am Ascheschrein rasten")
        } else if near(self.winch, 40.0) && !self.lv.shortcut {
            Some("E  Winde drehen")
        } else if self.pickups.iter().any(|p| near((p.1, p.2), 30.0)) {
            Some("E  Aufheben")
        } else if self.exec_target().is_some() {
            Some("J  Hinrichten")
        } else {
            None
        }
    }
    fn interact(&mut self) {
        let (x, y) = (self.p.b.x, self.p.b.y);
        let near = |q: (f32, f32), r: f32| (q.0 - x).abs() < r && (q.1 - y).abs() < 20.0;
        if near(self.shrine, 40.0) {
            self.rest();
        } else if near(self.winch, 40.0) && !self.lv.shortcut {
            self.lv.shortcut = true;
            self.sfx_q.push("chain");
            self.shake = 4.0;
            self.banner("ABKÜRZUNG GEÖFFNET", "Die Kettenleiter rasselt hinab zum Schrein", 3.0);
            self.persist();
        } else if let Some(i) = self.pickups.iter().position(|p| near((p.1, p.2), 30.0)) {
            let (ch, px, py) = self.pickups.remove(i);
            let key = if ch == '1' { "stoss" } else { "schild" };
            if !self.save.arts.iter().any(|a| a == key) {
                self.save.arts.push(key.into());
            }
            self.save.picked.push(ch);
            let a = art_def(key);
            self.banner(&format!("KRIEGSASCHE: {}", a.name.to_uppercase()), "Corvus am Schrein bindet sie an deine Klinge", 3.0);
            self.sfx_q.push("level");
            self.fx.burst("soulLight", px, py, 20, BurstOpt { speed: Some(120.0), up: Some(20.0), ..Default::default() });
            self.persist();
        }
    }

    // ================================================================== Ablauf
    pub fn update(&mut self, dt: f32) {
        self.t += dt;
        if let Some(b) = self.banner.as_mut() {
            b.2 -= dt;
            if b.2 <= 0.0 {
                self.banner = None;
            }
        }
        if let Some(b) = self.toast.as_mut() {
            b.1 -= dt;
            if b.1 <= 0.0 {
                self.toast = None;
            }
        }
        match self.mode {
            Mode::Title | Mode::Pause | Mode::Controls | Mode::Shrine | Mode::End => {
                self.fx.update(dt);
                return;
            }
            Mode::Play => {}
        }
        self.save.time += dt as f64;
        if let Some(t) = self.end_t.as_mut() {
            *t += dt;
            if *t > 7.0 {
                self.end_t = None;
                self.mode = Mode::End;
                self.sel = 0;
                return;
            }
        }
        // Treffer-Stopp: wenige Bilder fast angehalten, damit Treffer Gewicht haben
        let dt = if self.hitstop > 0.0 {
            self.hitstop -= dt;
            dt * 0.15
        } else {
            dt
        };
        if let Some(t) = self.death_t.as_mut() {
            *t += dt;
            if *t > 3.4 {
                self.respawn();
                return;
            }
        }
        self.update_player(dt);
        if self.input.pressed(Act::Interact) && self.death_t.is_none() {
            self.interact();
        }
        // Aschefleck aufheben
        if let Some((x, y, n)) = self.save.drop {
            if (self.p.b.x - x).abs() < 24.0 && (self.p.b.y - y).abs() < 40.0 && self.death_t.is_none() {
                self.save.drop = None;
                self.gain_ash(n);
                self.toast(&format!("Asche zurückgeholt: {n}"));
                self.sfx_q.push("pearls");
            }
        }
        // Nebeltor: wer hindurchgeht, kämpft
        // erst schließen, wenn der Spieler ganz durch ist (sonst steckt er im Tor fest)
        if self.boss.state == BS::Dormant && self.p.b.x - self.p.b.w * 0.5 > self.gate_x + TS + 2.0 && self.death_t.is_none() {
            self.lv.gate_closed = true;
            self.boss.state = BS::Intro;
            self.boss.t = 0.0;
            self.mood = "boss";
            self.sfx_q.push("fog");
            self.banner(BOSS_NAME, "", 2.4);
        }
        if self.boss.state == BS::Dead && self.mood == "boss" {
            self.mood = "glut";
        }
        self.update_foes(dt);
        self.update_boss(dt);
        self.update_shots(dt);
        self.fx.ambient(dt, self.view_rect(), 5, false);
        self.fx.update(dt);
        self.update_camera(dt);
    }

    fn view_rect(&self) -> (f32, f32, f32, f32) {
        (self.cam.0 - 520.0, self.cam.1 - 320.0, 1040.0, 640.0)
    }

    fn update_camera(&mut self, dt: f32) {
        let (lw, lh) = self.lv.size();
        let mut tx = self.p.b.x + self.p.face * 40.0;
        let ty = self.p.b.y - 60.0;
        // im Bosskampf hält die Kamera Arena und Spieler im Bild (auch auf schmalen Bildschirmen)
        if self.boss.active() {
            tx = self.p.b.x * 0.6 + (self.boss.home.0 - 60.0) * 0.4;
        }
        let k = (dt * 6.0).min(1.0);
        self.cam.0 += (tx - self.cam.0) * k;
        self.cam.1 += (ty - self.cam.1) * k;
        let _ = (lw, lh);
        self.shake = (self.shake - dt * 30.0).max(0.0);
    }

    // ================================================================== Zeichnen
    pub fn draw(&mut self, art: &Art, f: &mut Frame, dark: bool) {
        // Kamera innerhalb der Karte, Wackeln bei Treffern
        let (lw, lh) = self.lv.size();
        let (vw, vh) = (f.low.0 as f32, f.low.1 as f32);
        let (sx, sy) = ((rnd() - 0.5) * self.shake, (rnd() - 0.5) * self.shake);
        let cx = if vw >= lw { lw * 0.5 } else { self.cam.0.clamp(vw * 0.5, lw - vw * 0.5) };
        let cy = if vh >= lh { lh * 0.5 } else { self.cam.1.clamp(vh * 0.5, lh - vh * 0.5) };
        f.set_camera(cx + sx, cy + sy);
        env::apply(&self.env, &mut f.g, dark, 1.0, true);
        // Seitenansicht: keine Diorama-Kippung, keine Tiefenschärfe, keine Sonnenschatten
        f.g.hd[0] = 0.0;
        f.g.hd[1] = 0.0;
        f.g.sun_dir[3] = 0.0;
        f.focus_y = None;
        self.lv.draw_back(art, f, self.t);
        self.lv.draw_tiles(art, f);
        self.draw_things(art, f);
        self.draw_actors(art, f);
        self.fx.draw(f);
        let (vx, vy, vw, vh) = f.view();
        f.fog(vx - 20.0, vy + vh * 0.55, vw + 40.0, vh * 0.5, [60, 44, 40, 70], 6.0);
        f.fog(vx - 20.0, vy - 20.0, vw + 40.0, vh + 40.0, [40, 34, 36, 40], 3.0);
    }

    fn draw_things(&self, art: &Art, f: &mut Frame) {
        let t = self.t;
        // Ascheschrein (Säbelfeuer) mit Licht
        if let Some(p) = art.props.get("saberfire") {
            f.world_q(0.5, Frame::quad_spr(&p.spr, self.shrine.0, self.shrine.1, &SprOpt::default()));
        }
        f.light(LightDef { x: self.shrine.0, y: self.shrine.1 + 4.0, z: 30.0, r: 230.0, col: [1.6, 0.95, 0.5], flicker: 0.6 });
        let w = art.spr(if self.lv.shortcut { "s:winch1" } else { "s:winch0" });
        f.world_q(0.5, Frame::quad_spr(&w, self.winch.0, self.winch.1, &SprOpt::default()));
        for &(x, y) in &self.torches {
            if let Some(p) = art.props.get("torch") {
                f.world_q(0.4, Frame::quad_spr(&p.spr, x, y, &SprOpt::default()));
            }
            f.light(LightDef { x, y: y + 6.0, z: 40.0, r: 150.0, col: [1.3, 0.8, 0.45], flicker: 0.7 });
        }
        // Deko: Schädel, Kerzen, Kanone und Fässer an festen Stellen
        for (name, tx, ty) in [("skullpile", 10, 36), ("candles1", 5, 36), ("barrels", 16, 29), ("cannon", 12, 23), ("crates", 6, 17), ("swordGrave0", 9, 11), ("crates", 40, 11), ("barrels", 47, 11), ("candles2", 7, 36)] {
            if let Some(p) = art.props.get(name) {
                f.world_q(0.3, Frame::quad_spr(&p.spr, tx as f32 * TS + TS * 0.5, (ty + 1) as f32 * TS, &SprOpt { tint: [190, 175, 170, 255], ..Default::default() }));
            }
        }
        // Kriegsasche zum Aufheben: schwebendes Seelenlicht
        for &(_, x, y) in &self.pickups {
            let yy = y - 18.0 + (t * 2.0).sin() * 3.0;
            f.glow(x, yy, 10.0, [95, 242, 200, 200]);
            f.glow(x, yy, 4.0, [220, 255, 240, 255]);
            f.light(LightDef { x, y: yy + 20.0, z: 20.0, r: 90.0, col: [0.4, 1.4, 1.1], flicker: 0.2 });
        }
        // Aschefleck
        if let Some((x, y, _)) = self.save.drop {
            f.glow(x, y - 10.0, 9.0 + (t * 3.0).sin() * 2.0, [255, 140, 70, 220]);
            f.light(LightDef { x, y: y + 10.0, z: 20.0, r: 80.0, col: [1.5, 0.6, 0.25], flicker: 0.3 });
        }
        // Nebeltor: leuchtender Schleier, solange der Boss lebt
        if self.boss.state != BS::Dead {
            let gx = self.gate_x + TS * 0.5;
            let (gy0, gy1) = (9.0 * TS, 12.0 * TS);
            for k in 0..3 {
                let a = if self.lv.gate_closed { 120 } else { 60 };
                let wob = (t * 1.3 + k as f32).sin() * 4.0;
                f.over(Frame::quad_rect(gx - 10.0 + wob + k as f32 * 4.0, gy0, 8.0, gy1 - gy0, [210, 200, 190, a], crate::gfx::modes::SOFT, 0), true);
            }
            f.light(LightDef { x: gx, y: gy1 - 20.0, z: 40.0, r: 110.0, col: [0.9, 0.9, 1.0], flicker: 0.2 });
        }
        // Teerbomben (Marke, Fall) und brennender Teer
        let gy = self.boss.home.1;
        for h in &self.boss.hazards {
            if !h.fell {
                let k = h.t;
                f.over(Frame::quad_rect(h.x - 30.0, gy - 6.0, 60.0, 12.0, [255, 80, 30, (60.0 + k * 140.0) as u8], crate::gfx::modes::SOFT, 0), true);
                let fy = gy - 420.0 * (1.0 - k).max(0.0);
                f.over(Frame::quad_rect(h.x - 9.0, fy - 18.0, 18.0, 18.0, [20, 14, 12, 255], crate::gfx::modes::SOFT, 0), false);
                f.glow(h.x, fy - 9.0, 14.0, [255, 120, 40, 200]);
            }
        }
        for &(x, rest) in &self.boss.flames {
            let a = (rest.min(1.0) * 200.0) as u8;
            f.over(Frame::quad_rect(x - 32.0, gy - 22.0, 64.0, 26.0, [255, 110, 40, a], crate::gfx::modes::SOFT, 0), true);
            f.light(LightDef { x, y: gy + 10.0, z: 20.0, r: 90.0, col: [1.6, 0.6, 0.2], flicker: 0.8 });
        }
        // Anker an der Kette
        if let Some((ax, _, _, _)) = self.boss.anchor {
            let ay = self.boss.b.y - 40.0;
            let (x0, x1) = (self.boss.b.x.min(ax), self.boss.b.x.max(ax));
            f.world_q(1.7, Frame::quad_rect(x0, ay - 1.5, x1 - x0, 3.0, [70, 64, 70, 255], crate::gfx::modes::SOLID, 0));
            if let Some(p) = art.props.get("anchorDeco") {
                f.world_q(1.75, Frame::quad_spr(&p.spr, ax, ay + 20.0, &SprOpt { rot: self.t * 12.0, scale: 0.7, ..Default::default() }));
            }
        }
        // Kanonenkugeln
        for s in &self.shots {
            f.over(Frame::quad_rect(s.0 - 6.0, s.1 - 6.0, 12.0, 12.0, [24, 20, 22, 255], crate::gfx::modes::SOFT, 0), false);
            f.glow(s.0, s.1, 8.0, [255, 120, 50, 120]);
        }
    }

    fn draw_actors(&self, art: &Art, f: &mut Frame) {
        for fo in &self.foes {
            let d = foe_def(fo.kind);
            let Some(sh) = art.chars.get(d.sheet) else { continue };
            let (anim, frame) = match fo.state {
                FS::Hang | FS::Idle => ("idle", fo.at * 2.0),
                FS::Chase if fo.b.vx.abs() > 1.0 => ("walk", fo.at * 9.0),
                FS::Chase | FS::Recover => ("idle", fo.at * 2.5),
                FS::Windup => (if fo.kind == Kind::Gunner { "shoot" } else { "atk" }, 1.0),
                FS::Strike => (if fo.kind == Kind::Gunner { "shoot" } else { "atk" }, 3.0),
                FS::Hurt | FS::Broken => ("hurt", 0.0),
                FS::Dead => ("dead", 0.0),
            };
            if fo.state == FS::Dead && fo.t > 1.6 {
                continue;
            }
            let dir = if anim == "dead" { 0 } else { 2 };
            let s = sh.get(dir, anim_index(anim), frame as usize);
            let fade = if fo.state == FS::Dead { (255.0 * (1.0 - fo.t / 1.6)).max(0.0) as u8 } else { 255 };
            let mut o = SprOpt { flip: fo.face < 0.0, flags: if fo.flash > 0.0 { flags::FLASH } else { 0 }, tint: [255, 255, 255, fade], ..Default::default() };
            let (mut x, mut y) = (fo.b.x, fo.b.y);
            if fo.state == FS::Hang {
                // kopfüber an der Decke
                o.rot = std::f32::consts::PI;
                y -= fo.b.h;
                x += 0.0;
            }
            if fo.state == FS::Windup {
                o.tint = [255, 200, 170, 255];
            }
            f.world_q(1.0, Frame::quad_spr(&s, x, y, &o));
            if fo.burn > 0.0 {
                f.glow(fo.b.x, fo.b.y - fo.b.h * 0.5, 14.0, [255, 110, 40, 90]);
            }
        }
        // Barnaby
        let b = &self.boss;
        if b.state != BS::Dormant && !(b.state == BS::Dead && b.t > 2.5) {
            if let Some(sh) = art.chars.get("barnaby") {
                let (anim, frame) = match b.state {
                    BS::Idle if b.b.vx.abs() > 1.0 => ("walk", b.at * 8.0),
                    BS::Sweep => ("atk", if b.t < 0.6 { 1.0 } else { 3.0 }),
                    BS::Throw => ("shoot", if b.hit_done { 1.0 } else { 0.0 }),
                    BS::Slam => ("atk", if b.hit_done { 4.0 } else { 0.0 }),
                    BS::Broken | BS::Transform => ("hurt", 0.0),
                    BS::Dead => ("dead", 0.0),
                    _ => ("idle", b.at * 2.0),
                };
                let s = sh.get(if anim == "dead" { 0 } else { 2 }, anim_index(anim), frame as usize);
                let fade = if b.state == BS::Dead { (255.0 * (1.0 - b.t / 2.5)).max(0.0) as u8 } else { 255 };
                let tint = if b.phase == 2 { [255, 190, 160, fade] } else { [255, 255, 255, fade] };
                f.world_q(1.5, Frame::quad_spr(&s, b.b.x, b.b.y, &SprOpt { flip: b.face < 0.0, flags: if b.flash > 0.0 { flags::FLASH } else { 0 }, tint, ..Default::default() }));
                if b.phase == 2 {
                    f.light(LightDef { x: b.b.x, y: b.b.y, z: 50.0, r: 150.0, col: [1.4, 0.5, 0.2], flicker: 0.6 });
                }
            }
        }
        // Spieler
        let p = &self.p;
        let (anim, frame) = match p.state {
            PS::Free if !p.b.ground => ("walk", 2.0),
            PS::Free if p.blocking => ("cast", 0.0),
            PS::Free if p.b.vx.abs() > 1.0 => ("walk", p.at * 10.0),
            PS::Free => ("idle", p.at * 2.5),
            PS::Climb => ("walk", (p.b.y / 8.0).abs()),
            PS::Roll => ("roll", (p.t / ROLL_T * 5.0).min(4.0)),
            PS::Attack | PS::Art | PS::Exec => {
                let k = p.t / (p.sw.wind + p.sw.act + p.sw.rec).max(0.01);
                ("atk", (k * 5.0).min(4.0))
            }
            PS::Hurt | PS::Stun => ("hurt", 0.0),
            PS::Drink => ("drink", (p.t * 3.5).min(2.0)),
            PS::Dead => ("dead", 0.0),
        };
        let s = art.player.get(if anim == "dead" { 0 } else { 2 }, anim_index(anim), frame as usize);
        let mut o = SprOpt { flip: p.face < 0.0, flags: if p.flash > 0.0 { flags::FLASH } else { 0 }, ..Default::default() };
        if p.state == PS::Roll && p.iframes > 0.0 {
            o.tint = [200, 220, 255, 230];
        }
        if p.state == PS::Climb {
            o.flip = (p.b.y / 16.0) as i32 % 2 == 0;
        }
        f.world_q(2.0, Frame::quad_spr(&s, p.b.x, p.b.y, &o));
        // eine kleine Laterne am Gürtel: Lichtinsel um den Spieler
        f.light(LightDef { x: p.b.x + p.face * 8.0, y: p.b.y, z: 30.0, r: 120.0, col: [0.9, 0.7, 0.5], flicker: 0.15 });
    }
}

fn new_player(at: (f32, f32), save: &Save) -> Player {
    let max_hp = 90.0 + 12.0 * save.vit as f32;
    let max_st = 90.0 + 8.0 * save.end as f32;
    Player {
        b: Body { x: at.0, y: at.1, w: 18.0, h: 44.0, ..Default::default() },
        face: 1.0,
        hp: max_hp,
        st: max_st,
        fp: 50.0,
        flasks: save.flasks,
        state: PS::Free,
        t: 0.0,
        sw: LIGHT,
        combo: 0,
        queued: None,
        hit_done: false,
        iframes: 0.0,
        st_delay: 0.0,
        block_t: 1.0,
        blocking: false,
        heal_left: 0.0,
        coyote: 0.0,
        jump_buf: 0.0,
        drop_t: 0.0,
        exec: None,
        flash: 0.0,
        at: 0.0,
    }
}
