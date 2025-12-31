use std::{env, path::Path};

use recad_core::{
    plot::{PlotCommand, Plotter},
    Plot, Schema,
};

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} input_file output_file", args[0]);
        return;
    }

    let input_path = Path::new(&args[1]);

    let schema = Schema::load(input_path).unwrap();
    let mut raqote = recad_core::plot::RaqotePlotter::new();
    schema
        .plot(&mut raqote, PlotCommand::new().border(Some(true)))
        .unwrap();
    raqote.save(Path::new(&args[2])).unwrap();
}
