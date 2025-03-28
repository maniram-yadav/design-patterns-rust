mod builders;
mod payment_gateway;

fn main(){
	println!("Hello Design pattern");
	let gateway = payment_gateway::PaymentGateway::new();
	
}