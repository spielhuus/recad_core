mod tests {
    mod parser {
        use std::path::Path;
        use recad_core::{plot::theme::{Theme, Themes}, Plot, plot::Plotter};

        fn init() {
            let _ = env_logger::builder().is_test(true).try_init();
            let path = Path::new("/tmp/recad");
            if !path.exists() {
                std::fs::create_dir_all(path).unwrap();
            }
        }

        #[test]
        fn load_echo() {
            init();
            let schema = recad_core::Schema::load(Path::new("tests/echo/echo.kicad_sch")).unwrap();
            let mut file = std::fs::File::create("/tmp/recad/echo.kicad_sch").unwrap();
            schema.write(&mut file).unwrap();

            let mut svg = recad_core::plot::SvgPlotter::new();
            schema.plot(&mut svg, &Theme::from(Themes::Kicad2020)).unwrap();
            let mut file = std::fs::File::create("/tmp/recad/echo.svg").unwrap();
            svg.write(&mut file).unwrap();
        }
        
        #[test]
        fn load_summe() {
            init();
            let schema = recad_core::Schema::load(Path::new("tests/summe.kicad_sch")).unwrap();
            let mut file = std::fs::File::create("/tmp/recad/summe.kicad_sch").unwrap();
            schema.write(&mut file).unwrap();

            let mut svg = recad_core::plot::SvgPlotter::new();
            schema.plot(&mut svg, &Theme::from(Themes::Kicad2020)).unwrap();
            let mut file = std::fs::File::create("/tmp/recad/summe.svg").unwrap();
            svg.write(&mut file).unwrap();
        }

        //#[test]
        //fn load_pcb() {
        //    init();
        //    let pcb = recad_core::Pcb::load(Path::new("tests/echo/echo.kicad_pcb"));
        //
        //    assert_eq!(254, pcb.segments.len());
        //    assert_eq!(51, pcb.nets.len());
        //    assert_eq!(70, pcb.footprints.len());
        //    //let schema = crate::Schema::load(Path::new("/usr/share/kicad/demos/kit-dev-coldfire-xilinx_5213/kit-dev-coldfire-xilinx_5213.kicad_sch"));
        //    //let schema = crate::schema::Schema::load(Path::new("/home/etienne/github/elektrophon/src/threeler/main/main.kicad_sch"));
        //
        //    let svg = recad_core::plot::SvgPlotter::new();
        //    //let mut plotter = recad_core::plot::SchemaPlotter::new(schema, svg, recad_core::plot::theme::Themes::Kicad2020);
        //    //let mut file = std::fs::File::create("/tmp/summe.svg").unwrap();
        //    //plotter.plot();
        //    //plotter.write(&mut file).unwrap();
        //
        //}
    }
}
