mod tests {
    mod parser {
        use std::fs::File;

        use recad_core::{
            draw::At, gr::Pt, plot::{
                theme::{Theme, Themes},
                Plotter, SvgPlotter,
            }, schema::{Junction, LocalLabel, Symbol, Wire}, Drawable, Drawer, Plot, Schema
        };
        fn init() {
            let _ = env_logger::builder().is_test(true).try_init();
        }
        #[test]

        fn draw_schema() {
            init();

            let mut builder = Schema::new();
            builder.move_to(At::Pt(Pt { x: 50.8, y: 50.8 }));
            builder.draw(LocalLabel::new("Vin").rotate(180.0)).unwrap();
            builder.draw(Wire::new().right().len(4.0)).unwrap();
            builder.draw(Wire::new().up().len(8.0)).unwrap();
            builder.draw(Wire::new().right().len(4.0)).unwrap();
            builder.draw(
                Symbol::new("R1", "100k", "Device:R")
                    .rotate(90.0)
                    .anchor("1"),
            ).unwrap();
            builder.draw(Wire::new().right()).unwrap();
            builder.draw(Junction::new()).unwrap();
            builder.draw(
                Symbol::new("U1", "TL072", "Amplifier_Operational:LM2904")
                    .anchor("2")
                    .mirror("x"),
            ).unwrap();
            builder.draw(Wire::new().up().len(4.0)).unwrap();
            builder.draw(
                Symbol::new("R2", "100k", "Device:R")
                    .anchor("1")
                    .tox("U1", "1"),
            ).unwrap();

            //builder.write(&mut std::io::stdout()).unwrap();
            let mut file = File::create("/tmp/test_builder.kicad_sch").unwrap();
            builder.write(&mut file).unwrap();

            let mut svg = SvgPlotter::new();
            builder
                .plot(&mut svg, &Theme::from(Themes::Kicad2020))
                .unwrap();
            let mut file = File::create("/tmp/test_builder.svg").unwrap();
            svg.write(&mut file).unwrap();
        }
    }
}
