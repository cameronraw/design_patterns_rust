pub trait Mediator {
    fn send_message(&self, message: &str);
}