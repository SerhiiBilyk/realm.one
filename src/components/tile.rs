use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};

pub struct TileComponent {
    pub collidable: bool,
}

impl TileComponent {
    pub fn new(collidable: bool) -> TileComponent {
        TileComponent {
           collidable, 
        }
    }
}

impl Component for TileComponent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

