use std::collections::HashMap;
use super::payment_strategy::PaymentStrategy;
use rand::Rng;
use rand;
use rng;


pub struct NetBankingStrategy;

impl PaymentStrategy for NetBankingStrategy {

    fn pay(&self,amount:f64,payment_details:&HashMap<String,String>) -> Result<bool,String>{

        let username = payment_details.get("username").ok_or("Username is required for netbanking")?;
        let password = payment_details.get("password").ok_or("Password is required for netbanking")?;
        if username.is_empty() || password.is_empty() {
            return Err("Username or pasword cannot be empty".to_string()) ;
        }
        Ok(rand::thread_rng().random_bool(0.5))

    }

}