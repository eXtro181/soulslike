//! Eclipse Ashes – 2D-Dark-Fantasy-Soulslike in HD-Pixelart (Rust/Bevy, eigener wgpu-Renderer).
//! Alles – Grafik, Welt, Musik – wird beim Start aus Code erzeugt.

mod art;
mod audio;
mod game;
// Renderer-Schnittstelle vollständig behalten (Boden-Chunks, Wasser, Spiegelungen): die Shader
// können es, spätere Zonen (Pechseen, Katakomben) brauchen es wieder.
#[allow(dead_code)]
mod gfx;
mod prof;
mod synth;
mod ui;

pub use art::Art;
use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::prelude::*;
use bevy::window::{CursorMoved, PrimaryWindow, WindowResolution};
use game::{Game, Mode};
use gfx::*;
use ui::touch::TouchPad;

#[derive(Resource)]
struct App {
    game: Game,
    touch: TouchPad,
    t: f32,
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    bevy::app::App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Eclipse Ashes".into(),
                        canvas: Some("#bevy".into()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        resolution: WindowResolution::new(1280.0, 720.0),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin { meta_check: bevy::asset::AssetMetaCheck::Never, ..default() }),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(GfxPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (resize, tick).chain())
        .run();
}

fn noise_texture(f: &mut Frame) {
    let nz = sw_art::noise::Noise::new(7);
    let nz2 = sw_art::noise::Noise::new(8);
    let mut data = vec![0u8; 256 * 256 * 4];
    for y in 0..256 {
        for x in 0..256 {
            let o = (y * 256 + x) * 4;
            let r = nz.fbmp(x as f32 / 64.0, y as f32 / 64.0, 4, 4) * 0.5 + 0.5;
            let (d1, d2, _) = sw_art::noise::worley(x as f32 / 32.0, y as f32 / 32.0, 8, 3);
            let g = 1.0 - ((d2 - d1) / 0.12).clamp(0.0, 1.0);
            let b = nz2.fbmp(x as f32 / 32.0, y as f32 / 32.0, 3, 8) * 0.5 + 0.5;
            let a = sw_art::noise::h01(x as i32, y as i32, 5);
            data[o] = (r * 255.0) as u8;
            data[o + 1] = (g * 255.0) as u8;
            data[o + 2] = (b * 255.0) as u8;
            data[o + 3] = (a * 255.0) as u8;
        }
    }
    f.uploads.push(Upload { tex: Tex::Noise, layer: 0, x: 0, y: 0, w: 256, h: 256, data });
}

fn setup(world: &mut bevy::ecs::world::World) {
    let mut f = world.remove_resource::<Frame>().unwrap_or_default();
    noise_texture(&mut f);
    let art = art::build(&mut f);
    #[cfg(target_arch = "wasm32")]
    {
        let params = web_sys::window().and_then(|w| w.location().search().ok()).unwrap_or_default();
        if params.contains("prof") {
            prof::enable();
        }
        if let Some(el) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id("load")) {
            el.remove();
        }
    }
    info!("Welt erzeugt");
    // nur der Browser liest die Testlauf-Parameter (nativ bleibt das Spiel unverändert)
    #[allow(unused_mut)]
    let mut game = Game::new();
    // Testläufe: ?play startet sofort ein neues Spiel, &pos=x,y setzt den Spieler auf eine Kachel
    #[cfg(target_arch = "wasm32")]
    {
        let q = web_sys::window().and_then(|w| w.location().search().ok()).unwrap_or_default();
        if q.contains("play") {
            game.start(false);
            game.banner = None;
            if let Some(v) = q.split(['?', '&']).find_map(|kv| kv.strip_prefix("pos=")) {
                let n: Vec<f32> = v.split(',').filter_map(|x| x.parse().ok()).collect();
                if n.len() == 2 {
                    game.p.b.x = n[0] * 32.0 + 16.0;
                    game.p.b.y = (n[1] + 1.0) * 32.0;
                    game.cam = (game.p.b.x, game.p.b.y);
                }
            }
        }
    }
    world.insert_resource(f);
    world.insert_resource(art);
    world.insert_resource(App { game, touch: TouchPad::new(), t: 0.0 });
    world.insert_non_send_resource(audio::Audio::new());
}

