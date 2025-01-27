use super::Command;

pub struct Button {
    command :  Box<dyn Command>,
}

impl Button {
    pub(crate) fn new(command : Box<dyn Command>) -> Self  {
            Self {
                command,
            }
    }

    pub fn pressed(&mut self) {
        self.command.execute();
    }

}

