extern crate graphics;
extern crate gfx;

use gfx::state::Rasterizer;
use graphics::{Packet, Vertex};

pub fn make_square_render() -> Packet {
    let vertices = vec!(
        Vertex::new([0.0, 0.0, 0.0], [1.0, 1.0]),
        Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0]),
        Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0]),
        Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0]),
    );

    let indices = vec!(
        0, 3, 2, 2, 1, 0,
    );

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

pub type RenderType = u8;
pub type Layer = u8;
pub type Name = &'static str;
pub type Size = &'static [f32; 2];
pub type Tint = &'static [f32; 4];
pub type Sprite = &'static [f32; 4];

pub mod layers {
    use ::Layer;

    pub const PLAYER_1_ARM: Layer = 1;
    pub const PLAYER_1_BODY: Layer = 2;
    pub const PLAYER_2_ARM: Layer = 3;
    pub const PLAYER_2_BODY: Layer = 4;
}

pub mod main {
    use ::{Name, RenderType, Size, Sprite, Tint};

    pub const NAME: Name = "main.png";
    pub const SIZE: Size = &[256.0, 256.0];
    pub const DEFAULT_TINT: Tint = &[0.5, 0.5, 0.5, 1.0];
    pub const ID: RenderType = 0;

    pub const PLAYER_STAND: Sprite = &[0.0, 0.0, 32.0, 31.5];
    pub const TEST: Sprite = &[0.0, 0.0, 1.0, 0.5];
    pub const PLAYER_BODY: Sprite = &[32.0, 0.0, 32.0, 31.5];
    pub const PLAYER_ARM: Sprite = &[0.0, 32.0, 32.0, 31.5];

    pub const PLAYER_1_TINT: Tint = &[0.0, 0.0, 1.0, 1.0];
    pub const PLAYER_2_TINT: Tint = &[1.0, 0.0, 0.0, 1.0];
}
