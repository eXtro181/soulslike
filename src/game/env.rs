//! Lichtstimmung von Eclipse Ashes (Dark Fantasy): Umgebungslicht, Glutsonne, Aschedunst,
//! Farbkorrektur. Die Albedo bleibt farbig; die Stimmung kommt aus dem Licht.

use crate::gfx::{lin, Globals};

#[derive(Clone, Copy)]
pub struct Env {
    pub ambient: [f32; 3],
    pub sky: f32,
    pub sun_dir: [f32; 3],
    pub sun: [f32; 3],
    pub shadow: f32,
    pub fog: [f32; 3],
    pub fog_k: f32,
    pub lift: [f32; 3],
    pub gain: [f32; 3],
    pub contrast: f32,
    pub sat: f32,
    pub bloom: f32,
    pub vignette: f32,
    pub water_a: [f32; 3],
    pub water_b: [f32; 3],
    pub water_c: [f32; 3],
    pub abyss: f32,
    /// Lichtstrahlen (HD-2D), skaliert mit der Sonne
    pub rays: f32,
}

fn l(c: u32, k: f32) -> [f32; 3] {
    let v = lin(c);
    [v[0] * k, v[1] * k, v[2] * k]
}
fn n3(v: [f32; 3]) -> [f32; 3] {
    let m = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / m, v[1] / m, v[2] / m]
}

/// Aschenene Klippen: roter Dunst, Glut, kaum Sonne (Werte aus der alten Aschebucht)
pub fn ash() -> Env {
    Env {
        ambient: l(0x948A94, 0.9),
        sky: 0.4,
        sun_dir: n3([-0.55, -0.2, 0.8]),
        sun: l(0xFF9A7A, 0.5),
        shadow: 0.35,
        fog: l(0x4E3E40, 1.0),
        fog_k: 0.05,
        lift: [0.012, 0.008, 0.01],
        gain: [1.0, 0.98, 0.97],
        contrast: 1.06,
        sat: 0.72,
        bloom: 0.55,
        vignette: 0.75,
        water_a: l(0x6A3A50, 1.0),
        water_b: l(0x2A1426, 1.0),
        water_c: l(0xE8A0A8, 1.0),
        abyss: 1.0,
        rays: 0.6,
    }
}

/// In die Shader-Konstanten schreiben. `dark` = Option „Düster“, `exposure` = Helligkeit.
pub fn apply(e: &Env, g: &mut Globals, dark: bool, exposure: f32, fx_high: bool) {
    // Dark Fantasy: gedämpftes Umgebungslicht, tief stehende Sonne, Lichtinseln durch Laternen
    let dk = if dark { 0.5 } else { 0.68 };
    let sk = if dark { 0.55 } else { 0.72 };
    g.ambient = [e.ambient[0] * dk, e.ambient[1] * dk, e.ambient[2] * dk, e.sky];
    g.sun_dir = [e.sun_dir[0], e.sun_dir[1], e.sun_dir[2], if fx_high { e.shadow + 0.15 } else { 0.0 }];
    g.sun_col = [e.sun[0] * sk, e.sun[1] * sk, e.sun[2] * sk, 0.6];
    g.fog = [e.fog[0], e.fog[1], e.fog[2], e.fog_k];
    g.lift = [e.lift[0] + 0.004, e.lift[1] + 0.008, e.lift[2] + 0.016, e.contrast + 0.06];
    g.gain = [e.gain[0], e.gain[1], e.gain[2], e.sat * if dark { 0.75 } else { 1.0 }];
    // +15 % Helligkeit gegenüber der ursprünglichen Abstimmung (Wunsch: man sieht sonst zu wenig)
    g.misc = [(e.vignette + 0.2).min(1.0), if fx_high { 0.018 } else { 0.0 }, if fx_high { e.bloom } else { 0.2 }, exposure * 1.15];
    g.water_a = [e.water_a[0], e.water_a[1], e.water_a[2], 1.0];
    g.water_b = [e.water_b[0], e.water_b[1], e.water_b[2], 1.0];
    g.water_c = [e.water_c[0], e.water_c[1], e.water_c[2], 0.3];
    g.map[3] = e.abyss;
    // HD-2D: Schrägsicht immer, Tiefenschärfe, Verdeckung, Strahlen und Staub nur bei hohen Effekten
    let hk = if fx_high { 1.0 } else { 0.0 };
    g.hd = [crate::gfx::TILT, hk, g.hd[2], e.rays * hk * if dark { 0.7 } else { 1.0 }];
    g.hd2 = [hk, 0.6 * hk, 0.55, 0.5 * hk];
}
