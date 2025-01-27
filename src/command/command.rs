
use super::Command;
use crate::command::tv::Tv;

pub struct OnCommand <'a>{
    pub tv :&'a mut Tv,
}

impl<'a> OnCommand<'a> {
    pub fn new(tv : &'a mut Tv) -> Self {
            Self {
                tv,
            }
    }
}


impl<'a> Command for OnCommand<'a> {

      fn execute(&mut self){
            self.tv.on();
    }
}


pub struct OffCommand<'a>{
    pub  tv : &'a mut  Tv,
}

impl<'a> OffCommand<'a> {
    pub fn new(tv : &'a mut Tv) -> Self {
            Self {
                tv,
            }
    }
}


impl<'a> Command for OffCommand<'a> {

    fn execute(&mut self){
            self.tv.off();
    }
}