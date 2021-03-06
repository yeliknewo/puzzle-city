use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct RenderData {
    default_tint: [f32; 4],
    tint: [f32; 4],
    layer: u8,
    spritesheet_rect: &'static [f32; 4],
    spritesheet_size: &'static [f32; 2],
    dirty: bool,
}

impl RenderData {
    pub fn new(layer: u8, tint: [f32; 4], spritesheet_rect: &'static [f32; 4], spritesheet_size: &'static [f32; 2]) -> RenderData {
        RenderData {
            default_tint: tint.clone(),
            tint: tint,
            layer: layer,
            spritesheet_rect: spritesheet_rect,
            spritesheet_size: spritesheet_size,
            dirty: true,
        }
    }

    pub fn set_layer(&mut self, layer: u8) {
        self.layer = layer;
        self.set_dirty();
    }

    pub fn set_spritesheet_rect(&mut self, spritesheet_rect: &'static [f32; 4]) {
        self.spritesheet_rect = spritesheet_rect;
        self.set_dirty();
    }

    pub fn set_tint(&mut self, tint: [f32; 4]) {
        self.tint = tint;
        self.set_dirty();
    }

    pub fn reset_tint(&mut self) {
        self.tint = self.default_tint;
        self.set_dirty();
    }

    pub fn get_default_tint(&self) -> [f32; 4] {
        self.default_tint.clone()
    }

    pub fn get_layer(&self) -> u8 {
        self.layer
    }

    pub fn get_tint(&self) -> [f32; 4] {
        self.tint.clone()
    }

    pub fn get_spritesheet_rect(&self) -> [f32; 4] {
        self.spritesheet_rect.clone()
    }

    pub fn get_spritesheet_size(&self) -> [f32; 2] {
        self.spritesheet_size.clone()
    }

    fn set_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn take_dirty(&mut self) -> bool {
        let temp = self.dirty;
        self.dirty = false;
        temp
    }
}

impl Component for RenderData {
    type Storage = VecStorage<RenderData>;
}
