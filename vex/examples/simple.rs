use std::time::Duration;

use minifier::Minifier;
use vex::vex;

const SAMPLE_1: &str = "./vex/examples/samples/simple.css";

fn benchmark<T: FnOnce()>(cl: T) -> Duration {
	let start = std::time::Instant::now();
	cl();
	let end = std::time::Instant::now();
	return end - start;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let input = std::fs::read_to_string(SAMPLE_1)?;
	let input = input.as_str();

	let mut minified = None;
	let diff = benchmark(|| {
		minified = vex(input, vec![Minifier]);
	});

	println!("Minified in {:.4}s", diff.as_secs_f32());

	let minified = minified.unwrap();
	println!("{} -> {}", input.len(), minified.len());

	std::fs::write(SAMPLE_1, minified)?;

	Ok(())
}
