//! Oberfläche in UI-Pixeln: Lebens-, Ausdauer- und Glutbalken, Boss- und Haltungsleisten, Hinweise,
//! Banner sowie die Menüs (Titel, Pause, Steuerung, Ascheschrein mit Katha und Corvus).

use super::fight::*;
use super::input::Act;
use super::{Game, Mode};
use crate::art::Art;
use crate::gfx::{Frame, SprOpt};
use crate::ui::text::{self, col, Al};

const CONTROLS: [(&str, &str); 12] = [
    ("A D / ←→", "Laufen"),
    ("W S", "Leitern klettern"),
    ("Leertaste", "Springen (S + Leertaste: durch Planken fallen)"),
    ("Umschalt / K", "Rolle (unverwundbar in Bild 3–17)"),
    ("J", "Leichter Hieb · Hinrichtung"),
    ("L", "Schwerer Hieb"),
    ("Q halten", "Schild · kurz vor dem Treffer: Parade"),
    ("C", "Aschenkunst"),
    ("R", "Aschentrank"),
    ("E", "Rasten, Winde, Aufheben"),
    ("Esc", "Pause"),
    ("Touch", "Stick links, Knöpfe rechts"),
];

impl Game {
    /// Menüs bedienen und die ganze Oberfläche zeichnen.
    pub fn ui(&mut self, art: &Art, f: &mut Frame) {
        let (uw, uh) = f.ui_size();
        match self.mode {
            Mode::Title => self.title(art, f, uw, uh),
            Mode::Play => self.hud(art, f, uw, uh),
            Mode::Pause => {
                self.hud(art, f, uw, uh);
                f.ui_rect(0.0, 0.0, uw, uh, [6, 6, 8, 150]);
                text::draw(f, art, "PAUSE", uw * 0.5, uh * 0.3, col::BONE, Al::C, 2.0, true);
                match self.menu(art, f, &["Weiter", "Steuerung", "Zum Titel"], uw * 0.5, uh * 0.42, None) {
                    Some(0) => self.mode = Mode::Play,
                    Some(1) => self.mode = Mode::Controls,
                    Some(_) => {
                        *self = Game::new();
                    }
                    None if self.input.pressed(Act::Menu) => self.mode = Mode::Play,
                    None => {}
                }
            }
            Mode::Controls => {
                f.ui_rect(0.0, 0.0, uw, uh, [6, 6, 8, 200]);
                text::draw(f, art, "STEUERUNG", uw * 0.5, 30.0, col::BONE, Al::C, 2.0, true);
                for (i, (k, v)) in CONTROLS.iter().enumerate() {
                    let y = 64.0 + i as f32 * 16.0;
                    text::draw(f, art, k, uw * 0.5 - 8.0, y, col::GOLD, Al::R, 1.0, true);
                    text::draw(f, art, v, uw * 0.5 + 8.0, y, col::BONE, Al::L, 1.0, true);
                }
                text::draw(f, art, "Esc / Enter: zurück", uw * 0.5, uh - 24.0, col::MIST, Al::C, 1.0, true);
                if self.input.pressed(Act::Menu) || self.input.pressed(Act::Confirm) || self.input.pressed(Act::Attack) || self.input.click {
                    self.mode = if self.mood == "title" { Mode::Title } else { Mode::Pause };
                    self.sel = 0;
                }
            }
            Mode::Shrine => {
                self.hud(art, f, uw, uh);
                self.shrine_menu(art, f, uw, uh);
            }
        }
    }

    /// Einfache Liste: W/S wählen, J/E/Enter bestätigen, Maus und Tippen. Gibt die gewählte Zeile zurück.
    fn menu(&mut self, art: &Art, f: &mut Frame, items: &[&str], x: f32, y: f32, desc: Option<&[String]>) -> Option<usize> {
        let n = items.len();
        self.sel = self.sel.min(n - 1);
        if self.input.pressed(Act::Down) {
            self.sel = (self.sel + 1) % n;
            self.sfx_q.push("click");
        }
        if self.input.pressed(Act::Up) {
            self.sel = (self.sel + n - 1) % n;
            self.sfx_q.push("click");
        }
        let w = items.iter().map(|s| text::width(art, s)).fold(120.0, f32::max) + 24.0;
        let mut pick = None;
        for (i, s) in items.iter().enumerate() {
            let yy = y + i as f32 * 18.0;
            let m = self.input.mouse;
            let hover = m.x > x - w * 0.5 && m.x < x + w * 0.5 && m.y > yy - 3.0 && m.y < yy + 13.0;
            if hover && (self.input.mouse_moved || self.input.click) {
                self.sel = i;
            }
            if hover && self.input.click {
                pick = Some(i);
            }
            let on = i == self.sel;
            f.ui_rect(x - w * 0.5, yy - 3.0, w, 16.0, if on { [60, 30, 22, 220] } else { [11, 12, 16, 190] });
            if on {
                f.ui_rect(x - w * 0.5, yy - 3.0, 2.0, 16.0, col::GOLD);
            }
            text::draw(f, art, s, x, yy, if on { col::GOLD } else { col::BONE_DIM }, Al::C, 1.0, true);
        }
        if let Some(d) = desc {
            if let Some(s) = d.get(self.sel) {
                text::draw(f, art, s, x, y + n as f32 * 18.0 + 8.0, col::MIST, Al::C, 1.0, true);
            }
        }
        if self.input.pressed(Act::Confirm) || self.input.pressed(Act::Attack) || self.input.pressed(Act::Interact) {
            pick = Some(self.sel);
        }
        if pick.is_some() {
            self.sfx_q.push("click");
        }
        pick
    }

