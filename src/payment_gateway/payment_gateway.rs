use std::collections::HashMap;
use super::payment_strategy::PaymentStrategy;
use super::netbanking_strategy::NetBankingStrategy;
use super::router::Router;
use super::bank::Bank;

pub struct PaymentGateway {
    clients : Vec<String>,
    banks : Vec<String>,
    router : Vec<String>,
    payment_strategy : HashMap<String,Box<dyn PaymentStrategy>>

}

impl PaymentGateway {

    pub  fn new() -> Self {
        
        let mut payment_strategies : HashMap<String,Box<dyn PaymentStrategy>> = HashMap::new();
        payment_strategies.insert("UPI".to_string(),Box::new(NetBankingStrategy));

        PaymentGateway { 
            clients : Vec::new(),
            banks : Vec::new(),
            router : Router{banks: Vec::<Bank>::new()},
            payment_strategies,
        }
    }
}