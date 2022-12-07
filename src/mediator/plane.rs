use crate::mediator::control_tower::ControlTower;
use crate::mediator::mediator::Mediator;

pub struct Plane<'a> {
    control_tower: &'a ControlTower
}

impl<'a> Plane<'a> {
    pub fn new(control_tower: &'a ControlTower) -> Self {
        Plane {
            control_tower
        }
    }

    pub fn contact_tower(self, message: &str) {
        self.control_tower.send_message(message)
    }
}