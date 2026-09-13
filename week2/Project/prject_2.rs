fn main() {
	let ta:f64 = 450_000.0;
	let ma:f64 = 1_500_000.0;
	let ha:f64 = 750_000.0;
	let da:f64 = 2_850_000.0;
	let aca:f64 = 250_000.0;
	let tq:f64 = 2.0;
	let mq:f64 = 1.0;
	let hq:f64 = 3.0;
	let dq:f64 = 3.0;
	let acq:f64 = 1.0;
	let sum:f64 = (ta*tq)+(ma*mq)+(ha*hq)+(da*dq)+(acq*aca);
	let qty:f64 = tq+mq+hq+dq+acq;
	let average:f64 = sum/qty	;

	println!("Sum is {}",sum);
	println!("average is {}",average);
}