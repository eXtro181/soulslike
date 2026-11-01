//! Kacheln für die 2D-Seitenansicht von Eclipse Ashes: Säulenbasalt mit Aschekruste, Rückwand,
//! versteinerte Planken, Leitern und die Winde der Abkürzungen. Kachelgröße 32 px.

use crate::canvas::Canvas;
use crate::color::*;
use crate::noise::{h01, Noise};

pub const T: i32 = 32;

/// Ganze Kachel auf eine Höhe über Grund setzen (sonst entstünde je Kachel ein Höhenverlauf).
fn flat_ev(c: &mut Canvas, e: f32) {
    c.ev.iter_mut().for_each(|v| *v = e);
}

/// Säulenbasalt: zwei Säulen je Kachel, Fugen an den Kachelrändern (so fallen die Nähte nicht auf).
/// `top` = oberste Kachel mit Aschekruste und glimmenden Funken, `dark` = Rückwand hinter Höhlen.
pub fn rock(v: i32, top: bool, dark: bool) -> Canvas {
    let mut c = Canvas::new(T, T);
    let nz = Noise::new(31 + v as u32 * 7 + dark as u32 * 101);
    let p = pal(&[0x262326, 0x353034, 0x464044, 0x5A5356, 0x70686A, 0x88807E]);
    let k = if dark { 0.55 } else { 1.0 };
    // zwei Säulen je Kachel, Teilung je Variante verschoben (12, 16, 20 px), damit kein Vorhangmuster entsteht
    let split = 12 + v * 4;
    for col in 0..2 {
        let (x0, cw) = if col == 0 { (0, split) } else { (split, T - split) };
        // eine waagrechte Bruchkante je Säule
        let crack = 6 + (h01(v, col, 3) * 20.0) as i32;
        let half = (cw - 1) as f32 * 0.5;
        for y in 0..T {
            for dx in 0..cw {
                let x = x0 + dx;
                let edge = dx == 0 || dx == cw - 1;
                // Wölbung der Säule: Mitte hoch, Ränder tief → Licht von links oben
                let bulge = 1.0 - ((dx as f32 - half) / half).powi(2);
                let n = nz.fbm(x as f32 * 0.11, y as f32 * 0.07, 3) * 0.5 + 0.5;
                let mut cc = pick(&p, (n * 0.6 + bulge * 0.35 + 0.05).min(0.99));
                let mut rel = bulge * 3.5 + n;
                if edge || y == crack {
                    cc = shade(cc, 0.45);
                    rel = 0.0;
                }
                c.put(x, y, shade(cc, k), rel * k);
            }
        }
    }
    if top && !dark {
        for x in 0..T {
            let d = 3 + (h01(x / 3 + v * 11, 0, 4) * 4.0) as i32;
            for y in 0..d {
                let t = y as f32 / d as f32;
                c.put(x, y, mix(hex(0x8E8682), hex(0x4A4446), t), 4.0 - t * 3.0);
            }
            if h01(x, v, 8) > 0.93 {
                c.lit(x, d, hex(0xE06A2A), 0.9);
            }
        }
    }
    flat_ev(&mut c, if dark { 0.0 } else { 6.0 });
    c
}

/// Versteinerte Schiffsplanke (Plattform, von unten durchspringbar) mit Eisenwinkel.
pub fn plank(v: i32) -> Canvas {
    let mut c = Canvas::new(T, 12);
    let p = pal(&[0x2A221E, 0x3A302A, 0x4C4038, 0x5E5046, 0x726254, 0x8A7864]);
    for y in 0..7 {
        for x in 0..T {
            let grain = ((x as f32 * 0.4 + (y as f32 * 1.7).sin() * 2.0 + v as f32).sin() * 0.5 + 0.5) * 0.4;
            let t = if y == 0 { 0.95 } else if y == 6 { 0.1 } else { 0.35 + grain };
            c.put(x, y, pick(&p, t), if y == 0 { 2.0 } else { 1.0 });
        }
    }
    for &nx in &[4, 27] {
        c.lit(nx, 3, hex(0x9A8A7A), 0.0);
    }
    // Winkel unter der Planke
    let iron = hex(0x2C2A2E);
    for i in 0..5 {
        c.put(6 + i, 7 + i, iron, 1.0);
        c.put(25 - i, 7 + i, iron, 1.0);
    }
    flat_ev(&mut c, 8.0);
    c
}

/// Leiter aus versteinertem Holz; `chain` = Kettenleiter der Abkürzung.
pub fn ladder(chain: bool) -> Canvas {
    let mut c = Canvas::new(T, T);
    let (rail, rung) = if chain { (hex(0x55525A), hex(0x6E6A72)) } else { (hex(0x4C4038), hex(0x6A5A4C)) };
    for y in 0..T {
        for &x in &[7, 24] {
            for d in 0..3 {
                let r = if chain && (y + d) % 4 == 0 { shade(rail, 0.6) } else { rail };
                c.put(x + d, y, if d == 0 { shade(r, 1.25) } else { r }, 2.0 - d as f32 * 0.5);
            }
        }
    }
    for y in (3..T).step_by(8) {
        for x in 10..24 {
            c.put(x, y, rung, 1.5);
            c.put(x, y + 1, shade(rung, 0.6), 0.5);
        }
    }
    flat_ev(&mut c, 10.0);
    c
}

/// Winde mit Kurbel und Kettentrommel; `wound` = schon gedreht (Kette abgerollt).
pub fn winch(wound: bool) -> Canvas {
    let mut c = Canvas::new(40, 44);
    let wood = pal(&[0x2A221E, 0x3A302A, 0x4C4038, 0x5E5046, 0x726254, 0x8A7864]);
    let iron = pal(&[0x1A181C, 0x29262C, 0x3A363E, 0x4E4954, 0x66606C, 0x847C88]);
    // Gestell
    c.tube(&[(8.0, 43.0), (14.0, 14.0)], 2.5, 2.0, &wood, 1.5, 1.0);
    c.tube(&[(32.0, 43.0), (26.0, 14.0)], 2.5, 2.0, &wood, 1.5, 1.0);
    // Trommel
    c.dome(20.0, 18.0, 10.0, 8.0, &iron, 4.0, 1.0, 0.3, 0.1);
    let turns = if wound { 1 } else { 4 };
    for i in 0..turns {
        c.hline(11, 29, 13 + i * 3, hex(0x7A7480));
    }
    // Kurbel
    c.tube(&[(20.0, 18.0), (34.0, 8.0)], 1.6, 1.4, &iron, 1.0, 1.0);
    c.disc(34.0, 8.0, 2.4, hex(0x8A5A3A));
    c.outline(true, 0.7);
    flat_ev(&mut c, 12.0);
    c
}
