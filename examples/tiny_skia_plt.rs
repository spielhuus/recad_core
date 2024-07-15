use std::{
    env, path::Path
};

use recad_core::{
    Schema, Plot,
    plot::{
        Plotter,
        PlotCommand
    },
};
fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} input_file output_file", args[0]);
        return;
    }

    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    let schema = Schema::load(input_path).unwrap();
    let mut svg = recad_core::plot::TinySkiaPlotter::new();
    schema.plot(&mut svg, PlotCommand::new().border(Some(true))).unwrap();
    svg.save(output_path).unwrap();
}

