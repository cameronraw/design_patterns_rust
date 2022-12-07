use crate::mediator::mediator::Mediator;

pub struct ControlTower {

}

impl ControlTower {
    pub fn new() -> ControlTower {
        ControlTower{}
    }
}

impl Mediator for ControlTower {
    fn send_message(&self, message: &str){
        println!("Control tower received this message: {}", message)
    }
}