use crate::builders::cars::components::{CarType,Engine,Transmission,GpsNavigator};

pub struct Car {
    car_type:CarType,
    seats : u16,
    engine:Engine,
    transmission: Transmission,
    gps_navigator: Option<GpsNavigator>,
    fuel :f64,
}

impl Car {
   pub  fn new(
        car_type:CarType,
        seats : u16,
        engine:Engine,
        transmission: Transmission,
        gps_navigator: Option<GpsNavigator>,
        fuel :f64,) -> Self {
            Self {
                car_type,
                seats,
                engine,
                transmission,
                gps_navigator,
                fuel,
            }
    }

    
    pub fn seats(&self) -> u16 {
        self.seats
    }
    
    pub fn engine(&self) -> &Engine {
        &self.engine
    }
    pub fn car_type(&self) -> &CarType {
        &self.car_type
    }

    pub fn fuel(&self) -> f64{
        self.fuel
    }

    pub fn set_fuel(&mut self,fuel : f64) -> f64 {
        self.fuel = fuel;
        self.fuel
    }

    pub fn transmission(&self) -> &Transmission{
        &self.transmission
    }

    pub fn gps_navigator(&self) -> &Option<GpsNavigator>{
        &self.gps_navigator
    }

    

}


impl std::fmt::Display for Car {
    fn fmt(&self,f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       
       writeln!{f,"Type of car {:?}",self.car_type};
       writeln!{f,"No of seats {:?}",self.seats};
       writeln!{f,"Fuel {}",self.fuel};
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