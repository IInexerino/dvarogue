use std::fmt::Display;
use bevy::math::IVec2;
use rand::{random_range, seq::IndexedRandom};
use crate::game::map::{Map, TileKind};

const MIN_ROOM_WIDTH:i32 = 3;
const MAX_ROOM_WIDTH:i32 = 12;
const MIN_ROOM_HEIGHT:i32 = 3;
const MAX_ROOM_HEIGHT:i32 = 12;

const PERCENT_CHANCE_TO_CHANGE_DIRECTION: u8 = 5;

#[derive(Debug, Clone)]
pub struct Room {
    pub bottom_left: IVec2,
    pub width: i32,
    pub height: i32,
}

impl Room {
    pub fn left(&self) -> i32 {
        self.bottom_left.x
    }

    pub fn right(&self) -> i32 {
        self.bottom_left.x + self.width - 1
    }

    pub fn bottom(&self) -> i32 {
        self.bottom_left.y
    }

    pub fn top(&self) -> i32 {
        self.bottom_left.y + self.height - 1
    }
    pub fn intersects(
        &self,
        other: &Self,
    ) -> bool {
        !(self.right() < other.left()
        || self.left() > other.right()
        || self.top() < other.bottom()
        || self.bottom() > other.top())
    }
    pub fn expanded(
        &self,
        amount: i32,
    ) -> Self {
        Self {
            bottom_left:
                self.bottom_left
                - IVec2::splat(amount),

            width:
                self.width
                + amount * 2,

            height:
                self.height
                + amount * 2,
        }
    }
    pub fn gap_to(
        &self,
        other: &Room,
    ) -> (i32, i32) {

        let horizontal_gap =
            if self.right() < other.left() 
                { other.left() - self.right() - 1 }
            else if other.right() < self.left()
                { self.left() - other.right() - 1}
            else 
                { 0 };

        let vertical_gap =
            if self.top() < other.bottom()
                { other.bottom() - self.top() - 1 }
            else if other.top() < self.bottom()
                { self.bottom() - other.top() - 1 }
            else { 0 };

        (
            horizontal_gap,
            vertical_gap
        )
    }
}

impl Map {
    pub fn carve_room(&mut self, room: &Room) {
        let mut tiles_to_carve = Vec::new();
        for y in room.bottom()..=room.top(){
            for x in room.left()..=room.right() {
                tiles_to_carve.push(IVec2::new(x, y));
            }
        }
        for tile in &tiles_to_carve {
            self.set_tile(tile, TileKind::Floor).unwrap();
        }
        self.regions.push(tiles_to_carve);
    }

    pub fn get_side_neighbors(&self, coords: &IVec2) -> Vec<IVec2> {
        let mut neighbors = Vec::new();

        if coords.x - 1 > 0 {
            neighbors.push(IVec2::new(coords.x - 1, coords.y));
        }
        if coords.y - 1 > 0 {
            neighbors.push(IVec2::new(coords.x, coords.y - 1));
        }
        if coords.x + 1 < self.size.width {
            neighbors.push(IVec2::new(coords.x + 1, coords.y));
        }
        if coords.y + 1 < self.size.height {
            neighbors.push(IVec2::new(coords.x, coords.y + 1));
        }

        neighbors
    }

    pub fn get_side_neighbors_with_dir(&self, coords: &IVec2) -> Vec<(IVec2, Dir)> {
        let mut neighbors = Vec::new();

        if coords.x - 1 > 0 {
            neighbors.push((IVec2::new(coords.x - 1, coords.y), Dir::W));
        }
        if coords.y - 1 > 0 {
            neighbors.push((IVec2::new(coords.x, coords.y - 1), Dir::S));
        }
        if coords.x + 1 < self.size.width {
            neighbors.push((IVec2::new(coords.x + 1, coords.y), Dir::E));
        }
        if coords.y + 1 < self.size.height {
            neighbors.push((IVec2::new(coords.x, coords.y + 1), Dir::N));
        }

        neighbors
    }

