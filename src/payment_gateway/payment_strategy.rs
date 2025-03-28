use std::collections::HashMap;

trait PaymentStrategy {
    fn pay(&self,amount:f64,payment_details:&HashMap<String,String>) -> Result<bool,String>;
}