    fn title(&mut self, art: &Art, f: &mut Frame, uw: f32, uh: f32) {
        let logo = art.spr("ui:logo");
        let sc = ((uw * 0.7) / logo.w as f32).clamp(1.0, 3.0).floor();
        f.ui_spr(&logo, uw * 0.5 - logo.w as f32 * sc * 0.5, uh * 0.16, SprOpt { scale: sc, ..Default::default() });
        text::draw(f, art, "Die Asche des ertrunkenen Throns", uw * 0.5, uh * 0.16 + logo.h as f32 * sc + 8.0, col::MIST, Al::C, 1.0, true);
        let mut items = vec!["Neues Spiel"];
        if self.has_save {
            items.insert(0, "Fortsetzen");
        }
        items.push("Steuerung");
        let has = self.has_save;
        match self.menu(art, f, &items, uw * 0.5, uh * 0.55, None) {
            Some(i) if has && i == 0 => self.start(true),
            Some(i) if i == items.len() - 1 => self.mode = Mode::Controls,
            Some(_) => self.start(false),
            None => {}
        }
        text::draw(f, art, "Zone 01 von 15 · Die Aschenen Klippen von Calamita", uw * 0.5, uh - 18.0, col::GREY, Al::C, 1.0, false);
    }

    fn shrine_menu(&mut self, art: &Art, f: &mut Frame, uw: f32, uh: f32) {
        let s = &self.save;
        let cost = s.level_cost();
        let art_name = art_def(&s.arts[s.art.min(s.arts.len() - 1)]).name;
        let lines = [
            "Weiterziehen".to_string(),
            format!("Katha · Vitalität {} → {}", s.vit, s.vit + 1),
            format!("Katha · Ausdauer {} → {}", s.end, s.end + 1),
            format!("Katha · Kraft {} → {}", s.str, s.str + 1),
            format!("Corvus · Aschenkunst: {art_name}"),
            format!("Corvus · Prägung: {}", AFFS[s.aff].0),
        ];
        let a = art_def(&s.arts[s.art.min(s.arts.len() - 1)]);
        let desc = [
            "Der Schrein glimmt weiter. Die Toten stehen wieder auf.".to_string(),
            format!("Mehr Leben. Kosten: {cost} Asche (du hast {})", s.ash),
            format!("Mehr Ausdauer. Kosten: {cost} Asche (du hast {})", s.ash),
            format!("Mehr Schaden. Kosten: {cost} Asche (du hast {})", s.ash),
            format!("{} ({} Glut) · {} von {} Kriegsaschen", a.desc, a.fp, s.arts.len(), ARTS.len()),
            AFFS[s.aff].1.to_string(),
        ];
        f.ui_rect(0.0, 0.0, uw, uh, [6, 4, 4, 120]);
        text::draw(f, art, "ASCHESCHREIN", uw * 0.5, uh * 0.2, col::GOLD, Al::C, 2.0, true);
        text::draw(f, art, &format!("Stufe {} · Asche {}", s.level(), s.ash), uw * 0.5, uh * 0.2 + 20.0, col::BONE_DIM, Al::C, 1.0, true);
        let items: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
        let pick = self.menu(art, f, &items, uw * 0.5, uh * 0.36, Some(&desc));
        match pick {
            Some(0) => self.mode = Mode::Play,
            Some(i @ 1..=3) => {
                if self.save.ash < cost {
                    self.toast("Zu wenig Asche.");
                } else {
                    self.save.ash -= cost;
                    match i {
                        1 => self.save.vit += 1,
                        2 => self.save.end += 1,
                        _ => self.save.str += 1,
                    }
                    self.p.hp = self.max_hp();
                    self.p.st = self.max_st();
                    self.sfx_q.push("level");
                    self.persist();
                }
            }
            Some(4) => {
                self.save.art = (self.save.art + 1) % self.save.arts.len();
                self.persist();
            }
            Some(_) => {
                self.save.aff = (self.save.aff + 1) % AFFS.len();
                self.persist();
            }
            None if self.input.pressed(Act::Menu) => self.mode = Mode::Play,
            None => {}
        }
    }

