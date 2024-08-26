use hta_generator::HtaGenerator;
use config::load_config;
use logger::init_logger;
use std::env;
use std::process;

fn main() {
    init_logger();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: hta-gen <output_file> <driver_file>");
        process::exit(1);
    }

    let output_file = &args[1];
    let driver_file = &args[2];

    let generator = HtaGenerator::new(output_file, driver_file);
    if let Err(e) = generator.generate() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    println!("HTA file successfully generated.");
}
