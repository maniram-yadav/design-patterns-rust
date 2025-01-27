pub mod command;
pub mod tv;
pub mod button;


pub trait Command {
    fn execute(&mut self);
}
