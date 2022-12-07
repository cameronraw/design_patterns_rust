use design_patterns::patterns::mediator::control_tower::*;
use design_patterns::patterns::mediator::plane::*;
use design_patterns::utils::message_printing::*;

pub fn main(){
    let control_tower = ControlTower::new();
    let plane_1 = Plane::new(&control_tower);
    let plane_2 = Plane::new(&control_tower);
    let plane_3 = Plane::new(&control_tower);

    create_title("Mediator Pattern");

    plane_1.contact_tower("Hello there, tower! This is Plane 1");
    plane_2.contact_tower("Plane 2 here, looking for a landing");
    plane_3.contact_tower("This is plane 3, looking to takeoff");
}
