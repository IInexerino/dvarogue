use bevy::{ecs::{query::With, system::{Res, ResMut, Single}}, math::IVec2, state::state::NextState};

use crate::{action::kinds::Action, app::states::TurnState, input::{Dir, centralization::{GameInput, InputKind}}, things_on_grid::components::{PendingAction, PlayerActor}};


pub fn register_player_input(
    game_inputs: Res<GameInput>,
    mut player_pending_action: Single<&mut PendingAction, With<PlayerActor>>,
    mut turn_state: ResMut<NextState<TurnState>>
) {
    if let Some(input) = game_inputs.get_just_pressed().next() {
        let action = match input {
            InputKind::ToggleZoom => None,
            InputKind::Move(dir) => Some(Action::Move(dir.into_delta_offset())),
            InputKind::Wait => Some(Action::Wait),
            InputKind::Rotate(dir) => Some(Action::Rotate(*dir)),
            InputKind::PickupItems => Some(Action::PickupItems),
        };
        if action.is_some() {
            turn_state.set(TurnState::PerformingActions);
            player_pending_action.action = action;
        }
    }
}