#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Enemy;