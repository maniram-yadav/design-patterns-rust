use std::collections::HashMap;
use super::PaymentStrategy;

struct PaymentGateway {
    clients : Vec<String>,
    banks : Vec<String>,
    router : Vec<String>,
    payment_strategy : HashMap<String,Box<dyn PaymentStrategy>>

}

impl PaymentGateway {
    fn new() -> Self {
        PaymentGateway { 
            clients : Vec::new(),
            banks : Vec::new(),
        }
    }
}