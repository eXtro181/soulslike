//! Simulationstests: spielen Zone 01 ohne Grafikkarte Bild für Bild mit virtuellen Tasten durch.
//! `cargo test --bin schwarzwasser` (nativ, ohne wasm).

use super::fight::*;
use super::input::Act;
use super::level::TS;
use super::{Game, Mode};
use crate::gfx::Frame;

const DT: f32 = 1.0 / 60.0;

fn game() -> Game {
    let mut g = Game::new();
    g.start(false);
    g.banner = None;
    // ein paar Bilder, bis der Spieler am Boden steht
    step(&mut g, 0.1);
    g
}
/// Spieler auf Kachel (x, y) stellen (steht auf der Kachel darunter)
fn put(g: &mut Game, tx: i32, ty: i32) {
    g.p.b.x = tx as f32 * TS + TS * 0.5;
    g.p.b.y = (ty + 1) as f32 * TS;
    g.p.b.vx = 0.0;
    g.p.b.vy = 0.0;
}
fn step(g: &mut Game, secs: f32) {
    for _ in 0..(secs / DT) as i32 {
        g.update(DT);
        g.input.end_frame();
    }
}
fn press(g: &mut Game, a: Act) {
    g.input.v_press(a);
    g.input.v_release(a);
    g.update(DT);
    g.input.end_frame();
}
fn hold(g: &mut Game, a: Act, secs: f32) {
    g.input.v_held.insert(a);
    step(g, secs);
    g.input.v_held.remove(&a);
}
/// Gegner wegräumen, damit Tests nicht von Treffern gestört werden
fn no_foes(g: &mut Game) {
    g.foes.clear();
}

#[test]
fn map_check_and_start() {
    let g = game();
    assert_eq!(g.mode, Mode::Play);
    assert_eq!(g.foes.len(), 9);
    assert!(!g.lv.blocked_at(g.p.b.x, g.p.b.y, g.p.b.w, g.p.b.h), "Start im Fels");
}

#[test]
fn ladder_up_ends_on_top() {
    let mut g = game();
    no_foes(&mut g);
    put(&mut g, 18, 36);
    step(&mut g, 0.2);
    hold(&mut g, Act::Up, 4.0);
    assert_eq!(g.p.state, PS::Free);
    assert_eq!(g.p.b.y, 30.0 * TS, "oben auf Ebene 3 stehen");
    assert!(!g.lv.blocked_at(g.p.b.x, g.p.b.y, g.p.b.w, g.p.b.h));
}

#[test]
fn shortcut_ladder_reaches_hall_without_ceiling() {
    let mut g = game();
    no_foes(&mut g);
    g.lv.shortcut = true;
    put(&mut g, 2, 36);
    step(&mut g, 0.2);
    hold(&mut g, Act::Up, 12.0);
    assert_eq!(g.p.state, PS::Free);
    assert_eq!(g.p.b.y, 12.0 * TS, "auf Hallenhöhe aussteigen");
    assert!(!g.lv.blocked_at(g.p.b.x, g.p.b.y, g.p.b.w, g.p.b.h), "Kopf im Fels");
    // und nach rechts in die Halle laufen
    hold(&mut g, Act::Right, 0.5);
    assert!(g.p.b.x > 3.0 * TS);
}

#[test]
fn closed_shaft_is_a_drop() {
    let mut g = game();
    no_foes(&mut g);
    put(&mut g, 3, 11);
    hold(&mut g, Act::Left, 0.3);
    step(&mut g, 2.0);
    assert!(g.p.b.y > 30.0 * TS, "ohne Winde fällt man den Schacht hinab");
}

#[test]
fn drink_twice_heals_twice() {
    let mut g = game();
    no_foes(&mut g);
    let max = g.max_hp();
    g.p.hp = 5.0;
    press(&mut g, Act::Drink);
    step(&mut g, 0.95);
    press(&mut g, Act::Drink);
    step(&mut g, 2.0);
    assert!(g.p.hp > max * 0.85, "zwei Tränke heilen zweimal ({} von {max})", g.p.hp);
    assert_eq!(g.p.flasks, 1);
}

