// #![deny(missing_docs)]

//! Types and functions required to run a Runty8 game.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate core as std;
#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

mod draw_data;
mod flags;
mod input;
mod map;
mod pico8;
pub mod serialize;
mod sprite_sheet;
mod state;
pub use draw_data::colors;

pub mod draw;
pub mod font;

pub use flags::Flags;
pub use input::Input;
pub use map::Map;
pub use pico8::*;
pub use sprite_sheet::{Sprite, SpriteSheet};

/// A regular pico8 app.
pub trait App {
    fn init(pico8: &mut Pico8) -> Self;
    fn update(&mut self, pico8: &mut Pico8);
    fn draw(&mut self, pico8: &mut Pico8);
}

/// A pico8 color.
///
/// Valid colors are in the range `0..=15`.
pub type Color = u8; // Actually a u4

/// Pico8's supported input buttons.
#[derive(Debug)]
pub enum Button {
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// X key.
    X,
    /// C key.
    C,
    /// Left mouse button.
    Mouse,
}

/// Game assets: sprite sheet, map, flags.
// TODO: Rename to assets?
#[derive(Debug, Default)]
pub struct Resources {
    pub assets_path: &'static str,
    pub sprite_sheet: SpriteSheet,
    pub sprite_flags: Flags,
    pub map: Map,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Key state: up or down.
pub enum KeyState {
    Up,
    Down,
}
/// Keyboard keys.
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum Key {
    ///
    A,
    ///
    B,
    ///
    C,
    ///
    D,
    ///
    E,
    ///
    F,
    ///
    G,
    ///
    H,
    ///
    I,
    ///
    J,
    ///
    K,
    ///
    L,
    ///
    M,
    ///
    N,
    ///
    O,
    ///
    P,
    ///
    Q,
    ///
    R,
    ///
    S,
    ///
    T,
    ///
    U,
    ///
    V,
    ///
    W,
    ///
    X,
    ///
    Y,
    ///
    Z,
    ///
    Control,
    ///
    LeftArrow,
    ///
    RightArrow,
    ///
    UpArrow,
    ///
    DownArrow,
    ///
    Escape,
    ///
    Alt,
    ///
    Space,
}

/// Keyboard event (key up/down).
#[derive(Clone, Copy, Debug)]
pub struct KeyboardEvent {
    /// Key that was pressed or released.
    pub key: Key,
    /// Whether the key was pressed or released.
    pub state: KeyState,
}

/// Input events (mouse/keyboard).
#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    /// Keyboard event
    Keyboard(KeyboardEvent),
    /// Mouse event
    Mouse(MouseEvent),
}

/// Mouse buttons.
#[derive(Clone, Copy, Debug)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Middle mouse button
    Middle,
    /// Right mouse button
    Right,
}

/// Mouse events (mouse move, button presses).
#[derive(Clone, Copy, Debug)]
pub enum MouseEvent {
    /// Mouse move event.
    // Contains the current position of the mouse.
    Move {
        ///
        x: i32,
        ///
        y: i32,
    },
    /// Mouse button pressed/released.
    Button {
        /// Mouse button that was pressed or released.
        button: MouseButton,
        /// Whether the button was pressed or released.
        state: KeyState,
    },
}

/// Runty8 events (input, tick, etc).
#[derive(Clone, Copy, Debug)]
pub enum Event {
    ///
    Input(InputEvent),
    ///
    Tick {
        /// How much time passed since the last [`Event::Tick`], in milliseconds.
        delta_millis: f64,
    },
    // TODO: Remove this
    WindowClosed,
}

/// Embed game assets in your binary (that is, loading them at compile time).
#[macro_export]
macro_rules! load_assets {
    ($path:tt) => {{
        (|| {
            let map = $crate::Map::deserialize(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path, "/map.txt")))
                .expect("no map");

            let sprite_flags = $crate::Flags::deserialize(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path, "/sprite_flags.txt")))
                .expect("no flags");

            let sprite_sheet = $crate::SpriteSheet::deserialize(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path, "/sprite_sheet.txt")))
                .expect("no sprite sheet");
            Ok::<$crate::Resources, &str>($crate::Resources {
                map,
                sprite_flags,
                sprite_sheet,
                assets_path: $path
            })
        })()
    }};
}

#[macro_export]
macro_rules! load_assets_bin {
    ($path:tt) => {{
        (|| {
            let map = $crate::Map::from_bytes(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path, "/map.bin")).clone());

            let sprite_flags = $crate::Flags::from_bytes(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path, "/sprite_flags.bin")).clone());

            let sprite_sheet = $crate::SpriteSheet::from_bytes(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path, "/sprite_sheet.bin")).clone());
            Ok::<$crate::Resources, &str>($crate::Resources {
                map,
                sprite_flags,
                sprite_sheet,
                assets_path: $path
            })
        })()
    }};
}
