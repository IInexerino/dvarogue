use bevy::{asset::AssetServer, color::{Color, palettes::css::{BLACK, PINK}}, ecs::{component::Component, system::Res}, math::IVec2, sprite::Sprite};

#[derive(Clone, PartialEq)]
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
    pub fn to_sprite(&self, asset_server: &Res<AssetServer>) -> Option<Sprite> {
        match self {
            TileKind::Floor => Some(Sprite::from_image(asset_server.load("purple_floor.png"))),
            TileKind::WallBedrock => {
                let mut sprite = Sprite::from_image(asset_server.load("wallrock.png")); 
                sprite.color = Color::Srgba(BLACK);
                Some(sprite)
            } 
            TileKind::WallRock => Some(Sprite::from_image(asset_server.load("wallrock.png"))),
            TileKind::Door(false) => {
                let mut sprite = Sprite::from_image(asset_server.load("wallrock.png"));
                sprite.color = Color::Srgba(PINK);
                Some(sprite)
            },
            TileKind::Door(true) => Some(Sprite::from_image(asset_server.load("purple_floor.png"))),

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