mod tests {
    mod parser {
        use std::path::Path;
        use recad_core::{plot::theme::{Theme, Themes}, Plot, plot::Plotter};

        const TESTS_DIR: &str = "target/out/tests";
        const ECHO_IN: &str = "tests/echo/echo.kicad_sch";
        const ECHO_OUT: &str = "target/out/echo.kicad_sch";
        const ECHO_PLOT: &str = "target/out/echo.svg";
        const SUMME_IN: &str = "tests/summe/summe.kicad_sch";
        const SUMME_OUT: &str = "target/out/summe.kicad_sch";
        const SUMME_PLOT: &str = "target/out/summe.svg";
        const CP3_IN: &str = "tests/cp3/cp3.kicad_sch";
        const CP3_OUT: &str = "target/out/cp3.kicad_sch";
        const CP3_PLOT: &str = "target/out/cp3.svg";

        fn init() {
            let _ = env_logger::builder().is_test(true).try_init();
            let path = Path::new(TESTS_DIR);
            if !path.exists() {
                std::fs::create_dir_all(path).unwrap();
            }
        }

        #[test]
        fn load_echo() {
            init();
            let schema = recad_core::Schema::load(Path::new(ECHO_IN)).unwrap();
            let mut file = std::fs::File::create(ECHO_OUT).unwrap();
            schema.write(&mut file).unwrap();

            let mut svg = recad_core::plot::SvgPlotter::new();
            schema.plot(&mut svg, &Theme::from(Themes::Kicad2020)).unwrap();
            let mut file = std::fs::File::create(ECHO_PLOT).unwrap();
            svg.write(&mut file).unwrap();
        }
        
        #[test]
        fn load_summe() {
            init();
            let schema = recad_core::Schema::load(Path::new(SUMME_IN)).unwrap();
            let mut file = std::fs::File::create(SUMME_OUT).unwrap();
            schema.write(&mut file).unwrap();

            let mut svg = recad_core::plot::SvgPlotter::new();
            schema.plot(&mut svg, &Theme::from(Themes::Kicad2020)).unwrap();
            let mut file = std::fs::File::create(SUMME_PLOT).unwrap();
            svg.write(&mut file).unwrap();
        }

        //#[test]
        //fn load_cp3() {
        //    init();
        //    let schema = recad_core::Schema::load(Path::new(CP3_IN)).unwrap();
        //    let mut file = std::fs::File::create(CP3_OUT).unwrap();
        //    schema.write(&mut file).unwrap();
        //
        //    let mut svg = recad_core::plot::SvgPlotter::new();
        //    schema.plot(&mut svg, &Theme::from(Themes::Kicad2020)).unwrap();
        //    let mut file = std::fs::File::create(CP3_PLOT).unwrap();
        //    svg.write(&mut file).unwrap();
        //}
    }
}
