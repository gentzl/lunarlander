use macroquad::input::{is_key_down, KeyCode};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(PartialEq, Debug)]
pub enum UserActionSimulation {
    RotateLeft,
    RotateRight,
    TrustActive,
    Restart,
    None,
}
impl Distribution<UserActionSimulation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UserActionSimulation {
        match rng.gen_range(0..=3) {
            // rand 0.8
            0 => UserActionSimulation::RotateLeft,
            1 => UserActionSimulation::RotateRight,
            2 => UserActionSimulation::TrustActive,
            _ => UserActionSimulation::None,
        }
    }
}
pub struct UserAction {
    pub action: UserActionSimulation,
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
