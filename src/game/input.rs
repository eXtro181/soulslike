//! Aktionen aus Tastatur, Maus und Touch (virtuelle Knöpfe + Stick).

use bevy::input::keyboard::KeyCode as K;
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Act {
    Up,
    Down,
    Left,
    Right,
    Jump,
    Attack,
    Heavy,
    Roll,
    /// halten = Schild, kurz vor dem Treffer gedrückt = Parade
    Block,
    Drink,
    Interact,
    /// Aschenkunst der Waffe
    Art,
    Menu,
    Confirm,
}

const ALL: [Act; 14] = [Act::Up, Act::Down, Act::Left, Act::Right, Act::Jump, Act::Attack, Act::Heavy, Act::Roll, Act::Block, Act::Drink, Act::Interact, Act::Art, Act::Menu, Act::Confirm];

fn keys(a: Act) -> &'static [K] {
    use Act::*;
    match a {
        Up => &[K::KeyW, K::ArrowUp],
        Down => &[K::KeyS, K::ArrowDown],
        Left => &[K::KeyA, K::ArrowLeft],
        Right => &[K::KeyD, K::ArrowRight],
        Jump => &[K::Space],
        Attack => &[K::KeyJ],
        Heavy => &[K::KeyL],
        Roll => &[K::ShiftLeft, K::ShiftRight, K::KeyK],
        Block => &[K::KeyQ],
        Drink => &[K::KeyR],
        Interact => &[K::KeyE],
        Art => &[K::KeyC],
        Menu => &[K::Escape],
        Confirm => &[K::Enter, K::NumpadEnter],
    }
}

#[derive(Default)]
pub struct Input {
    held: HashSet<Act>,
    hits: HashSet<Act>,
    pub v_held: HashSet<Act>,
    pub v_hits: HashSet<Act>,
    pub stick: Vec2,
    pub touch_stick: Vec2,
    pub enabled: bool,
    pub any: bool,
    /// Maus in UI-Pixeln
    pub mouse: Vec2,
    pub mouse_moved: bool,
    pub click: bool,
    pub using_touch: bool,
}

impl Input {
    pub fn new() -> Self {
        Input { enabled: true, mouse: Vec2::new(-1.0, -1.0), ..Default::default() }
    }
    pub fn feed(&mut self, kb: &ButtonInput<KeyCode>) {
        self.held.clear();
        self.hits.clear();
        for a in ALL {
            let ks = keys(a);
            if ks.iter().any(|k| kb.pressed(*k)) {
                self.held.insert(a);
            }
            if ks.iter().any(|k| kb.just_pressed(*k)) {
                self.hits.insert(a);
                self.any = true;
                self.using_touch = false;
            }
        }
        if kb.get_just_pressed().next().is_some() {
            self.any = true;
        }
        self.stick = if self.touch_stick.length() > 0.05 { self.touch_stick } else { Vec2::ZERO };
    }
    pub fn held(&self, a: Act) -> bool {
        self.enabled && (self.held.contains(&a) || self.v_held.contains(&a))
    }
    pub fn pressed(&self, a: Act) -> bool {
        self.enabled && (self.hits.contains(&a) || self.v_hits.contains(&a))
    }
    pub fn axis(&self) -> Vec2 {
        if !self.enabled {
            return Vec2::ZERO;
        }
        let mut v = Vec2::new(
            (self.held(Act::Right) as i32 - self.held(Act::Left) as i32) as f32,
            (self.held(Act::Down) as i32 - self.held(Act::Up) as i32) as f32,
        );
        if self.stick.length() > 0.05 {
            v = self.stick;
        }
        if v.length() > 1.0 {
            v = v.normalize();
        }
        v
    }
    pub fn v_press(&mut self, a: Act) {
        self.v_hits.insert(a);
        self.v_held.insert(a);
        self.any = true;
    }
    pub fn v_release(&mut self, a: Act) {
        self.v_held.remove(&a);
    }
    pub fn end_frame(&mut self) {
        self.v_hits.clear();
        self.any = false;
        self.click = false;
    }
}
