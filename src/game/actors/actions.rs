use bevy::{camera::Camera2d, ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query, Res, ResMut, Single}}, math::IVec2, state::state::NextState, transform::components::Transform};

use crate::{TurnState, game::{actors::{Actor, PlayerActor, PlayerSelectedAction, Position, SelectedAction}, map::{CollisionKind, CurrentFloor, DiscoveredFloors, TileKind}, scheduler::{Clock, ScheduledActor, Scheduler}}};

#[derive(Clone)]
pub enum Action {
    Move(IVec2),
    /// With attacked Entity id
    Attack (Entity),
    Wait,
    /// With door Entity id
    OpenDoor (IVec2),
    Pickup (Entity)
}

impl Action {
    pub fn to_delay(&self) -> u64 {
        match self {
            Action::Move(_) => 100,
            Action::Attack(_) => 100,
            Action::Wait => 100,
            Action::OpenDoor(_) => 50,
            Action::Pickup(_) => 150,
        }
    }

    pub fn to_delay_with_multiplier(&self, mult: u64) -> u64 {
        (self.to_delay() * mult) / 100
    }
}

pub fn execute_actions(
    enemy_action_q: Query<(Entity, &SelectedAction)>,
    player_entity_q: Single<Entity, With<PlayerActor>>,
    actor_q: Query<&Actor>,
    mut position_q: Query<&mut Position>,
    mut transform_q: Query<&mut Transform, Without<Camera2d>>,
    mut camera_transform_q: Single<&mut Transform, With<Camera2d>>,
    current_floor: Res<CurrentFloor>,
    mut discovered_floors: ResMut<DiscoveredFloors>,
    player_action: Res<PlayerSelectedAction>,
    clock: Res<Clock>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut scheduler: ResMut<Scheduler>,
) {
    let mut enemy_action = None;
    for (enemy_entity, selected_action) in enemy_action_q {
        if let Some(s) = &selected_action.0 {
            enemy_action = Some((enemy_entity, s.clone()))
        }
    }

    // figuring out which entity, which action, which priority
    let (entity, action, priority) = if let Some(action) = &player_action.0 {
        let p_entity = player_entity_q.into_inner();
        (p_entity, action.clone(), 3_u8)
    } else if let Some((ent, act)) = enemy_action { 
        (ent, act, 1)
    } else { return };

    // getting the current map
    let (map, spatial_map) = discovered_floors.get_mut(&current_floor).expect("Error: CurrentFloor not present in DiscoveredFloors");

    let pass = match action {
        Action::Move(dir) => {
            let current_pos = position_q.get(entity).expect("Error: Actor Entity does not have Position Component").0;
            let dest_pos = current_pos + dir;
            let dest_tile_kind = &map.get_tile(&dest_pos).unwrap().kind;
            let dest_tile_collision = CollisionKind::from(dest_tile_kind);

            match dest_tile_collision {
                CollisionKind::None => {
                    spatial_map.entities.get_mut(&current_pos).expect("Error: Position coords not present in SpatialMap").retain(
                        |&a | a != entity
                    );
                    spatial_map.entities.get_mut(&dest_pos).expect("Error: Position coords not present in SpatialMap").push(entity);  

                    if dest_tile_kind == &TileKind::Door(false) {
                        map.set_tile(&dest_pos, TileKind::Door(true)).unwrap();
                    }

                    let (delta_x, delta_y) = (
                        (32 * (dir.x)) as f32,
                        (32 * (dir.y)) as f32
                    );

                    let mut transform = transform_q.get_mut(entity).unwrap();
                    transform.translation.x += delta_x;
                    transform.translation.y += delta_y;

                    position_q.get_mut(entity).unwrap().0 = dest_pos;

                    if priority == 3 {
                        let mut camera_transform = camera_transform_q;
                            camera_transform.translation.x += delta_x;
                            camera_transform.translation.y += delta_y;

                    }

                    true
                },
                CollisionKind::Solid | CollisionKind::DeepWater | CollisionKind::Digable(_) => false,
            }

        } 
        _ => panic!("Err: Action not registered")
    };

    let delay_mult = actor_q.get(entity).expect("Error: Entity has no Actor Component").delay_mult;
    // push the actor that acted back onto the scheduler with its next schedule
    scheduler.queue.push(ScheduledActor {
        next_tick: clock.0 + action.to_delay_with_multiplier(delay_mult),
        entity,
        priority
    });

    // set turnstate to cycle
    turn_state.set(TurnState::CyclingActors);
}