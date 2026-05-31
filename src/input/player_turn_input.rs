use bevy::{ecs::{query::With, system::{Res, ResMut, Single}}, math::IVec2, state::state::NextState};

use crate::{action::kinds::Action, app::states::TurnState, input::{Dir, keybinds::{GameInput, InputKind}}, things_on_grid::components::{PendingAction, PlayerActor}};


pub fn register_player_input(
    game_inputs: Res<GameInput>,
    mut player_pending_action: Single<&mut PendingAction, With<PlayerActor>>,
    mut turn_state: ResMut<NextState<TurnState>>
) {
    let inputs = game_inputs.get_just_pressed();

    let mut movement = IVec2::new(0, 0);

    for input_kind in inputs {

        if let InputKind::Move(dir) = input_kind {
            match dir {
                Dir::N => movement.y += 1,
                Dir::S => movement.y -= 1,
                Dir::E => movement.x += 1,
                Dir::W => movement.x -= 1,
            }
        }
    }

    if movement != IVec2::new(0, 0) {
        player_pending_action.action = Some(Action::Move(movement));
        turn_state.set(TurnState::PerformingActions);
        return 
    }

}