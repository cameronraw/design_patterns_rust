pub struct MarsRover {
    current_orientation: char
}

impl MarsRover {
    pub fn new() -> Self {
        MarsRover {
            current_orientation: 'N'
        }
    }

    pub fn execute(&mut self, command_string: &str) -> String {
        command_string.chars().for_each(|cmd| {
            if cmd == 'R' {
                self.current_orientation = self.handle_r_command();
            }
        });

        format!("0:0:{}", self.current_orientation)
    }

    fn handle_r_command(&self) -> char {
        match &self.current_orientation {
            'N' => 'E',
            'E' => 'S',
            'S' => 'W',
            'W' => 'N',
            _ => { panic!("Invalid orientation: {}", &self.current_orientation) }
        }
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