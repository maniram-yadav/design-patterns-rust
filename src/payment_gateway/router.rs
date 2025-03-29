use rand;
use super::bank::Bank;
use rand::Rng;

pub struct Router {
   pub banks : Vec<Bank>,
}

impl Router {

    fn route_payment(&self) -> &Bank {    

        let mut rng = rand::rng();
        let random_value = rng.random_range(0..100);
        let mut  cumulative_percent = 0.0;

        for bank in &self.banks {
            cumulative_percent  += bank.traffic_percent;
            if f64::from(random_value) <= cumulative_percent {
                return bank;
            }
        }
        &self.banks[0]

    }

}