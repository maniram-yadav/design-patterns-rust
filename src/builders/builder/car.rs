use crate::builders::cars::{
    car::Car,
    car_manual::Manual,
    components::{CarType,Engine,GpsNavigator,Transmission},
};

use super::Builder ;
pub const DEFAULT_FUEL: f64 = 5f64;

#[derive(Default)]
pub struct CarBuilder{
    car_type : Option<CarType>,
    engine : Option<Engine>,
    navigator : Option<GpsNavigator>,
    transmission: Option<Transmission>,
    seats: Option<u16>,
}

impl Builder for CarBuilder {
    type OutputType = Car;

    fn set_car_type(&mut self,car_type:CarType) {
        self.car_type = Some(car_type);
    }

    fn set_engine(&mut self,engine:Engine) {
        self.engine = Some(engine);
    }

    fn set_transmission(&mut self,transmission:Transmission) {
        self.transmission = Some(transmission);
    }

    fn set_gps_nav(&mut self,gps_navigator:GpsNavigator) {
        self.navigator = Some(gps_navigator);
    }

    fn set_seats(&mut self,seats:u16) {
        self.seats = Some(seats);
    }

    fn build(self) -> Car {
        
        Car::new(
            self.car_type.expect("Please provide car type"),
            self.seats.expect("Please provide no of seats"),
            self.engine.expect("Please provide Engine"),
            self.transmission.expect("Please provide Transmission"),
            self.navigator,
            DEFAULT_FUEL
        )
    }


}