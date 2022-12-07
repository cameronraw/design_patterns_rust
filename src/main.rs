use design_patterns::mediator::control_tower::ControlTower;
use design_patterns::mediator::plane::Plane;

pub fn main(){
    let control_tower = ControlTower::new();
    let plane = Plane::new(&control_tower);

    println!("Mediator Pattern");
    println!("----------------\n");

    plane.contact_tower("Hello there, tower!");
}