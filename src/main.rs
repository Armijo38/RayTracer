use ray_tracer::{process,Config};
use image;

use std::fs;

use clap::Parser;


#[derive(Parser,Debug)]
struct Args {
    /// Path to config file
    #[arg(short, long)]
    config: String,

    /// Path to output file
    #[arg(short, long)]
    output_file: String,

    /// Print objects info
    #[arg(short, long, default_value_t = false)]
    print_debug_objects: bool,
}


fn main() {
    let args = Args::parse();

    let config_file_raw: String = fs::read_to_string(&args.config).expect("Should have been able to read config file");
    let config: Config = serde_json::from_str(&config_file_raw).expect("Should have been able to parse config file");

    let img = process(config, args.print_debug_objects);

    img.save_with_format(args.output_file, image::ImageFormat::Png).expect("Can not save result image");
}
