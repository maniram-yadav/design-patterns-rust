
mod car;
mod car_manual;

use crate::builders::cars::components::{CarType,Engine,Transmission,GpsNavigator};

pub trait Builder{
    type OutputType;
    fn set_car_type(&mut self,car_type:CarType);
    fn set_engine(&mut self,engine:Engine);
    fn set_transmission(&mut self,transmission:Transmission);
    fn set_gps_nav(&mut self,gps_navigator:GpsNavigator);
    fn set_seats(&mut self,seats:u16);
    fn build(self) -> Self::OutputType;
}

pub use self::car::CarBuilder;
pub use self::car_manual::CarManualBuilder;