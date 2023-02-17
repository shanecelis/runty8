#[cfg(feature = "std")]
use std::fmt::Display;

#[cfg(feature = "std")]
use itertools::Itertools;

use crate::serialize::Serialize;
use crate::sprite_sheet::SpriteSheet;
use display_utils::join;
use crate::alloc::string::ToString;

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use core::fmt::Display;

/// A pico8 game's flags.
#[derive(Debug, Clone)]
pub struct Flags {
    flags: [u8; SpriteSheet::SPRITE_COUNT],
}

#[cfg(feature = "std")]
impl Flags {
    pub fn file_name() -> String {
        "sprite_flags.txt".to_owned()
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let strings = self
                .flags
                .chunks(16)
                .map(|chunk| join(chunk.iter().map(|c| format!("{c:0>8b}")), " ").to_string());
        f.write_str(&join(strings, "\n").to_string())
    }
}

impl Flags {
    pub fn new() -> Self {
        let flags = [0; SpriteSheet::SPRITE_COUNT];

        Self { flags }
    }

    pub(crate) fn with_flags(flags: [u8; SpriteSheet::SPRITE_COUNT]) -> Self {
        Self { flags }
    }

    fn len(&self) -> usize {
        self.flags.len()
    }

    fn set(&mut self, index: usize, value: u8) {
        self.flags[index] = value;
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        // TODO: Check what pico8 does in cases when the index is out of bounds
        self.flags.get(index).copied()
    }

    // Pico8's fset(n, v)
    pub fn fset_all(&mut self, sprite: usize, flags: u8) -> u8 {
        self.set(sprite, flags);

        flags
    }

    // Pico8's fset(n, f, v)
    pub fn fset(&mut self, sprite: usize, flag: usize, value: bool) -> u8 {
        // TODO: Check what pico8 does in these cases:
        assert!(flag <= 7);

        let value = value as u8;
        let mut flags = self.get(sprite).unwrap();
        flags = (flags & !(1u8 << flag)) | (value << flag);

        self.set(sprite, flags);

        flags
    }

    pub fn fget_n(&self, sprite: usize, flag: u8) -> bool {
        // TODO: Check what pico8 does in these cases:
        assert!(sprite < self.len());
        assert!(flag <= 7);

        let res = (self.get(sprite).unwrap() & (1 << flag)) >> flag;
        assert!(res == 0 || res == 1);

        res != 0
    }

    pub fn deserialize(file_contents: &str) -> Result<Self, String> {
        // let flags_vec: Result<Vec<u8>, String> = file_contents
        let flags_vec: Result<Vec<u8>, String> = file_contents
            .lines()
            .map(
                |line| u8::from_str_radix(line, 2), //  line.parse::<u8>().map_err(|e| format!("{:?}", e))
            )
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|err| format!("{err:?}"));

        let flags_array: [u8; SpriteSheet::SPRITE_COUNT] =
            flags_vec?.try_into().map_err(|v: Vec<u8>| {
                format!(
                    "Incorrect number of elements, needed: {}, got: {}",
                    SpriteSheet::SPRITE_COUNT,
                    v.len()
                )
            })?;

        Ok(Self::with_flags(flags_array))
    }
}

impl Serialize for Flags {
    fn serialize(&self) -> String {
        join(self.flags
            .iter()
            .map(|flag| format!("{flag:0>8b}")),
            "\n").to_string()
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}
