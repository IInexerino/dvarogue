use std::{cmp::Ordering, collections::BinaryHeap};
use bevy::{ecs::{component::Component, entity::Entity, resource::Resource, system::{Commands, ResMut}}, prelude::{Deref, DerefMut}, state::state::{NextState, States}};

use crate::game::actors::actions::Action;

/// Creates a new default [`Clock`] resource, or resets an existing one to default.
pub fn new_reset_clock(mut commands: Commands) {
    commands.insert_resource(Clock::default());
}

/// State which defines whether it is a player's turn and an action selection should be waited for, 
/// or if the player has finished performing the action and other things should run.
#[derive(States, Default, Hash, Debug, Clone, Eq, PartialEq)]
pub enum TurnState {
    #[default]
    NotInGame,
    AwaitingPlayerInput,
    RunningSimulation,
}

/// System to set the [`TurnState`] to [`TurnState::AwaitingPlayerInput`]
pub fn set_turn_state_awaitingplayerinput(mut next_state: ResMut<NextState<TurnState>>) {
    next_state.set(TurnState::AwaitingPlayerInput);
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
#[derive(Component)]
pub struct Timing {
    pub next_action_tick: u64,
    /// Percentage (100 = 100%)
    pub delay_multiplier: u64,
}

impl Timing {

    pub fn action_delay_with_multiplier(&self, action: &Action) -> u64 {
        (action.to_delay() * self.delay_multiplier) / 100
    }

    /// Displays `self.delay` as divided by 100, with two decimal points, without using floats.
    /// 
    /// This is speficially to format it for presentation in the top-right-ui.
    /// 
    /// Examples: 0 = "0.00", 175 = "1.75"
    pub fn to_decimal_string(&self) -> String {
        let delay = self.action_delay_with_multiplier(&Action::Wait);

        let s = delay.to_string();
        // Possible Bug? ; Could this possibly return more or less 
        let len = s.len();

        return if len == 1 {
            format!("0.0{}", s)
        } else if len == 2 {
            format!("0.{}", s) 
        } else {
            format!("{}.{}", &s[..len-2], &s[len-2..])
        }
    }

    /// Reschedules the next action time in `self.next_action_tick`. 
    /// 
    /// Needs to be used any time its an [`Actor`]'s action is completed
    pub fn action_done(&mut self, action: &Action) {
        let delay = self.action_delay_with_multiplier(action);
        self.next_action_tick += delay
    }
}


#[derive(Resource)]
pub struct Scheduler {
    pub queue: BinaryHeap<ScheduledActor>,
}

#[derive(Eq, PartialEq)]
pub struct ScheduledActor {
    pub entity: Entity,
    pub next_tick: u64,
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