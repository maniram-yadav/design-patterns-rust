mod builders;
mod payment_gateway;
extern crate rand;

fn main() {

	println!("Hello Design pattern");
	let gateway = payment_gateway::payment_gateway::PaymentGateway::new();	
}