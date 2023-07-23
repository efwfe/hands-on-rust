pub use crate::prelude::*;


#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Render{
    pub color: ColorPair,
    pub glyph: FontCharType
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;
