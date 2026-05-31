use std::{cmp::Ordering, collections::BinaryHeap};
use bevy::{ecs::{entity::{ContainsEntity, Entity}, query::With, resource::Resource, system::{Commands, ResMut, Single}}, state::state::NextState};
use crate::{app::states::TurnState, things_on_grid::components::PlayerActor, turn::clock::Clock};


#[derive(Resource)]
pub struct Scheduler {
    pub queue: BinaryHeap<ScheduledActor>,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum ActorPriority {
    World,
    Enemy,
    Ally,
    Player,
}

#[derive(Eq, PartialEq)]
pub struct ScheduledActor {
    pub entity: Entity,
    pub next_tick: u64,
    /// Higher priority will act before if the next_tick is the same as another actor. Can also be read for actor type.
    pub priority: ActorPriority,
}

impl Ord for ScheduledActor {
    fn cmp(&self, other: &Self) -> Ordering {
        other.next_tick.cmp(&self.next_tick)
            .then_with(|| (self.priority as u8).cmp(&(other.priority as u8)))
    }
}

impl PartialOrd for ScheduledActor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub fn reset_scheduler(mut commands: Commands, player_ent: Single<Entity, With<PlayerActor>>) {
    commands.insert_resource(Scheduler { 
        queue: BinaryHeap::from([ScheduledActor {
            entity: player_ent.entity(),
            next_tick: 0,
            priority: ActorPriority::Player
        }]) 
    });
}

pub fn cycle_actions(
    mut scheduler: ResMut<Scheduler>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut clock: ResMut<Clock>,
) {
    let actor = scheduler.queue.pop()
        .expect("Actor queue should never be empty. There should always be one player on each rerun, and it should terminate when player is selected");

    clock.0 = actor.next_tick;

    // if the actor is a player, go to choose player actions
    if actor.priority == ActorPriority::Player { 
        turn_state.set(TurnState::AwaitingPlayerInput);
        return
    }

    // run the behavooural ai of whatever actor there is


    turn_state.set(TurnState::PerformingActions);
}