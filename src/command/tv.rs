

pub struct Tv {
     is_on : bool,
}

impl Tv {
    pub fn new() -> Tv {
        Self {
            is_on:false,
        }
    }

    pub fn on(&mut self) {

        self.is_on = true;
        println!("Tv is On");
    }
    
    pub fn off(&mut self) {

        self.is_on = false;
        println!("Tv is Off");
        
    }
}