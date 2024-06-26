use std::{thread, time::Duration};

use crate::useractions::{UserAction, UserActionSimulation};

pub fn learn(user_actions: &mut UserAction) {
    user_actions.set_action(UserActionSimulation::RotateLeft);
}
