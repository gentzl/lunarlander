use macroquad::input::{is_key_down, KeyCode};

#[derive(PartialEq)]
pub enum UserActionSimulation {
    RotateLeft,
    RotateRight,
    TrustActive,
    Restart,
    None,
}

pub struct UserAction {
    action: UserActionSimulation,
}
impl UserAction {
    pub fn new() -> Self {
        UserAction {
            action: UserActionSimulation::None,
        }
    }

    pub fn set_action(&mut self, action: UserActionSimulation) {
        self.action = action;
    }

    pub fn rotate_left(&self) -> bool {
        if self.action == UserActionSimulation::RotateLeft {
            return true;
        }
        return is_key_down(KeyCode::Left);
    }

    pub fn rotate_right(&self) -> bool {
        if self.action == UserActionSimulation::RotateRight {
            return true;
        }
        return is_key_down(KeyCode::Right);
    }

    pub fn trust_active(&self) -> bool {
        if self.action == UserActionSimulation::TrustActive {
            return true;
        }
        return is_key_down(KeyCode::Up);
    }

    pub fn restart(&self) -> bool {
        if self.action == UserActionSimulation::Restart {
            return true;
        }
        return is_key_down(KeyCode::Enter);
    }
}
