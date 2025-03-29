use rand;
use super::bank::Bank;

pub struct Router {
    banks : Vec<Bank>,
}

impl Router {

    fn route_payment(&self) -> &Bank {    

        let mut rng = rand::thread_rng();
        let random_value = rng.gen_range(0.0,100.0);
        let mut  cumulative_percent = 0.0;

        for bank in &self.banks {
            cumulative_percent  += bank.traffic_percent;
            if random_value <= cumulative_percent {
                return bank;
            }
        }
        &self.banks[0]

    }

}