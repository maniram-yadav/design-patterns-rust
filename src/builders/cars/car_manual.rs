use crate::builders::cars::components::{CarType,Engine,Transmission,GpsNavigator};

pub struct Manual {
    car_type :CarType,
    engine : Engine,
    seats : u16,
    transmission:Transmission,
    gps_navigator:Option<GpsNavigator>,
}


impl Manual {
    pub fn new (car_type : CarType,engine :Engine, seats : u16,
    transmission : Transmission, gps_navigator : Option<GpsNavigator> ) -> Self{
        Self {
            car_type,
            seats,
            engine,
            transmission,
            gps_navigator,
        }
    }
}

impl std::fmt::Display for Manual {
     fn fmt(&self,f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        writeln!{f,"Type of car {:?}",self.car_type};
        writeln!{f,"No of seats {:?}",self.seats};
        writeln!{f,"Transmission {:?}",self.transmission};
        writeln!{f,"Engine :: Volume {:?} , Mileage {:?}",self.engine.volume(),
                    self.engine.mileage()};
        match &self.gps_navigator {
            Some(gps) => writeln!(f,"Gps Detail :: {:?}",gps),
            None => writeln!(f,"navigator detail not available"),
        };
        Ok(())
    }
}