    pub fn get_diagonal_neighbors(&self, coords: &IVec2) -> Vec<IVec2> {
        let mut neighbors = Vec::new();

        if coords.x - 1 > 0 
        && coords.y - 1 > 0 {
            neighbors.push(IVec2::new(coords.x - 1, coords.y - 1));
        }
        if coords.x - 1 > 0
        && coords.y + 1 < self.size.height {
            neighbors.push(IVec2::new(coords.x - 1, coords.y + 1));
        }
        if coords.x + 1 < self.size.width
        && coords.y - 1 > 0 {
            neighbors.push(IVec2::new(coords.x + 1, coords.y - 1));
        }
        if coords.y + 1 < self.size.height
        && coords.x + 1 < self.size.width {
            neighbors.push(IVec2::new(coords.x + 1, coords.y + 1));
        }

        neighbors
    }

    pub fn get_all_neighbors(&self, coords: &IVec2) -> Vec<IVec2> {
        let mut side = self.get_side_neighbors(coords);
        let mut diagonal = self.get_diagonal_neighbors(coords);
        side.append(&mut diagonal);

        side 
    }

    // 2nd step
    pub fn set_boundary_border(&mut self) {
        for y in 0..self.size.height {
            for x in 0..self.size.width {
                if x == 0 || x == self.size.width - 1 || y == 0 || y == self.size.height - 1 {
                    self.set_tile(&IVec2::new(x, y), TileKind::WallBedrock).unwrap();
                }
            }
        }
    }

    // 3rd step
    pub fn place_random_rooms(&mut self, attempts: u32) {
        let mut rooms = Vec::new();
        let mut rng = rand::rng();
        for _ in 0..attempts {

            let r_w: Vec<i32> = (MIN_ROOM_WIDTH..=MAX_ROOM_WIDTH).collect();
            let r_h: Vec<i32> = (MIN_ROOM_HEIGHT..=MAX_ROOM_HEIGHT).collect();
            let (room_width, room_height) = (
                *r_w.choose(&mut rng).unwrap(),
                *r_h.choose(&mut rng).unwrap()
            );

            let b_l_x: Vec<i32> = (2..(self.size.width - 2 - (room_width - 1))).collect();
            let b_l_y: Vec<i32> = (2..(self.size.height - 2 - (room_height - 1))).collect();
            let bottom_left_position = IVec2::new(
                *b_l_x.choose(&mut rng).unwrap(),
                *b_l_y.choose(&mut rng).unwrap()
            );

            let candidate = Room {width: room_width, height: room_height, bottom_left: bottom_left_position};

            let invalid = rooms.iter().any(| room| candidate.expanded(1).intersects(&room) );

            if invalid {
                continue
            } else {
                self.carve_room(&candidate);
                rooms.push(candidate);
            }

        }
    }

    fn draw_maze(&mut self, start_coords: IVec2) {
        let mut rng = rand::rng();
        
        self.set_tile(&start_coords, TileKind::Floor).unwrap();
        let mut maze_region = vec![start_coords];

        let mut maze_stack = vec![(start_coords, None)];

        while !maze_stack.is_empty() {
            let current_cell = maze_stack.pop().unwrap();

            let side_neighbors = self.get_side_neighbors_with_dir(&current_cell.0);
            let valid_side_neighbors: Vec<(IVec2, Dir)> = side_neighbors.into_iter().filter(| candidate| {
                if self.get_tile(&candidate.0).unwrap().kind != TileKind::WallRock {
                    return false;
                }
                let side_neighbors_clear = self.get_side_neighbors(&candidate.0)
                    .into_iter()
                    .filter(|sn_coords| self.get_tile(sn_coords).unwrap().kind == TileKind::WallRock)
                    .collect::<Vec<IVec2>>().len() == 3;

                let diagonal_clear = self.get_diagonal_neighbors(&candidate.0)
                    .into_iter()
                    .filter(|dn_coords| !self.get_side_neighbors(&current_cell.0).contains(dn_coords))
                    .all(|dn_coords| self.get_tile(&dn_coords).unwrap().kind == TileKind::WallRock); 

                candidate.0.x >= 2 && candidate.0.y >= 2
                && candidate.0.x < self.size.width - 2 && candidate.0.y < self.size.height - 2
                && side_neighbors_clear
                && diagonal_clear
            })
            .collect(); 

            if !valid_side_neighbors.is_empty() { 
                maze_stack.push(current_cell);

                let picked_neighbor = 
                if let Some(dir) = current_cell.1 && random_range(0..100) > PERCENT_CHANCE_TO_CHANGE_DIRECTION {
                    valid_side_neighbors.iter().find(|&s| s.1 == dir)
                } else {
                    valid_side_neighbors.choose(&mut rng)
                };

                if let Some(picked_neighbor) = picked_neighbor {
                    self.set_tile(&picked_neighbor.0, TileKind::Floor).unwrap();
                    maze_stack.push((picked_neighbor.0, Some(picked_neighbor.1)));
                    maze_region.push(picked_neighbor.0);
                }
            }
        }
        self.regions.push(maze_region); 
    }

