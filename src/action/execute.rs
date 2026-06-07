use bevy::{camera::Camera2d, ecs::{entity::Entity, query::{Has, With, Without}, system::{Query, Res, ResMut, Single}}, state::state::NextState, transform::components::Transform};
use crate::{action::kinds::Action, app::states::TurnState, things_on_grid::components::{PendingAction, PlayerActor, Position}, turn::{clock::Clock, scheduler::{ActorPriority, ScheduledActor, Scheduler}}, world::{floor::{CurrentFloor, DiscoveredFloors}, map::tile::{CollisionKind, TileKind}, systems::DirtyMaprenderMarker}};


pub fn execute_actions(
    actors_q: Query<(Entity, &PendingAction)>,
    mut actor_entity_q: Query<(&mut Position, &mut Transform, Has<PlayerActor>), Without<Camera2d>>,
    mut camera_transform_q: Single<&mut Transform, With<Camera2d>>,
    current_floor: Res<CurrentFloor>,
    mut discovered_floors: ResMut<DiscoveredFloors>,
    clock: Res<Clock>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut scheduler: ResMut<Scheduler>,
    mut render_marker: ResMut<DirtyMaprenderMarker>,
) {
    for (entity, pending_action) in actors_q {
        if let Some(action) = &pending_action.action {
            let (mut pos, mut transform, is_player) = actor_entity_q.get_mut(entity).unwrap();
            let priority = if is_player { ActorPriority::Player } else { ActorPriority::Enemy };

            
            // getting the current map
            let (map, spatial_map) = discovered_floors.get_mut(&current_floor).expect("Error: CurrentFloor not present in DiscoveredFloors");

            let _ = match action {
                Action::Move(dir) => {
                    let current_pos = pos.0;
                    let dest_pos = current_pos + dir;
                    let dest_tile_kind = &map.get_tile(&dest_pos).unwrap().kind;
                    let dest_tile_collision = CollisionKind::from(dest_tile_kind);

                    match dest_tile_collision {
                        CollisionKind::None => {
                            spatial_map.move_entity(&current_pos, dest_pos, entity).unwrap();

                            if dest_tile_kind == &TileKind::Door(false) {
                                map.set_tile(&dest_pos, TileKind::Door(true)).unwrap();
                                render_marker.0 = true;
                            }

                            let (delta_x, delta_y) = (
                                (32 * (dir.x)) as f32,
                                (32 * (dir.y)) as f32
                            );

                            transform.translation.x += delta_x;
                            transform.translation.y += delta_y;

                            pos.0 = dest_pos;

                            if is_player {
                                camera_transform_q.translation.x += delta_x;
                                camera_transform_q.translation.y += delta_y;
                            }

                            true
                        },
                        CollisionKind::Solid | CollisionKind::DeepWater | CollisionKind::Digable(_) => false,
                    }
                } 
                _ => panic!("Err: Action not registered")
            };
            // push the actor that acted back onto the scheduler with its next schedule
            scheduler.queue.push(ScheduledActor {
                next_tick: clock.0 + action.to_delay_with_multiplier(pending_action.delay_mult),
                entity,
                priority
            });
        }
        // set turnstate to cycle
        turn_state.set(TurnState::CyclingActors);
        return
    }
}