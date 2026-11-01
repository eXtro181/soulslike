//! Kampf: Spieler (Laufen, Springen, Leitern, Rolle mit I-Frames, Hiebe nach Frame-Daten, Schild
//! und Parade, Aschenkünste, Hinrichtung), die Gegner der Aschenen Klippen von Calamita, der
//! Miniboss (Zertrümmerter Gallions-Schütze) und Kapitän Barnaby, der Aschen-Vogt, mit zwei Phasen. Zeiten in Sekunden, `F` = ein Bild bei 60 fps.

use super::fx::{c, rnd, rr, BurstOpt};
use super::input::Act;
use super::level::{Body, TS};
use super::Game;

pub const F: f32 = 1.0 / 60.0;
pub const GRAV: f32 = 1500.0;

/// Ein Hieb: Ausholen, aktive Bilder, Erholung, Reichweite, Faktoren für Schaden und Haltung, Ausdauer.
#[derive(Clone, Copy)]
pub struct Swing {
    pub wind: f32,
    pub act: f32,
    pub rec: f32,
    pub reach: f32,
    pub dmg: f32,
    pub post: f32,
    pub stam: f32,
}
pub const LIGHT: Swing = Swing { wind: 8.0 * F, act: 4.0 * F, rec: 14.0 * F, reach: 50.0, dmg: 1.0, post: 0.8, stam: 15.0 };
pub const HEAVY: Swing = Swing { wind: 18.0 * F, act: 5.0 * F, rec: 22.0 * F, reach: 58.0, dmg: 1.9, post: 2.2, stam: 28.0 };
/// Rolle: 24 Bilder, unverwundbar in Bild 3–17, 20 Ausdauer
pub const ROLL_T: f32 = 24.0 * F;
pub const ROLL_IFR: (f32, f32) = (3.0 * F, 17.0 * F);
/// Parade: Schild höchstens 10 Bilder vor dem Treffer gehoben
pub const PARRY_WIN: f32 = 10.0 * F;

/// Aschenkünste (am Schrein wählbar): Schlüssel, Name, Beschreibung, Glut (FP), Hieb
pub struct ArtDef {
    pub key: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub fp: f32,
    pub sw: Swing,
}
pub const ARTS: [ArtDef; 3] = [
    ArtDef { key: "wirbel", name: "Aschenwirbel", desc: "Drehhieb, trifft vorn und hinten.", fp: 18.0, sw: Swing { wind: 12.0 * F, act: 20.0 * F, rec: 18.0 * F, reach: 70.0, dmg: 1.4, post: 1.5, stam: 10.0 } },
    ArtDef { key: "stoss", name: "Glutstoß", desc: "Vorstoß, beim Anlauf unverwundbar.", fp: 14.0, sw: Swing { wind: 6.0 * F, act: 14.0 * F, rec: 16.0 * F, reach: 44.0, dmg: 1.6, post: 1.2, stam: 10.0 } },
    ArtDef { key: "schild", name: "Sturmschild", desc: "Konterhaltung: ein Treffer wird zurückgeschlagen.", fp: 12.0, sw: Swing { wind: 4.0 * F, act: 36.0 * F, rec: 10.0 * F, reach: 60.0, dmg: 2.2, post: 3.0, stam: 0.0 } },
];
pub fn art_def(k: &str) -> &'static ArtDef {
    ARTS.iter().find(|a| a.key == k).unwrap_or(&ARTS[0])
}
/// Prägungen der Waffe: Name, Wirkung
pub const AFFS: [(&str, &str); 4] = [("Gewöhnlich", "Keine Prägung."), ("Schwer", "+35 % Haltungsschaden."), ("Glut", "Brand: 3 Schaden je ½ s, 3 s lang."), ("Asche", "Jeder Treffer gibt 3 Glut zurück.")];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PS {
    Free,
    Climb,
    Attack,
    Roll,
    Hurt,
    Stun,
    Drink,
    Art,
    Exec,
    Dead,
}

