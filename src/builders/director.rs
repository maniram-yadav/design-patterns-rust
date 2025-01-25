
use crate::builders::{
    builder::Builder,
    cars::{ 
        components::{CarType,Engine,GpsNavigator,Transmission},
    }
};

pub struct Director;

impl Director {

    pub fn construct_sports_car(builder : &mut impl Builder){
            builder.set_car_type(CarType::Suv);
            builder.set_engine(Engine::new(12.0,89.0));
            builder.set_gps_nav(GpsNavigator::new());
            builder.set_transmission(Transmission::SemiAutomatic);
            builder.set_seats(2);
            
    }

    pub fn construct_city_car(builder : &mut impl Builder){
        builder.set_car_type(CarType::CityCar);
        builder.set_engine(Engine::new(1.0,2.0));
        builder.set_gps_nav(GpsNavigator::new());
        builder.set_transmission(Transmission::Manual);
        builder.set_seats(6);
        
        }

        pub fn construct_suv(builder : &mut impl Builder){
            builder.set_car_type(CarType::Suv);
            builder.set_engine(Engine::new(2.0,9.0));
            builder.set_gps_nav(GpsNavigator::new());
            builder.set_transmission(Transmission::Automatic);
            builder.set_seats(9);
            
    }


}