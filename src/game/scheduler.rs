use std::{cmp::Ordering, collections::BinaryHeap};
use bevy::{ecs::{entity::Entity, resource::Resource, system::{Commands, ResMut}}, state::state::{NextState, States}};

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
#[derive(Resource, Default)] 
pub struct Clock(pub u64);

impl Clock {
    pub fn to_decimal_string(&self) -> String {
        let s = self.0.to_string();
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