#[test]
fn roll_iframes_dodge_a_hit() {
    let mut g = game();
    no_foes(&mut g);
    press(&mut g, Act::Roll);
    step(&mut g, ROLL_IFR.0 + 2.0 * F);
    let hp = g.p.hp;
    assert!(!g.hurt_player(20.0, None, false, true), "in den I-Frames getroffen");
    assert_eq!(g.p.hp, hp);
}

#[test]
fn parry_breaks_and_execution_kills_marauder() {
    let mut g = game();
    g.foes.retain(|f| f.kind == Kind::Marauder);
    g.foes.truncate(1);
    put(&mut g, 8, 36);
    g.foes[0].b.x = g.p.b.x + 30.0;
    g.foes[0].b.y = g.p.b.y;
    g.p.face = 1.0;
    // Schild heben und den nächsten Treffer parieren
    g.input.v_press(Act::Block);
    g.update(DT);
    g.input.end_frame();
    assert!(g.p.blocking);
    assert!(!g.hurt_player(14.0, Some(Tgt::Foe(0)), true, true));
    assert_eq!(g.foes[0].state, FS::Broken);
    g.input.v_held.remove(&Act::Block);
    press(&mut g, Act::Attack);
    assert_eq!(g.p.state, PS::Exec);
    step(&mut g, 1.2);
    assert_eq!(g.foes[0].state, FS::Dead);
    assert!(g.save.ash >= 40);
}

#[test]
fn miniboss_death_is_saved() {
    let mut g = game();
    g.foes.retain(|f| f.kind == Kind::Gunner);
    assert_eq!(g.foes.len(), 1);
    put(&mut g, 13, 11);
    g.foes[0].b.x = g.p.b.x + 30.0;
    g.foes[0].b.y = g.p.b.y;
    g.foes[0].hp = 1.0;
    g.p.face = 1.0;
    press(&mut g, Act::Attack);
    step(&mut g, 0.5);
    assert_eq!(g.foes[0].state, FS::Dead);
    assert!(g.save.miniboss, "Sieg über den Miniboss nicht gespeichert");
    // nach dem Rasten bleibt er tot
    put(&mut g, 6, 36);
    press(&mut g, Act::Interact);
    assert!(!g.foes.iter().any(|f| f.kind == Kind::Gunner));
}

#[test]
fn shrine_and_pause_menus_stay_open() {
    let mut f = Frame::default();
    f.screen = (1280, 720);
    f.ui_scale = 2.0;
    f.low = (960, 540);
    let art = crate::art::build(&mut f);
    let mut g = game();
    no_foes(&mut g);
    put(&mut g, 6, 36);
    // E am Schrein: Rasten öffnet das Menü, dasselbe E darf es nicht wieder schließen
    g.input.v_press(Act::Interact);
    g.update(DT);
    g.ui(&art, &mut f);
    g.input.end_frame();
    assert_eq!(g.mode, Mode::Shrine);
    for _ in 0..5 {
        g.update(DT);
        g.ui(&art, &mut f);
        g.input.end_frame();
    }
    assert_eq!(g.mode, Mode::Shrine);
    press(&mut g, Act::Interact);
    g.input.v_press(Act::Interact);
    g.ui(&art, &mut f);
    g.input.end_frame();
    assert_eq!(g.mode, Mode::Play, "Weiterziehen schließt das Menü");
    // Esc öffnet die Pause und sie bleibt offen
    g.input.v_press(Act::Menu);
    g.update(DT);
    g.ui(&art, &mut f);
    g.input.end_frame();
    for _ in 0..5 {
        g.update(DT);
        g.ui(&art, &mut f);
        g.input.end_frame();
    }
    assert_eq!(g.mode, Mode::Pause);
}

