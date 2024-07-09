mod tests {
    mod parser {
        use std::{fs::File, path::Path};

        use recad::{
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

            let builder = Schema::new()
                .move_to(At::Pt(Pt { x: 50.8, y: 50.8 }))
                .draw(LocalLabel::new("Vin").rotate(180.0))
                .draw(Wire::new().right().len(4.0))
                .draw(Wire::new().up().len(8.0))
                .draw(Wire::new().right().len(4.0))
                .draw(
                    Symbol::new("R1", "100k", "Device:R")
                        .rotate(90.0)
                        .anchor("1"),
                )
                .draw(Wire::new().right())
                .draw(Junction::new())
                .draw(
                    Symbol::new("U1", "TL072", "Amplifier_Operational:LM2904")
                        .anchor("2")
                        .mirror("x"),
                )
                .draw(Wire::new().up().len(4.0))
                .draw(
                    Symbol::new("R2", "100k", "Device:R")
                        .anchor("1")
                        .tox("U1", "1"),
                );

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