fn resize(win: Query<&Window, With<PrimaryWindow>>, mut f: ResMut<Frame>) {
    let Ok(w) = win.single() else { return };
    let (pw, ph) = (w.physical_width().max(1), w.physical_height().max(1));
    let short = pw.min(ph) as f32;
    let k = (short / 540.0).max(0.5);
    f.low = (((pw as f32) / k).round() as u32, ((ph as f32) / k).round() as u32);
    f.screen = (pw, ph);
    f.ui_scale = (short / 360.0).round().max(1.0);
}

#[allow(clippy::too_many_arguments)]
fn tick(
    mut f: ResMut<Frame>,
    mut app: ResMut<App>,
    art: Res<Art>,
    mut audio: NonSendMut<audio::Audio>,
    time: Res<Time>,
    kb: Res<ButtonInput<KeyCode>>,
    mb: Res<ButtonInput<MouseButton>>,
    mut touches: EventReader<TouchInput>,
    mut cursor: EventReader<CursorMoved>,
    win: Query<&Window, With<PrimaryWindow>>,
) {
    let dt = time.delta_secs().min(0.05);
    let app = &mut *app;
    let f = &mut *f;
    // FullHD: interne Auflösung verdoppeln, sobald der Bildschirm groß genug ist
    let short = f.screen.0.min(f.screen.1);
    f.rs = if short >= 900 { 2 } else { 1 };
    f.g.hd3 = [1.0, f.rs as f32, 0.0, 0.0];
    app.t += dt;
    prof::frame();
    f.begin();
    let Ok(window) = win.single() else { return };
    let sf = window.scale_factor();
    let to_ui = |p: Vec2| p * sf / f.ui_scale.max(1.0);
    // ---------------------------------------------------------------- Eingabe
    let g = &mut app.game;
    let inp = &mut g.input;
    inp.feed(&kb);
    let moved = cursor.read().last().is_some();
    if let Some(p) = window.cursor_position() {
        inp.mouse = to_ui(p);
    }
    inp.mouse_moved = moved;
    if moved {
        inp.using_touch = false;
    }
    inp.click = mb.just_pressed(MouseButton::Left);
    let (mut starts, mut moves, mut ends) = (vec![], vec![], vec![]);
    for t in touches.read() {
        let p = to_ui(t.position);
        match t.phase {
            TouchPhase::Started => starts.push((t.id, p)),
            TouchPhase::Moved => moves.push((t.id, p)),
            _ => ends.push(t.id),
        }
    }
    if !starts.is_empty() {
        inp.using_touch = true;
    }
    // im Spiel steuert der Touch-Stick, in Menüs wirkt Tippen wie ein Klick
    let playing = g.mode == Mode::Play;
    app.touch.visible = inp.using_touch && playing;
    if playing {
        app.touch.feed(inp, f.ui_size(), &starts, &moves, &ends);
    } else {
        app.touch.feed(inp, f.ui_size(), &[], &[], &ends);
        if let Some(&(_, p)) = starts.first() {
            inp.mouse = p;
            inp.click = true;
        }
    }
    // ---------------------------------------------------------------- Spiel
    if g.mode == Mode::Title {
        // Titel: langsame Kamerafahrt durch die Halle über den Klippen
        g.cam = (380.0 + (app.t * 0.05).sin() * 220.0, 200.0 + (app.t * 0.03).cos() * 40.0);
    }
    prof::time(prof::UPDATE, || g.update(dt));
    prof::time(prof::DRAW, || g.draw(&art, f, false));
    prof::time(prof::UI, || g.ui(&art, f));
    app.touch.draw(f, &art);
    audio.set_mood(g.mood);
    for s in g.sfx_q.drain(..) {
        audio.sfx(s, 1.0);
    }
    prof::time(prof::AUDIO, || audio.update(dt, None));
    g.input.end_frame();
}
