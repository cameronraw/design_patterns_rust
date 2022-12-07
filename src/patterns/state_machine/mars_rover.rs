use crate::patterns::state_machine::orientation::Orientation;
use crate::patterns::state_machine::orientation::Orientation::{EAST, NORTH, SOUTH, WEST};

pub struct MarsRover {
    current_orientation: Orientation
}

impl MarsRover {
    pub fn new() -> Self {
        MarsRover {
            current_orientation: NORTH
        }
    }

    pub fn execute(&mut self, command_string: &str) -> String {
        command_string.chars().for_each(|cmd| {
            if cmd == 'R' {
                self.current_orientation = self.current_orientation.to_right();
            }
        });

        self.get_current_state()
    }

    fn get_current_state(&self) -> String {
        format!("0:0:{}", self.current_orientation.get_char())
    }

}

#[allow(non_snake_case)]
#[cfg(test)]
mod mars_rover_should {
    use super::*;

    #[test]
    pub fn return_0_0_N_when_sent_no_commands(){
        let mut rover = MarsRover::new();
        let response = rover.execute("");
        assert_eq!(response, "0:0:N");
    }

    #[test]
    pub fn return_0_0_E_when_sent_R(){
        let mut rover = MarsRover::new();
        let response = rover.execute("R");
        assert_eq!(response, "0:0:E");
    }

    #[test]
    pub fn return_0_0_S_when_sent_RR(){
        let mut rover = MarsRover::new();
        let response = rover.execute("RR");
        assert_eq!(response, "0:0:S");
    }

    #[test]
    pub fn return_0_0_W_when_sent_RRR(){
        let mut rover = MarsRover::new();
        let response = rover.execute("RRR");
        assert_eq!(response, "0:0:W");
    }
}