use bevy::{asset::Handle, color::{Color, palettes::css::{BLACK, PINK}}, ecs::component::Component, image::{Image, TextureAtlas, TextureAtlasLayout}, math::IVec2, sprite::Sprite};

#[derive(Clone, Copy, PartialEq)]
pub enum TileKind {
    Floor,
    WallBedrock,
    WallAlloy,
    WallRock,
    WallWood,
    WallDirt,
    ShallowWater,
    DeepWater,
    StairsDown,
    Door(bool),
}

impl TileKind {
    pub fn to_sprite(&self, image: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Option<Sprite> {
        match self {
            TileKind::Floor => Some(Sprite::from_atlas_image(image, TextureAtlas { layout, index: 1 })),
            TileKind::WallBedrock => {
                let mut sprite = Sprite::from_atlas_image(image, TextureAtlas { layout, index: 0 }); 
                sprite.color = Color::Srgba(BLACK);
                Some(sprite)
            } 
            TileKind::WallRock => Some(Sprite::from_atlas_image(image, TextureAtlas { layout, index: 0 })),
            TileKind::Door(false) => {
                let mut sprite = Sprite::from_atlas_image(image, TextureAtlas { layout, index: 0 });
                sprite.color = Color::Srgba(PINK);
                Some(sprite)
            },
            TileKind::Door(true) => Some(Sprite::from_atlas_image(image, TextureAtlas { layout, index: 1 })),

            _ => None
        }
    }
}

pub enum CollisionKind {
    None,
    Solid,
    Digable(u8),
    DeepWater,
}

impl From<&TileKind> for CollisionKind {
    fn from(value: &TileKind) -> Self {
        match value {
            TileKind::WallBedrock => Self::Solid,
            TileKind::WallDirt => Self::Digable(1),
            TileKind::WallWood => Self::Digable(2),
            TileKind::WallRock => Self::Digable(4),
            TileKind::WallAlloy => Self::Digable(8),
            TileKind::DeepWater => Self::DeepWater,
            _ => Self::None
        }
    }
}

#[derive(Clone)]
pub struct Tile {
    pub kind: TileKind,
    pub visible: bool,
    pub discovered: bool,
}

impl From<TileKind> for Tile {
    fn from(value: TileKind) -> Self {
        Tile {kind: value, visible: false, discovered: false }
    }
}

#[derive(Component)]
pub struct TilePos(pub IVec2);