pub struct Player {
    pub b: Body,
    pub face: f32,
    pub hp: f32,
    pub st: f32,
    pub fp: f32,
    pub flasks: i32,
    pub state: PS,
    pub t: f32,
    pub sw: Swing,
    pub combo: i32,
    pub queued: Option<bool>,
    pub hit_done: bool,
    pub iframes: f32,
    pub st_delay: f32,
    /// Zeit seit dem Heben des Schilds (für die Parade)
    pub block_t: f32,
    pub blocking: bool,
    pub heal_left: f32,
    pub coyote: f32,
    pub jump_buf: f32,
    pub drop_t: f32,
    pub exec: Option<Tgt>,
    pub flash: f32,
    pub at: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tgt {
    Foe(usize),
    Boss,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    Marauder,
    Grabber,
    Crawler,
    /// Miniboss: feuert Kanonenkugeln, hält Abstand
    Gunner,
}
pub struct FoeDef {
    pub name: &'static str,
    pub sheet: &'static str,
    pub hp: f32,
    pub post: f32,
    pub dmg: f32,
    pub speed: f32,
    pub reach: f32,
    pub wind: f32,
    pub rec: f32,
    pub ash: i64,
    pub w: f32,
    pub h: f32,
}
pub fn foe_def(k: Kind) -> FoeDef {
    match k {
        Kind::Marauder => FoeDef { name: "Aschen-Marodeur", sheet: "ashSkeleton", hp: 70.0, post: 60.0, dmg: 14.0, speed: 125.0, reach: 40.0, wind: 0.32, rec: 0.45, ash: 40, w: 20.0, h: 46.0 },
        Kind::Grabber => FoeDef { name: "Rost-Skelett", sheet: "skelBrute", hp: 220.0, post: 160.0, dmg: 30.0, speed: 48.0, reach: 118.0, wind: 0.8, rec: 0.9, ash: 140, w: 30.0, h: 58.0 },
        Kind::Crawler => FoeDef { name: "Schlick-Krabbler", sheet: "crab", hp: 50.0, post: 40.0, dmg: 12.0, speed: 90.0, reach: 34.0, wind: 0.28, rec: 0.5, ash: 30, w: 28.0, h: 20.0 },
        Kind::Gunner => FoeDef { name: "Zertrümmerter Gallions-Schütze", sheet: "gunner", hp: 420.0, post: 200.0, dmg: 24.0, speed: 55.0, reach: 340.0, wind: 0.7, rec: 1.0, ash: 600, w: 34.0, h: 70.0 },
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FS {
    Idle,
    Hang,
    Chase,
    Windup,
    Strike,
    Recover,
    Hurt,
    /// Haltung gebrochen oder pariert: hinrichtbar, Restzeit
    Broken,
    Dead,
}

pub struct Foe {
    pub kind: Kind,
    pub b: Body,
    pub face: f32,
    pub hp: f32,
    pub post: f32,
    pub post_t: f32,
    pub state: FS,
    pub t: f32,
    pub cool: f32,
    pub combo: i32,
    pub hit_done: bool,
    pub burn: f32,
    pub burn_tick: f32,
    pub flash: f32,
    pub at: f32,
    pub broken_t: f32,
}

impl Foe {
    pub fn new(kind: Kind, x: f32, y: f32) -> Self {
        let d = foe_def(kind);
        let hang = kind == Kind::Crawler;
        Foe {
            kind,
            b: Body { x, y: if hang { y - TS + d.h } else { y }, w: d.w, h: d.h, ..Default::default() },
            face: -1.0,
            hp: d.hp,
            post: 0.0,
            post_t: 0.0,
            state: if hang { FS::Hang } else { FS::Idle },
            t: 0.0,
            cool: 0.5,
            combo: 0,
            hit_done: false,
            burn: 0.0,
            burn_tick: 0.0,
            flash: 0.0,
            at: rnd() * 3.0,
            broken_t: 0.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BS {
    Dormant,
    Intro,
    Idle,
    Sweep,
    Throw,
    Slam,
    Broken,
    Transform,
    Dead,
}

/// Teerbomben der zweiten Phase: Marke am Boden, Einschlag mit Explosion, danach brennender Teer.
pub struct Hazard {
    pub x: f32,
    pub t: f32,
    pub fell: bool,
}

pub struct Boss {
    pub b: Body,
    pub face: f32,
    pub hp: f32,
    pub post: f32,
    pub post_t: f32,
    pub phase: i32,
    pub state: BS,
    pub t: f32,
    pub cool: f32,
    pub hit_done: bool,
    pub flash: f32,
    pub at: f32,
    pub burn: f32,
    pub burn_tick: f32,
    pub broken_t: f32,
    /// Ankerwurf: x, Richtung, Weg, zurück
    pub anchor: Option<(f32, f32, f32, bool)>,
    pub slam_from: (f32, f32),
    pub home: (f32, f32),
    pub hazards: Vec<Hazard>,
    /// brennender Teer am Boden: x, Restzeit
    pub flames: Vec<(f32, f32)>,
    pub hazard_t: f32,
    pub flame_tick: f32,
}
pub const BOSS_NAME: &str = "Kapitän Barnaby, der Aschen-Vogt";
pub const BOSS_HP: f32 = 1000.0;
pub const BOSS_POST: f32 = 320.0;

impl Boss {
    pub fn new(x: f32, y: f32) -> Self {
        Boss {
            b: Body { x, y, w: 40.0, h: 84.0, ..Default::default() },
            face: -1.0,
            hp: BOSS_HP,
            post: 0.0,
            post_t: 0.0,
            phase: 1,
            state: BS::Dormant,
            t: 0.0,
            cool: 1.0,
            hit_done: false,
            flash: 0.0,
            at: 0.0,
            burn: 0.0,
            burn_tick: 0.0,
            broken_t: 0.0,
            anchor: None,
            slam_from: (x, y),
            home: (x, y),
            hazards: vec![],
            flames: vec![],
            hazard_t: 2.0,
            flame_tick: 0.0,
        }
    }
    pub fn active(&self) -> bool {
        !matches!(self.state, BS::Dormant | BS::Dead)
    }
}

/// Kanonenkugel des Gallions-Schützen: x, y, vx, vy, Restzeit
pub type Shot = (f32, f32, f32, f32, f32);

/// Überlappen sich zwei Kästen (Mitte x, Unterkante y)?
fn overlap(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) -> bool {
    (ax - bx).abs() < (aw + bw) * 0.5 && ay - ah < by && by - bh < ay
}

impl Game {
    // ================================================================== Spieler
    pub fn max_hp(&self) -> f32 {
        90.0 + 12.0 * self.save.vit as f32
    }
    pub fn max_st(&self) -> f32 {
        90.0 + 8.0 * self.save.end as f32
    }
    pub fn max_fp(&self) -> f32 {
        50.0
    }
    fn base_dmg(&self) -> f32 {
        22.0 * (1.0 + 0.08 * self.save.str as f32)
    }
    fn use_st(&mut self, n: f32) -> bool {
        let n = n * if self.save.aff == 1 { 1.1 } else { 1.0 };
        if self.p.st <= 0.0 {
            return false;
        }
        self.p.st -= n;
        self.p.st_delay = 0.6;
        true
    }

    pub fn update_player(&mut self, dt: f32) {
        let inp_x = self.input.axis().x;
        let up = self.input.held(Act::Up);
        let down = self.input.held(Act::Down);
        let (max_hp, max_st, block_held) = (self.max_hp(), self.max_st(), self.input.held(Act::Block));
        let p = &mut self.p;
        p.t += dt;
        p.at += dt;
        p.iframes = (p.iframes - dt).max(0.0);
        p.flash = (p.flash - dt).max(0.0);
        p.coyote -= dt;
        p.jump_buf -= dt;
        p.drop_t -= dt;
        p.st_delay -= dt;
        p.blocking = false;
        if self.input.pressed(Act::Jump) {
            p.jump_buf = 0.12;
        }
        if p.st_delay <= 0.0 && p.state != PS::Roll {
            p.st = (p.st + 55.0 * dt * if block_held { 0.5 } else { 1.0 }).min(max_st);
        }
        if p.heal_left > 0.0 {
            let h = p.heal_left.min(max_hp * 0.9 * dt);
            p.hp = (p.hp + h).min(max_hp);
            p.heal_left -= h;
        }
        if p.state == PS::Dead {
            p.b.vx = 0.0;
            p.b.vy = (p.b.vy + GRAV * dt).min(900.0);
            let mut b = p.b;
            self.lv.step(&mut b, dt);
            self.p.b = b;
            return;
        }
        let mut speed_x = 0.0;
        let mut gravity = true;
        match self.p.state {
            PS::Free => {
                let p = &mut self.p;
                if p.b.ground {
                    p.coyote = 0.1;
                }
                // Leiter greifen
                let tx = (p.b.x / TS).floor() as i32;
                let feet = ((p.b.y - 2.0) / TS).floor() as i32;
                let below = (p.b.y / TS).floor() as i32;
                if (up && self.lv.ladder(tx, feet)) || (down && p.b.ground && self.lv.ladder(tx, below)) {
                    p.state = PS::Climb;
                    p.b.x = tx as f32 * TS + TS * 0.5;
                    p.b.vx = 0.0;
                    p.b.vy = 0.0;
                    return;
                }
                p.blocking = self.input.held(Act::Block);
                if self.input.pressed(Act::Block) {
                    p.block_t = 0.0;
                }
                p.block_t += dt;
                let sp = if p.blocking { 60.0 } else { 150.0 };
                speed_x = inp_x * sp;
                if inp_x.abs() > 0.2 && !p.blocking {
                    p.face = inp_x.signum();
                }
                // Sprung (mit Puffer und Koyotenzeit), Planken nach unten durchfallen
                if p.jump_buf > 0.0 && p.coyote > 0.0 {
                    p.jump_buf = 0.0;
                    p.coyote = 0.0;
                    if down && p.b.ground && self.lv.oneway(tx, (p.b.y / TS).floor() as i32) {
                        p.drop_t = 0.25;
                    } else {
                        p.b.vy = -440.0;
                        self.sfx_q.push("roll");
                    }
                }
                if !self.input.held(Act::Jump) && self.p.b.vy < -150.0 {
                    self.p.b.vy *= 0.5;
                }
                let ground = self.p.b.ground;
                if self.input.pressed(Act::Roll) && ground && self.use_st(20.0) {
                    self.p.state = PS::Roll;
                    self.p.t = 0.0;
                    if inp_x.abs() > 0.2 {
                        self.p.face = inp_x.signum();
                    }
                    self.sfx_q.push("roll");
                } else if self.input.pressed(Act::Attack) {
                    if let Some(t) = self.exec_target() {
                        self.start_exec(t);
                    } else if self.use_st(LIGHT.stam) {
                        self.start_swing(LIGHT, false);
                    }
                } else if self.input.pressed(Act::Heavy) && self.use_st(HEAVY.stam) {
                    self.start_swing(HEAVY, false);
                } else if self.input.pressed(Act::Art) && ground {
                    let a = art_def(&self.save.arts[self.save.art.min(self.save.arts.len() - 1)]);
                    if self.p.fp < a.fp {
                        self.toast("Zu wenig Glut.");
                    } else if self.use_st(a.sw.stam) {
                        self.p.fp -= a.fp;
                        self.p.state = PS::Art;
                        self.p.t = 0.0;
                        self.p.sw = a.sw;
                        self.p.hit_done = false;
                        self.sfx_q.push("charge");
                    }
                } else if self.input.pressed(Act::Drink) && ground {
                    if self.p.flasks > 0 {
                        self.p.flasks -= 1;
                        self.p.state = PS::Drink;
                        self.p.t = 0.0;
                        self.sfx_q.push("drink");
                    } else {
                        self.toast("Kein Aschentrank mehr.");
                    }
                }
            }
            PS::Climb => {
                let p = &mut self.p;
                let dir = (down as i32 - up as i32) as f32;
                p.b.vy = dir * 120.0;
                let tx = (p.b.x / TS).floor() as i32;
                if p.jump_buf > 0.0 {
                    p.jump_buf = 0.0;
                    p.state = PS::Free;
                    p.b.vy = -200.0;
                    p.b.vx = inp_x * 150.0;
                } else {
                    let ny = p.b.y + p.b.vy * dt;
                    let feet = ((ny - 2.0) / TS).floor() as i32;
                    if dir < 0.0 && !self.lv.ladder(tx, feet) {
                        // oben angekommen: auf den Leiterkopf stellen
                        p.b.y = (feet + 1) as f32 * TS;
                        p.b.vy = 0.0;
                        p.state = PS::Free;
                        p.b.ground = true;
                    } else if dir > 0.0 && !self.lv.ladder(tx, feet) && !self.lv.ladder(tx, feet + 1) {
                        p.state = PS::Free;
                    } else {
                        p.b.drop = true;
                        let mut b = p.b;
                        self.lv.step(&mut b, dt);
                        if b.ground && dir > 0.0 {
                            self.p.state = PS::Free;
                        }
                        self.p.b = b;
                        self.p.b.drop = false;
                    }
                }
                return;
            }
            PS::Roll => {
                let p = &mut self.p;
                let k = p.t / ROLL_T;
                if p.t >= ROLL_IFR.0 && p.t <= ROLL_IFR.1 {
                    p.iframes = p.iframes.max(0.02);
                }
                speed_x = p.face * 270.0 * (1.0 - k * 0.6);
                if p.t > 0.08 && rnd() < 0.4 {
                    let (x, y) = (p.b.x, p.b.y);
                    self.fx.burst("dust", x, y, 1, BurstOpt { speed: Some(40.0), up: Some(20.0), g: Some(80.0), life: Some(0.4), ..Default::default() });
                }
                if self.p.t >= ROLL_T {
                    self.p.state = PS::Free;
                }
            }
            PS::Attack => {
                let sw = self.p.sw;
                let t = self.p.t;
                if t < sw.wind * 0.6 && inp_x.abs() > 0.3 {
                    self.p.face = inp_x.signum();
                }
                if !self.p.b.ground {
                    speed_x = self.p.b.vx;
                }
                if t >= sw.wind && !self.p.hit_done {
                    self.p.hit_done = true;
                    if self.p.b.ground {
                        speed_x = self.p.face * 140.0;
                    }
                    let face = self.p.face;
                    let dmg = self.base_dmg() * sw.dmg * (1.0 + self.p.combo as f32 * 0.15);
                    self.strike(face, sw.reach, 0.0, dmg, dmg * sw.post);
                }
                if t > sw.wind + sw.act && (self.input.pressed(Act::Attack) || self.input.pressed(Act::Heavy)) {
                    self.p.queued = Some(self.input.pressed(Act::Heavy));
                }
                // Rolle bricht die Erholung ab
                if t > sw.wind + sw.act && self.input.pressed(Act::Roll) && self.use_st(20.0) {
                    self.p.state = PS::Roll;
                    self.p.t = 0.0;
                    if inp_x.abs() > 0.2 {
                        self.p.face = inp_x.signum();
                    }
                } else if t >= sw.wind + sw.act + sw.rec {
                    let queued = self.p.queued;
                    match queued {
                        Some(h) if self.p.combo < 2 && self.use_st(if h { HEAVY.stam } else { LIGHT.stam }) => {
                            self.p.combo += 1;
                            self.start_swing(if h { HEAVY } else { LIGHT }, true);
                        }
                        _ => {
                            self.p.state = PS::Free;
                            self.p.combo = 0;
                        }
                    }
                }
            }
            PS::Art => {
                let sw = self.p.sw;
                let t = self.p.t;
                let key = self.save.arts[self.save.art.min(self.save.arts.len() - 1)].clone();
                let dmg = self.base_dmg() * sw.dmg;
                match key.as_str() {
                    "stoss" => {
                        if t < sw.wind + sw.act {
                            self.p.iframes = self.p.iframes.max(0.02);
                        }
                        if t >= sw.wind && t < sw.wind + sw.act {
                            speed_x = self.p.face * 420.0;
                            if !self.p.hit_done {
                                let face = self.p.face;
                                if self.strike(face, sw.reach, 0.0, dmg, dmg * sw.post) {
                                    self.p.hit_done = true;
                                }
                            }
                        }
                    }
                    "wirbel" => {
                        // zwei Treffer während der Drehung, vorn und hinten
                        let hits = ((t - sw.wind) / (sw.act * 0.5)).floor() as i32;
                        if t >= sw.wind && hits >= 0 && hits < 2 && (hits == 0) != self.p.hit_done {
                            self.p.hit_done = hits == 0;
                            self.strike(1.0, sw.reach, sw.reach * 0.5, dmg * 0.5, dmg * 0.5 * sw.post);
                            self.strike(-1.0, sw.reach, sw.reach * 0.5, dmg * 0.5, dmg * 0.5 * sw.post);
                            let (x, y) = (self.p.b.x, self.p.b.y - 26.0);
                            self.fx.slash(x, y, if hits == 0 { 0.0 } else { std::f32::consts::PI }, sw.reach, [1.0, 0.55, 0.3, 1.0], 2.0);
                        }
                    }
                    _ => {
                        // Sturmschild: Haltung, der Konter passiert in hurt_player
                        if t >= sw.wind && t < sw.wind + sw.act {
                            let (x, y) = (self.p.b.x + self.p.face * 14.0, self.p.b.y - 30.0);
                            if rnd() < 0.5 {
                                self.fx.burst("soulLight", x, y, 1, BurstOpt { speed: Some(30.0), ..Default::default() });
                            }
                        }
                    }
                }
                if self.p.t >= sw.wind + sw.act + sw.rec {
                    self.p.state = PS::Free;
                }
            }
            PS::Drink => {
                if self.p.t > 0.5 && self.p.t - dt <= 0.5 {
                    self.p.heal_left += self.max_hp() * 0.45;
                    let (x, y) = (self.p.b.x, self.p.b.y);
                    self.fx.rise("heal", x, y, 16, 28.0, Some(c::HEAL));
                    self.sfx_q.push("heal");
                }
                speed_x = inp_x * 40.0;
                if self.p.t > 0.9 {
                    self.p.state = PS::Free;
                }
            }
            PS::Hurt => {
                speed_x = self.p.b.vx * 0.9;
                if self.p.t > 0.3 {
                    self.p.state = PS::Free;
                }
            }
            PS::Stun => {
                if self.p.t > 0.9 {
                    self.p.state = PS::Free;
                }
            }
            PS::Exec => {
                gravity = true;
                self.p.iframes = 0.1;
                if self.p.t > 0.45 && !self.p.hit_done {
                    self.p.hit_done = true;
                    if let Some(t) = self.p.exec {
                        let dmg = self.base_dmg() * 5.0;
                        self.hurt_target(t, dmg, 0.0, true);
                        self.sfx_q.push("crit");
                        self.shake = 10.0;
                        self.hitstop = 0.18;
                        let (x, y) = self.target_pos(t);
                        self.fx.burst("blood", x, y, 26, BurstOpt { speed: Some(200.0), up: Some(60.0), ..Default::default() });
                        self.fx.burst("ember", x, y, 20, BurstOpt { speed: Some(160.0), up: Some(80.0), ..Default::default() });
                    }
                }
                if self.p.t > 1.0 {
                    self.p.state = PS::Free;
                    self.p.exec = None;
                }
            }
            _ => {}
        }
        let p = &mut self.p;
        p.b.vx = speed_x;
        if gravity {
            p.b.vy = (p.b.vy + GRAV * dt).min(900.0);
        }
        p.b.drop = p.drop_t > 0.0;
        let mut b = p.b;
        self.lv.step(&mut b, dt);
        self.p.b = b;
    }

    fn start_swing(&mut self, sw: Swing, chained: bool) {
        let p = &mut self.p;
        p.state = PS::Attack;
        p.t = 0.0;
        p.sw = sw;
        p.hit_done = false;
        p.queued = None;
        if !chained {
            p.combo = 0;
        }
        self.sfx_q.push(if sw.dmg > 1.5 { "heavy" } else { "swing" });
    }

    /// Ziele im Bild (lebend)
    fn targets(&self) -> Vec<Tgt> {
        let mut v: Vec<Tgt> = (0..self.foes.len()).filter(|i| self.foes[*i].state != FS::Dead).map(Tgt::Foe).collect();
        if self.boss.active() && !matches!(self.boss.state, BS::Intro | BS::Transform) {
            v.push(Tgt::Boss);
        }
        v
    }
    /// Mitte des Körpers
    pub fn target_pos(&self, t: Tgt) -> (f32, f32) {
        let b = self.target_body(t);
        (b.x, b.y - b.h * 0.5)
    }
    fn target_body(&self, t: Tgt) -> Body {
        match t {
            Tgt::Foe(i) => self.foes[i].b,
            Tgt::Boss => self.boss.b,
        }
    }
    fn broken(&self, t: Tgt) -> bool {
        match t {
            Tgt::Foe(i) => self.foes[i].state == FS::Broken,
            Tgt::Boss => self.boss.state == BS::Broken,
        }
    }
    /// hinrichtbares Ziel in Reichweite
    pub fn exec_target(&self) -> Option<Tgt> {
        self.targets().into_iter().find(|t| {
            let b = self.target_body(*t);
            self.broken(*t) && (b.x - self.p.b.x).abs() < 40.0 + b.w * 0.5 && (b.y - self.p.b.y).abs() < 40.0
        })
    }
    fn start_exec(&mut self, t: Tgt) {
        self.p.state = PS::Exec;
        self.p.t = 0.0;
        self.p.exec = Some(t);
        self.p.hit_done = false;
        let tx = self.target_body(t).x;
        self.p.face = (tx - self.p.b.x).signum();
        self.sfx_q.push("heavy");
        // das Ziel bleibt bis zum Stich gebrochen
        match t {
            Tgt::Foe(i) => self.foes[i].broken_t = self.foes[i].broken_t.max(1.2),
            Tgt::Boss => self.boss.broken_t = self.boss.broken_t.max(1.2),
        }
    }

    /// Hieb des Spielers nach `face`: trifft alles im Kasten davor (`back` = Reichweite nach hinten).
    fn strike(&mut self, face: f32, reach: f32, back: f32, dmg: f32, post: f32) -> bool {
        let (px, py) = (self.p.b.x, self.p.b.y);
        let (x0, x1) = if face > 0.0 { (px - back, px + reach) } else { (px - reach, px + back) };
        let mut hit = false;
        for t in self.targets() {
            let b = self.target_body(t);
            if overlap((x0 + x1) * 0.5, py + 4.0, x1 - x0, 56.0, b.x, b.y, b.w, b.h) {
                let d = (dmg * (0.9 + rnd() * 0.2)).round();
                self.hurt_target(t, d, post, false);
                hit = true;
                let (tx, ty) = self.target_pos(t);
                self.fx.burst("spark", tx, ty + b.h * 0.5, 6, BurstOpt { speed: Some(140.0), up: Some(b.h * 0.5), ..Default::default() });
            }
        }
        let (fx, fy) = (px + face * 12.0, py - 26.0);
        self.fx.slash(fx, fy, if face > 0.0 { 0.0 } else { std::f32::consts::PI }, reach * 0.8, [0.9, 0.95, 1.0, 1.0], 1.4);
        if hit {
            self.shake = self.shake.max(3.0);
            self.hitstop = self.hitstop.max(0.04);
            if self.save.aff == 3 {
                self.p.fp = (self.p.fp + 3.0).min(self.max_fp());
            }
            self.sfx_q.push("hit");
        }
        hit
    }

    /// Schaden und Haltungsschaden an ein Ziel; Prägung wird hier angewandt.
    pub fn hurt_target(&mut self, t: Tgt, dmg: f32, post: f32, exec: bool) {
        let aff = self.save.aff;
        let post = post * if aff == 1 { 1.35 } else { 1.0 };
        let dmg = dmg * if aff == 1 { 1.05 } else if aff == 3 { 0.95 } else { 1.0 };
        match t {
            Tgt::Foe(i) => {
                let d = foe_def(self.foes[i].kind);
                let f = &mut self.foes[i];
                if f.state == FS::Dead {
                    return;
                }
                f.hp -= dmg;
                f.flash = 0.1;
                if aff == 2 {
                    f.burn = 3.0;
                }
                if f.state == FS::Hang {
                    f.state = FS::Chase;
                }
                if exec {
                    f.post = 0.0;
                    f.state = FS::Hurt;
                    f.t = 0.0;
                } else {
                    f.post += post;
                    f.post_t = 0.0;
                    if f.post >= d.post && f.state != FS::Broken {
                        f.state = FS::Broken;
                        f.broken_t = 2.2;
                        self.sfx_q.push("parried");
                    } else if !matches!(f.kind, Kind::Grabber | Kind::Gunner) && !matches!(f.state, FS::Strike | FS::Broken) {
                        f.state = FS::Hurt;
                        f.t = 0.0;
                        f.b.vx = (f.b.x - self.p.b.x).signum() * 90.0;
                    }
                }
                if f.hp <= 0.0 {
                    f.hp = 0.0;
                    f.state = FS::Dead;
                    f.t = 0.0;
                    let (x, y) = (f.b.x, f.b.y);
                    self.gain_ash(d.ash);
                    if self.foes[i].kind == Kind::Gunner {
                        self.save.miniboss = true;
                        self.banner("ZERSCHMETTERT", "Der Gallions-Schütze schweigt", 3.0);
                        self.persist();
                    }
                    self.fx.burst("ember", x, y, 24, BurstOpt { speed: Some(90.0), up: Some(30.0), g: Some(-30.0), life: Some(1.4), ..Default::default() });
                    self.sfx_q.push("bones");
                }
            }
            Tgt::Boss => {
                let b = &mut self.boss;
                if !b.active() || matches!(b.state, BS::Intro | BS::Transform) {
                    return;
                }
                b.hp -= dmg;
                b.flash = 0.08;
                if aff == 2 {
                    b.burn = 3.0;
                }
                if exec {
                    b.post = 0.0;
                    b.state = BS::Idle;
                    b.cool = 0.8;
                } else {
                    b.post += post;
                    b.post_t = 0.0;
                    if b.post >= BOSS_POST && b.state != BS::Broken {
                        b.state = BS::Broken;
                        b.broken_t = 2.5;
                        b.anchor = None;
                        self.sfx_q.push("parried");
                    }
                }
                if b.hp <= 0.0 {
                    self.boss_die();
                } else if b.phase == 1 && b.hp <= BOSS_HP * 0.5 {
                    b.phase = 2;
                    b.state = BS::Transform;
                    b.t = 0.0;
                    b.anchor = None;
                    self.banner("TEER UND GLUT", "Barnaby entzündet die Teerfässer der Galeone", 2.6);
                    self.shake = 10.0;
                    self.sfx_q.push("roar");
                }
            }
        }
    }

    /// Schaden am Spieler. `from` = Angreifer (für Parade und Konter), `parry` = parierbar,
    /// `block` = mit dem Schild abwehrbar.
    pub fn hurt_player(&mut self, dmg: f32, from: Option<Tgt>, parry: bool, block: bool) -> bool {
        let (iframes, state, pt, sw, px, face, blocking, block_t) = (self.p.iframes, self.p.state, self.p.t, self.p.sw, self.p.b.x, self.p.face, self.p.blocking, self.p.block_t);
        if iframes > 0.0 || state == PS::Dead {
            return false;
        }
        let src_x = from.map(|t| self.target_body(t).x).unwrap_or(px - face);
        let facing = (src_x - px) * face >= 0.0;
        // Sturmschild: der Treffer wird zurückgeschlagen
        if state == PS::Art && art_def(&self.save.arts[self.save.art.min(self.save.arts.len() - 1)]).key == "schild" && pt >= sw.wind && pt < sw.wind + sw.act {
            if let Some(t) = from {
                let d = self.base_dmg() * sw.dmg;
                self.hurt_target(t, d, d * sw.post, false);
                self.p.iframes = 0.3;
                self.p.t = sw.wind + sw.act;
                self.sfx_q.push("parried");
                self.shake = 6.0;
                let (x, y) = self.target_pos(t);
                self.fx.burst("soulLight", x, y, 16, BurstOpt { speed: Some(180.0), ..Default::default() });
                return false;
            }
        }
        if blocking && facing && block {
            if parry && block_t <= PARRY_WIN {
                if let Some(t) = from {
                    self.on_parry(t);
                }
                return false;
            }
            let p = &mut self.p;
            p.hp -= (dmg * 0.15).round();
            p.st -= dmg * 1.6;
            p.st_delay = 0.6;
            p.b.vx = -p.face * 80.0;
            self.sfx_q.push("parry");
            if p.st <= 0.0 {
                p.state = PS::Stun;
                p.t = 0.0;
                self.toast("Deckung gebrochen!");
                self.sfx_q.push("crash");
            }
            if self.p.hp <= 0.0 {
                self.player_died();
            }
            return true;
        }
        let p = &mut self.p;
        p.hp -= dmg.round();
        p.flash = 0.12;
        p.iframes = 0.4;
        p.heal_left = 0.0;
        if p.state != PS::Exec {
            p.state = PS::Hurt;
            p.t = 0.0;
            p.b.vx = (p.b.x - src_x).signum() * 160.0;
            p.b.vy = -120.0;
        }
        let (x, y) = (p.b.x, p.b.y);
        self.fx.burst("blood", x, y, 8, BurstOpt { speed: Some(100.0), up: Some(30.0), ..Default::default() });
        self.shake = self.shake.max(5.0);
        self.sfx_q.push("hurt");
        if self.p.hp <= 0.0 {
            self.player_died();
        }
        true
    }

    fn on_parry(&mut self, t: Tgt) {
        self.sfx_q.push("parried");
        self.shake = 4.0;
        self.hitstop = 0.12;
        match t {
            Tgt::Foe(i) => {
                let f = &mut self.foes[i];
                f.state = FS::Broken;
                f.broken_t = 1.4;
            }
            Tgt::Boss => {
                let b = &mut self.boss;
                b.post += BOSS_POST * 0.3;
                b.anchor = None;
                b.state = BS::Broken;
                b.broken_t = if b.post >= BOSS_POST { 2.5 } else { 1.0 };
            }
        }
        let (x, y) = self.target_pos(t);
        self.fx.burst("spark", x, y, 18, BurstOpt { speed: Some(220.0), ..Default::default() });
        self.toast("Pariert! Jetzt hinrichten (J).");
    }

    // ================================================================== Gegner
    pub fn update_foes(&mut self, dt: f32) {
        for i in 0..self.foes.len() {
            self.update_foe(i, dt);
        }
    }

    fn update_foe(&mut self, i: usize, dt: f32) {
        let d = foe_def(self.foes[i].kind);
        let (px, py) = (self.p.b.x, self.p.b.y);
        let pdead = self.p.state == PS::Dead;
        // Spieler nah an einer Wand? Dann bedrängt der Plünderer stärker
        let cornered = self.lv.blocked_at(px - 64.0, py, 10.0, 30.0) || self.lv.blocked_at(px + 64.0, py, 10.0, 30.0);
        let f = &mut self.foes[i];
        f.t += dt;
        f.at += dt;
        f.flash = (f.flash - dt).max(0.0);
        f.cool = (f.cool - dt).max(0.0);
        f.post_t += dt;
        if f.post_t > 1.5 && f.state != FS::Broken {
            f.post = (f.post - 25.0 * dt).max(0.0);
        }
        if f.state == FS::Dead {
            return;
        }
        if f.burn > 0.0 {
            f.burn -= dt;
            f.burn_tick -= dt;
            if f.burn_tick <= 0.0 {
                f.burn_tick = 0.5;
                f.hp -= 3.0;
                let (x, y) = (f.b.x, f.b.y);
                self.fx.burst("ember", x, y, 3, BurstOpt { speed: Some(30.0), up: Some(30.0), ..Default::default() });
                if self.foes[i].hp <= 0.0 {
                    self.hurt_target(Tgt::Foe(i), 1.0, 0.0, false);
                    return;
                }
            }
        }
        let f = &mut self.foes[i];
        let dx = px - f.b.x;
        let dy = py - f.b.y;
        let sees = !pdead && dx.abs() < 300.0 && dy.abs() < 90.0;
        let mut gravity = true;
        let mut vx = 0.0;
        match f.state {
            FS::Hang => {
                gravity = false;
                f.b.vy = 0.0;
                // fällt herab, wenn der Spieler darunter vorbeigeht
                if !pdead && dx.abs() < 36.0 && dy > 0.0 && dy < 6.0 * TS {
                    f.state = FS::Chase;
                    self.sfx_q.push("crush");
                }
            }
            FS::Idle => {
                if sees {
                    f.state = FS::Chase;
                }
            }
            FS::Chase => {
                if !sees && f.t > 3.0 {
                    f.state = FS::Idle;
                } else {
                    if sees {
                        f.t = 0.0;
                    }
                    f.face = dx.signum();
                    if f.kind == Kind::Gunner && dx.abs() < 110.0 {
                        // weicht zurück und tritt dann zu
                        vx = -f.face * d.speed;
                        if f.cool <= 0.0 && dx.abs() < 60.0 {
                            f.state = FS::Windup;
                            f.t = d.wind * 0.5;
                            f.combo = 1;
                        }
                    } else if dx.abs() < d.reach + 8.0 && dy.abs() < if f.kind == Kind::Gunner { 160.0 } else { 60.0 } && f.cool <= 0.0 && f.b.ground {
                        f.state = FS::Windup;
                        f.t = 0.0;
                        f.combo = match f.kind {
                            Kind::Marauder if cornered => 3,
                            Kind::Marauder => 2,
                            _ => 1,
                        };
                    } else if dx.abs() > d.reach * 0.6 {
                        vx = f.face * d.speed * if f.kind == Kind::Marauder && cornered { 1.3 } else { 1.0 };
                    }
                }
            }
            FS::Windup => {
                let w = d.wind * if f.kind == Kind::Marauder && cornered { 0.7 } else { 1.0 };
                if f.t >= w {
                    f.state = FS::Strike;
                    f.t = 0.0;
                    f.hit_done = false;
                }
            }
            FS::Strike => {
                if f.kind == Kind::Marauder && f.t < 0.12 {
                    vx = f.face * 260.0;
                }
                if !f.hit_done && f.t > 0.04 {
                    f.hit_done = true;
                    let (fx0, fface, fy) = (f.b.x, f.face, f.b.y);
                    // Kettengreifer: der Anker fegt nach beiden Seiten
                    let (x0, x1) = match f.kind {
                        Kind::Grabber => (fx0 - d.reach, fx0 + d.reach),
                        _ if fface > 0.0 => (fx0, fx0 + d.reach + 10.0),
                        _ => (fx0 - d.reach - 10.0, fx0),
                    };
                    let kind = f.kind;
                    if kind == Kind::Gunner && dx.abs() > 70.0 {
                        // Kanonenschuss im flachen Bogen auf den Spieler
                        let t = (dx.abs() / 380.0).clamp(0.4, 1.0);
                        let (sx, sy) = (fx0 + fface * 24.0, fy - 44.0);
                        self.shots.push((sx, sy, (px - sx) / t, (py - 20.0 - sy) / t - GRAV * 0.25 * t, 2.0));
                        self.sfx_q.push("cannon");
                        self.shake = self.shake.max(3.0);
                        self.fx.burst("smoke", sx, sy, 6, BurstOpt { speed: Some(50.0), ..Default::default() });
                    } else if kind == Kind::Grabber {
                        self.shake = self.shake.max(5.0);
                        self.fx.ring(fx0, fy, 10.0, d.reach, 0.3, [0.7, 0.6, 0.5]);
                        self.sfx_q.push("chain");
                    } else {
                        self.sfx_q.push("swing");
                    }
                    if kind != Kind::Gunner || dx.abs() <= 70.0 {
                    if overlap((x0 + x1) * 0.5, fy, x1 - x0, if kind == Kind::Grabber { 70.0 } else { 44.0 }, px, py, self.p.b.w, self.p.b.h) {
                        self.hurt_player(d.dmg, Some(Tgt::Foe(i)), kind != Kind::Grabber, true);
                    }
                    }
                }
                let f = &mut self.foes[i];
                if f.t > 0.18 {
                    f.combo -= 1;
                    f.t = 0.0;
                    if f.combo > 0 {
                        f.state = FS::Windup;
                        f.t = d.wind * 0.45;
                    } else {
                        f.state = FS::Recover;
                    }
                }
            }
            FS::Recover => {
                if f.t > d.rec {
                    f.state = FS::Chase;
                    f.cool = 0.4 + rnd() * 0.6;
                }
            }
            FS::Hurt => {
                vx = f.b.vx * 0.85;
                if f.t > 0.25 {
                    f.state = FS::Chase;
                }
            }
            FS::Broken => {
                f.broken_t -= dt;
                if f.broken_t <= 0.0 {
                    f.state = FS::Chase;
                    f.post = 0.0;
                    f.cool = 0.3;
                }
            }
            FS::Dead => {}
        }
        let f = &mut self.foes[i];
        if f.state != FS::Hang {
            f.b.vx = vx;
            if gravity {
                f.b.vy = (f.b.vy + GRAV * dt).min(900.0);
            }
            // nicht über Kanten laufen
            if f.b.ground && vx != 0.0 {
                let ahead = f.b.x + vx.signum() * (f.b.w * 0.5 + 4.0);
                let tx = (ahead / TS).floor() as i32;
                let ty = (f.b.y / TS).floor() as i32;
                if !self.lv.solid(tx, ty) && self.lv.cell(tx, ty) != b'=' && !self.lv.ladder(tx, ty) {
                    f.b.vx = 0.0;
                }
            }
            let mut b = f.b;
            self.lv.step(&mut b, dt);
            self.foes[i].b = b;
        }
    }

    // ================================================================== Kapitän Barnaby
    pub fn update_boss(&mut self, dt: f32) {
        let (px, py) = (self.p.b.x, self.p.b.y);
        let b = &mut self.boss;
        b.t += dt;
        b.at += dt;
        b.flash = (b.flash - dt).max(0.0);
        b.cool -= dt;
        b.post_t += dt;
        if b.post_t > 2.0 && b.state != BS::Broken {
            b.post = (b.post - 20.0 * dt).max(0.0);
        }
        if !b.active() {
            return;
        }
        if b.burn > 0.0 {
            b.burn -= dt;
            b.burn_tick -= dt;
            if b.burn_tick <= 0.0 {
                b.burn_tick = 0.5;
                self.hurt_target(Tgt::Boss, 3.0, 0.0, false);
            }
        }
        let b = &mut self.boss;
        let dx = px - b.b.x;
        let mut vx = 0.0;
        let p2 = b.phase == 2;
        match b.state {
            BS::Intro => {
                b.face = dx.signum();
                if b.t > 1.6 {
                    b.state = BS::Idle;
                    b.cool = 0.6;
                }
            }
            BS::Idle => {
                b.face = dx.signum();
                if dx.abs() > 90.0 {
                    vx = b.face * if p2 { 80.0 } else { 60.0 };
                }
                if b.cool <= 0.0 && self.p.state != PS::Dead {
                    let near = dx.abs() < 170.0;
                    let r = rnd();
                    b.state = if near { if r < 0.6 { BS::Sweep } else { BS::Slam } } else if r < 0.5 { BS::Throw } else { BS::Slam };
                    b.t = 0.0;
                    b.hit_done = false;
                    b.slam_from = (b.b.x, b.b.y);
                }
            }
            BS::Sweep => {
                let wind = if p2 { 0.6 } else { 0.75 };
                if b.t >= wind && !b.hit_done {
                    b.hit_done = true;
                    let (bx, by, face) = (b.b.x, b.b.y, b.face);
                    let (x0, x1) = if face > 0.0 { (bx - 60.0, bx + 170.0) } else { (bx - 170.0, bx + 60.0) };
                    self.shake = 6.0;
                    self.sfx_q.push("heavy");
                    self.fx.slash(bx + face * 30.0, by - 50.0, if face > 0.0 { 0.0 } else { std::f32::consts::PI }, 150.0, [1.0, 0.7, 0.4, 1.0], 3.0);
                    if overlap((x0 + x1) * 0.5, by, x1 - x0, 80.0, px, py, self.p.b.w, self.p.b.h) {
                        self.hurt_player(32.0, Some(Tgt::Boss), true, true);
                    }
                    if p2 {
                        for k in 1..4 {
                            self.boss.flames.push((bx + face * 50.0 * k as f32, 3.5));
                        }
                    }
                }
                let b = &mut self.boss;
                if b.t > wind + 0.7 {
                    b.state = BS::Idle;
                    b.cool = if p2 { 0.5 } else { 0.9 };
                }
            }
            BS::Throw => {
                let wind = 0.6;
                if b.t >= wind && !b.hit_done {
                    b.hit_done = true;
                    b.anchor = Some((b.b.x + b.face * 30.0, b.face, 0.0, false));
                    self.sfx_q.push("chain");
                }
                let b = &mut self.boss;
                if let Some((ax, dir, dist, back)) = b.anchor {
                    let sp = 520.0 * dt;
                    let (ax, dist, back) = if back { (ax - dir * sp, dist - sp, true) } else { (ax + dir * sp, dist + sp, dist + sp > 380.0) };
                    b.anchor = if back && dist <= 0.0 { None } else { Some((ax, dir, dist, back)) };
                    let ay = b.b.y - 40.0;
                    if b.anchor.is_none() {
                        b.state = BS::Idle;
                        b.cool = if p2 { 0.6 } else { 1.0 };
                    }
                    if (ax - px).abs() < 26.0 && (ay - (py - 22.0)).abs() < 34.0 {
                        self.hurt_player(26.0, Some(Tgt::Boss), false, true);
                    }
                }
            }
            BS::Slam => {
                // ducken, springen (Bogen auf den Spieler), aufschlagen
                let (crouch, air) = (0.5, 0.6);
                if b.t < crouch {
                    b.face = dx.signum();
                    b.slam_from = (b.b.x, px.clamp(b.home.0 - 280.0, b.home.0 + 280.0));
                } else if b.t < crouch + air {
                    let k = (b.t - crouch) / air;
                    b.b.x = b.slam_from.0 + (b.slam_from.1 - b.slam_from.0) * k;
                    b.b.y = b.home.1 - (k * std::f32::consts::PI).sin() * 140.0;
                } else if !b.hit_done {
                    b.hit_done = true;
                    b.b.y = b.home.1;
                    let bx = b.b.x;
                    self.shake = 12.0;
                    self.sfx_q.push("crash");
                    self.fx.ring(bx, b.b.y, 10.0, 120.0, 0.35, [1.0, 0.6, 0.3]);
                    self.fx.burst("dust", bx, b.b.y, 20, BurstOpt { speed: Some(160.0), ..Default::default() });
                    if (px - bx).abs() < 120.0 && self.p.b.ground {
                        self.hurt_player(36.0, Some(Tgt::Boss), false, true);
                    }
                    if p2 {
                        self.boss.flames.push((bx - 60.0, 3.5));
                        self.boss.flames.push((bx + 60.0, 3.5));
                    }
                }
                let b = &mut self.boss;
                if b.t > crouch + air + 0.6 {
                    b.state = BS::Idle;
                    b.cool = if p2 { 0.5 } else { 0.9 };
                }
            }
            BS::Broken => {
                b.broken_t -= dt;
                if b.broken_t <= 0.0 {
                    b.state = BS::Idle;
                    b.cool = 0.4;
                    if b.post >= BOSS_POST {
                        b.post = 0.0;
                    }
                }
            }
            BS::Transform => {
                if b.t > 1.8 {
                    b.state = BS::Idle;
                    b.cool = 0.3;
                }
            }
            _ => {}
        }
        // zweite Phase: Teerbomben regnen, brennender Teer bleibt liegen
        let b = &mut self.boss;
        if p2 && b.state != BS::Transform {
            b.hazard_t -= dt;
            if b.hazard_t <= 0.0 {
                b.hazard_t = 2.4;
                let hx = b.home.0;
                b.hazards.push(Hazard { x: px + rr(-30.0, 30.0), t: 0.0, fell: false });
                b.hazards.push(Hazard { x: hx + rr(-260.0, 260.0), t: 0.0, fell: false });
            }
        }
        let mut impacts = vec![];
        for h in b.hazards.iter_mut() {
            h.t += dt;
            if h.t >= 1.0 && !h.fell {
                h.fell = true;
                impacts.push(h.x);
            }
        }
        b.hazards.retain(|h| h.t < 1.3);
        for f in b.flames.iter_mut() {
            f.1 -= dt;
        }
        b.flames.retain(|f| f.1 > 0.0);
        b.flame_tick -= dt;
        let in_flame = b.flame_tick <= 0.0 && b.flames.iter().any(|f| (f.0 - px).abs() < 30.0) && (py - b.home.1).abs() < 8.0;
        if b.flame_tick <= 0.0 {
            b.flame_tick = 0.5;
        }
        if b.state != BS::Slam {
            b.b.vx = vx;
            b.b.vy = (b.b.vy + GRAV * dt).min(900.0);
            let mut body = b.b;
            self.lv.step(&mut body, dt);
            self.boss.b = body;
        }
        for x in impacts {
            let gy = self.boss.home.1;
            self.boss.flames.push((x, 4.0));
            self.shake = self.shake.max(7.0);
            self.sfx_q.push("crash");
            self.fx.burst("ember", x, gy, 24, BurstOpt { speed: Some(180.0), up: Some(40.0), ..Default::default() });
            if (px - x).abs() < 44.0 {
                self.hurt_player(28.0, None, false, false);
            }
        }
        if in_flame {
            self.hurt_player(8.0, None, false, false);
        }
    }

    fn boss_die(&mut self) {
        let b = &mut self.boss;
        b.hp = 0.0;
        b.state = BS::Dead;
        b.t = 0.0;
        b.hazards.clear();
        b.flames.clear();
        b.anchor = None;
        let (x, y) = (b.b.x, b.b.y);
        self.save.boss = true;
        self.lv.gate_closed = false;
        self.gain_ash(2000);
        self.shake = 18.0;
        self.sfx_q.push("death");
        self.fx.burst("ember", x, y, 60, BurstOpt { speed: Some(200.0), up: Some(80.0), g: Some(-20.0), life: Some(2.0), ..Default::default() });
        self.banner("ASCHE VERWEHT", "Kapitän Barnaby ist gefallen · Der Pfad der Verdörrten Pilger öffnet sich", 6.0);
        self.end_t = Some(0.0);
        self.persist();
    }

    /// Kanonenkugeln: Flug mit Schwerkraft, Einschlag an Fels oder Spieler (abwehrbar, nicht parierbar)
    pub fn update_shots(&mut self, dt: f32) {
        let mut shots = std::mem::take(&mut self.shots);
        for s in shots.iter_mut() {
            s.3 += GRAV * 0.5 * dt;
            s.0 += s.2 * dt;
            s.1 += s.3 * dt;
            s.4 -= dt;
            let hit_p = (s.0 - self.p.b.x).abs() < 16.0 && s.1 > self.p.b.y - self.p.b.h && s.1 < self.p.b.y;
            let hit_w = self.lv.solid((s.0 / TS).floor() as i32, (s.1 / TS).floor() as i32);
            if hit_p || hit_w {
                s.4 = 0.0;
                self.fx.burst("ember", s.0, s.1 + 10.0, 12, BurstOpt { speed: Some(140.0), up: Some(10.0), ..Default::default() });
                self.fx.ring(s.0, s.1, 4.0, 40.0, 0.25, [1.0, 0.6, 0.3]);
                self.sfx_q.push("crash");
                if hit_p || (s.0 - self.p.b.x).abs() < 36.0 && (s.1 - self.p.b.y).abs() < 36.0 {
                    self.hurt_player(24.0, None, false, true);
                }
            }
        }
        shots.retain(|s| s.4 > 0.0);
        self.shots = shots;
    }
}
