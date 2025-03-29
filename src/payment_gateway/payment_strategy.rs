use std::collections::HashMap;

pub trait PaymentStrategy {
    fn pay(&self,amount:f64,payment_details:&HashMap<String,String>) -> Result<bool,String>;
}