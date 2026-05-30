use std::{cmp::Ordering, collections::BinaryHeap};
use bevy::{ecs::{entity::{ContainsEntity, Entity}, query::With, resource::Resource, system::{Commands, Res, ResMut, Single}}, input::{ButtonInput, keyboard::KeyCode}, prelude::{Deref, DerefMut}, state::state::NextState};

use crate::{TurnState, game::actors::{PlayerActor, actions::Action}};

/// Creates a new default [`Clock`], [`Scheduler`] resource, or resets an existing one to default.
pub fn reset_clock(mut commands: Commands) {
    commands.insert_resource(Clock::default());
}

pub fn reset_scheduler(mut commands: Commands, player_ent: Single<Entity, With<PlayerActor>>) {
    commands.insert_resource(Scheduler { 
        queue: BinaryHeap::from([ScheduledActor {
            entity: player_ent.entity(),
            next_tick: 0,
            priority: 3
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
    if actor.priority == 3 { 
        turn_state.set(TurnState::AwaitingPlayerInput);
        return
    }

    // run the behavooural ai of whatever actor there is

    turn_state.set(TurnState::PerformingActions);
}


/// Total turn counter that runs from the start to the end of a game.
/// 
/// Each 'average' turn is 100, this is kept as an int, 
/// but is usually displayed to the player as a string with one or two decimal points.
/// 
/// It does not advance by ticking `+= 1`, but instead advances to the scheduled action time 
/// of the soonest acting `Actor` entities, and is updated so.
#[derive(Resource, Default, Deref, DerefMut)] 
pub struct Clock(pub u64);

impl Clock {
    pub fn to_decimal_string(&self) -> String {
        let s = self.to_string();
        // Possible Bug? ; Could this possibly return more or less 
        let len = s.len();

        if len == 1 {
            return String::from("0.0")
        } else if len == 2 {
            return format!("0.{}", &s[len-2..len-1]) 
        } else {
            return format!("{}.{}", &s[..len-2], &s[len-2..len-1])
        }
    }
}


/// Component for Entities with [`Actor`].
/// 
/// Keeps track of the amount of `Clock` units that it takes to make a single action for the [`Actor`], 
/// as well as the `Clock` time at which its next action is to be taken.
/// 
/// `self.next_action_tick` must be updated every time its the entity's turn, and it has completed any action.


#[derive(Resource)]
pub struct Scheduler {
    pub queue: BinaryHeap<ScheduledActor>,
}

#[derive(Eq, PartialEq)]
pub struct ScheduledActor {
    pub entity: Entity,
    pub next_tick: u64,
    /// Higher priority will act before if the next_tick is the same as another actor. Can also be read for actor type.
    ///
    /// - world = 0
    /// - enemies = 1
    /// - allies = 2
    /// - player = 3
    pub priority: u8,
}

impl Ord for ScheduledActor {
    fn cmp(&self, other: &Self) -> Ordering {
        other.next_tick.cmp(&self.next_tick)
            .then_with(|| self.priority.cmp(&other.priority))
    }
}

impl PartialOrd for ScheduledActor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}