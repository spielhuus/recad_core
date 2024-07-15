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

    if args.len() < 2 {
        eprintln!("Usage: {} input_file", args[0]);
        return;
    }

    let input_path = Path::new(&args[1]);

    let schema = Schema::load(input_path).unwrap();
    let mut femto_vg = recad_core::plot::FemtoVgPlotter::new();
    schema.plot(&mut femto_vg, PlotCommand::new().border(Some(true))).unwrap(); 
    femto_vg.open();
}
