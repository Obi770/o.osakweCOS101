fn main(){
	let p:f32 = 210_000.0;
	let r:f32 = 5.0;
	let t:f32 = 3.0;
	let a:f32 = p*(1.0-r/100.0).powf(t);
	println!("The value of the TV depreciates to {}",a);
}	