    fn bar(f: &mut Frame, x: f32, y: f32, w: f32, h: f32, k: f32, c: [u8; 4]) {
        f.ui_rect(x - 1.0, y - 1.0, w + 2.0, h + 2.0, [122, 90, 62, 255]);
        f.ui_rect(x, y, w, h, [20, 14, 14, 255]);
        f.ui_rect(x, y, w * k.clamp(0.0, 1.0), h, c);
    }

    fn hud(&mut self, art: &Art, f: &mut Frame, uw: f32, uh: f32) {
        let p = &self.p;
        Self::bar(f, 12.0, 12.0, self.max_hp() * 1.2, 6.0, p.hp / self.max_hp(), col::BLOOD_L);
        Self::bar(f, 12.0, 22.0, self.max_fp() * 1.6, 4.0, p.fp / self.max_fp(), col::SOUL);
        Self::bar(f, 12.0, 30.0, self.max_st() * 1.2, 4.0, p.st / self.max_st(), col::GOLD);
        let a = art_def(&self.save.arts[self.save.art.min(self.save.arts.len() - 1)]);
        text::draw(f, art, &format!("Trank ×{}", p.flasks), 12.0, uh - 34.0, col::BONE, Al::L, 1.0, true);
        text::draw(f, art, &format!("{} · {}", a.name, AFFS[self.save.aff].0), 12.0, uh - 20.0, col::SOUL, Al::L, 1.0, true);
        text::draw(f, art, &format!("Asche {}", self.save.ash), uw - 12.0, uh - 20.0, col::GOLD, Al::R, 1.0, true);
        // kleine Leisten über verletzten Gegnern: Leben (rot) und Haltung (gold)
        for fo in &self.foes {
            let d = foe_def(fo.kind);
            if fo.state == FS::Dead || (fo.hp >= d.hp && fo.post <= 0.0) || fo.kind == Kind::Gunner {
                continue;
            }
            let (u, v, _) = f.world_to_screen(fo.b.x, fo.b.y - fo.b.h - 10.0);
            let (x, y) = (u * uw - 14.0, v * uh);
            Self::bar(f, x, y, 28.0, 2.0, fo.hp / d.hp, col::BLOOD_L);
            Self::bar(f, x, y + 4.0, 28.0, 1.0, fo.post / d.post, if fo.state == FS::Broken { col::BONE } else { col::GOLD });
        }
        // Boss- und Miniboss-Leiste unten
        let mut bars: Vec<(&str, f32, f32, bool)> = vec![];
        if let Some(g) = self.foes.iter().find(|fo| fo.kind == Kind::Gunner && !matches!(fo.state, FS::Idle | FS::Dead)) {
            let d = foe_def(Kind::Gunner);
            bars.push((d.name, g.hp / d.hp, g.post / d.post, g.state == FS::Broken));
        }
        if self.boss.active() {
            bars.push((BOSS_NAME, self.boss.hp / BOSS_HP, self.boss.post / BOSS_POST, self.boss.state == BS::Broken));
        }
        for (i, (name, hp, post, broken)) in bars.iter().enumerate() {
            let w = (uw * 0.6).min(320.0);
            let y = uh - 52.0 - i as f32 * 26.0;
            text::draw(f, art, name, uw * 0.5 - w * 0.5, y - 12.0, col::BONE, Al::L, 1.0, true);
            Self::bar(f, uw * 0.5 - w * 0.5, y, w, 5.0, *hp, col::BLOOD_L);
            Self::bar(f, uw * 0.5 - w * 0.25, y + 8.0, w * 0.5, 2.0, *post, if *broken { col::BONE } else { col::GOLD });
        }
        if let Some(s) = self.interaction() {
            text::draw(f, art, s, uw * 0.5, uh - 92.0, col::BONE, Al::C, 1.0, true);
        }
        if let Some((s, _)) = &self.toast {
            text::draw(f, art, s, uw * 0.5, 44.0, col::BONE, Al::C, 1.0, true);
        }
        if let Some(t) = self.death_t {
            f.ui_rect(0.0, 0.0, uw, uh, [8, 4, 4, ((t / 2.0).min(0.8) * 255.0) as u8]);
        }
        if let Some((a, b, rest)) = &self.banner {
            let k = rest.min(1.0);
            let c = |cc: [u8; 4]| [cc[0], cc[1], cc[2], (k * 255.0) as u8];
            f.ui_rect(0.0, uh * 0.36 - 8.0, uw, if b.is_empty() { 34.0 } else { 46.0 }, [4, 4, 6, (k * 150.0) as u8]);
            text::draw(f, art, a, uw * 0.5, uh * 0.36, c(if a.starts_with("ZU ASCHE") { col::RED } else { col::GOLD }), Al::C, 2.0, true);
            if !b.is_empty() {
                text::draw(f, art, b, uw * 0.5, uh * 0.36 + 22.0, c(col::BONE), Al::C, 1.0, true);
            }
        }
    }
}
