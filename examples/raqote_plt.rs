use std::{
    fs::File, path::Path
};

use recad_core::{
    Schema, Plot,
    plot::{
        Plotter,
        PlotCommand
    },
};

fn main() {
    let path = Path::new("tests/summe/summe.kicad_sch");
    let schema = Schema::load(path).unwrap();
    let mut svg = recad_core::plot::RaqotePlotter::new();
    schema.plot(&mut svg, PlotCommand::default().border(Some(true))).unwrap(); 
    svg.save(Path::new("raqote_summe.png")).unwrap();
}