#[test]
fn gate_closes_without_trapping_and_boss_kill_ends_chapter() {
    let mut g = game();
    no_foes(&mut g);
    put(&mut g, 29, 11);
    hold(&mut g, Act::Right, 1.2);
    assert_eq!(g.boss.state, BS::Intro);
    assert!(g.lv.gate_closed);
    let x = g.p.b.x;
    hold(&mut g, Act::Right, 0.3);
    assert!(g.p.b.x > x, "hinter dem Nebeltor eingeklemmt");
    // Boss fast tot neben den Spieler stellen und erschlagen
    step(&mut g, 2.0);
    g.boss.hp = 1.0;
    g.boss.state = BS::Idle;
    g.boss.cool = 5.0;
    g.boss.b.x = g.p.b.x + 40.0;
    g.boss.b.y = g.p.b.y;
    g.p.face = 1.0;
    g.p.iframes = 10.0;
    press(&mut g, Act::Attack);
    step(&mut g, 0.5);
    assert_eq!(g.boss.state, BS::Dead);
    assert!(g.save.boss && !g.lv.gate_closed);
    step(&mut g, 8.0);
    assert_eq!(g.mode, Mode::End);
}

#[test]
fn death_drops_ash_and_it_can_be_recovered() {
    let mut g = game();
    no_foes(&mut g);
    g.save.ash = 500;
    put(&mut g, 12, 36);
    let (x, y) = (g.p.b.x, g.p.b.y);
    g.hurt_player(999.0, None, false, false);
    assert_eq!(g.p.state, PS::Dead);
    step(&mut g, 4.0);
    assert!(g.death_t.is_none() && g.p.state == PS::Free, "Wiederbelebung");
    assert_eq!(g.save.ash, 0);
    assert_eq!(g.save.drop.map(|d| d.2), Some(500));
    g.p.b.x = x;
    g.p.b.y = y;
    step(&mut g, 0.1);
    assert_eq!(g.save.ash, 500);
    assert!(g.save.drop.is_none());
}

#[test]
fn boss_phase_two_and_tar_hazards() {
    let mut g = game();
    no_foes(&mut g);
    put(&mut g, 36, 11);
    step(&mut g, 2.0);
    assert!(g.boss.active());
    g.hurt_target(Tgt::Boss, BOSS_HP * 0.55, 0.0, false);
    assert_eq!(g.boss.phase, 2);
    g.p.iframes = 100.0;
    step(&mut g, 6.0);
    assert!(g.boss.hazards.len() + g.boss.flames.len() > 0, "Teerbomben in Phase 2");
}

/// zur Kachelspalte laufen (mit Zeitlimit)
fn walk_to(g: &mut Game, tx: i32) {
    let target = tx as f32 * TS + TS * 0.5;
    for _ in 0..1200 {
        let dx = target - g.p.b.x;
        if dx.abs() < 3.0 {
            break;
        }
        let a = if dx > 0.0 { Act::Right } else { Act::Left };
        g.input.v_held.insert(a);
        g.update(DT);
        g.input.end_frame();
        g.input.v_held.remove(&a);
    }
    step(g, 0.1);
}

#[test]
fn whole_route_start_to_boss_gate_is_walkable() {
    let mut g = game();
    no_foes(&mut g);
    // Schreinraum → Ebene 3 → 2 → 1 → Halle, jeweils über die Leiter auf der Gegenseite
    for (ladder_x, floor_row) in [(18, 30), (6, 24), (17, 18), (8, 12)] {
        walk_to(&mut g, ladder_x);
        assert!((g.p.b.x - (ladder_x as f32 * TS + TS * 0.5)).abs() < 4.0, "Leiter {ladder_x} nicht erreicht (x {})", g.p.b.x);
        hold(&mut g, Act::Up, 4.0);
        assert_eq!(g.p.b.y, floor_row as f32 * TS, "Leiter {ladder_x}: nicht oben angekommen");
        assert_eq!(g.p.state, PS::Free);
    }
    // Winde drehen, dann durch den Tunnel bis hinter das Nebeltor
    walk_to(&mut g, 5);
    press(&mut g, Act::Interact);
    assert!(g.lv.shortcut, "Winde nicht erreichbar");
    walk_to(&mut g, 34);
    assert!(g.boss.active(), "Arena nicht erreicht (x {})", g.p.b.x);
}