    // 4th step
    // checking only for sides when iterating through maze children yields an interesting maze where there are many diagonal connections
    // very dcss like
    // would be interesting to try to set the starting position of the maze to not check its diagonal neighbors
    // would also be interesting to do the opposite and check for the horizontals of the maze children
    pub fn draw_perfect_mazes(&mut self) {
        // begin with true to enter the loop
        let mut some_valid = true;
        while some_valid == true {
            // we dont know if any tiles are valid (not rooms) before we start checking
            some_valid = false;
            for y in 2..self.size.height - 2 {
                for x in 2..self.size.width - 2 {
                    let start_coords = IVec2::new(x, y);
                    if self.get_tile(&start_coords).unwrap().kind == TileKind::Floor { 
                        continue
                    }
                    let neighbors = self.get_all_neighbors(&start_coords);
                    if neighbors.iter().all(|n_coords| self.get_tile(n_coords).unwrap().kind == TileKind::WallRock ) {
                        some_valid = true;
                        self.draw_maze(start_coords);
                    }
                }
            }
        }
    }

    fn unify_regions(&mut self) {
        
    }


    pub fn remove_dead_ends(&mut self, dead_ends_to_remove: u32) {
        let mut removed = 0;
        let mut iterated = 0;
        'outer: loop {
            for y in 0..self.size.height {
                for x in 0..self.size.width {
                    let coords = IVec2::new(x, y);
                    if self.get_tile(&coords).unwrap().kind == TileKind::Floor {
                        let rock_neighbors: Vec<IVec2> = 
                            self.get_side_neighbors(&coords).into_iter()
                                .filter(|s| self.get_tile(s).unwrap().kind == TileKind::WallRock ).collect();
                        if rock_neighbors.len() == 3 {
                            self.set_tile(&coords, TileKind::WallRock).unwrap();
                            removed += 1;
                            iterated = 0;
                        }
                        if removed == dead_ends_to_remove || iterated == 10 { 
                            break 'outer 
                        }
                    }
                }
            }
            
            iterated += 1;
        }
    }

}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new(); 
        for y in (0..self.size.height).rev() {
            for x in 0..self.size.width {
                let tile = self.get_tile(&IVec2::new(x, y)).unwrap();
                match tile.kind {
                    TileKind::WallBedrock => str.push('B'),
                    TileKind::WallRock => str.push('#'),
                    TileKind::Floor => str.push('.'),
                    TileKind::Door(true) => str.push('x'),
                    TileKind::Door(false) => str.push('X'),
                    _ => panic!()
                }
            }
            str.push('\n');
        }

        write!(f, "{str}")
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Dir {
    N,
    S,
    E,
    W
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn creteg_and_print_map() {
        let map = Map::new_from_configs(
            crate::game::map::NextFloorConfigs::from_floor(
                crate::game::map::DungeonFloor::Dungeon(1)
            ).unwrap()
        );
        print!("{map}");
    }
}