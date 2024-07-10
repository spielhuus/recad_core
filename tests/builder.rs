mod tests {
    mod parser {
        use std::fs::File;

        use recad_core::{
            draw::{At, Attribute, Direction}, gr::Pt, plot::{
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
            builder.draw(LocalLabel::new("Vin").attr(Attribute::Rotate(180.0))).unwrap();
            builder.draw(Wire::new().attr(Attribute::Direction(Direction::Right)).attr(Attribute::Length(4.0 * 2.54))).unwrap();
            builder.draw(Wire::new().attr(Attribute::Direction(Direction::Up)).attr(Attribute::Length(8.0 * 2.54))).unwrap();
            builder.draw(Wire::new().attr(Attribute::Direction(Direction::Right)).attr(Attribute::Length(4.0 * 2.54))).unwrap();
            builder.draw(
                Symbol::new("R1", "100k", "Device:R")
                    .attr(Attribute::Rotate(90.0))
                    .attr(Attribute::Anchor(String::from("1")))
            ).unwrap();
            builder.draw(Wire::new().attr(Attribute::Direction(Direction::Right))).unwrap();
            builder.draw(Junction::new()).unwrap();
            builder.draw(
                Symbol::new("U1", "TL072", "Amplifier_Operational:LM2904")
                    .attr(Attribute::Anchor("2".to_string()))
                    .attr(Attribute::Mirror("x".to_string()))
            ).unwrap();
            builder.draw(Wire::new()
                .attr(Attribute::At(At::Pin("U1".to_string(), "2".to_string())))
                .attr(Attribute::Direction(Direction::Up))
                .attr(Attribute::Length(4.0 * 2.54))
            ).unwrap();
            builder.draw(
                Symbol::new("R2", "100k", "Device:R")
                    .attr(Attribute::Rotate(90.0))
                    .attr(Attribute::Anchor("1".to_string()))
                    .attr(Attribute::Tox(At::Pin("U1".to_string(), "1".to_string())))
            ).unwrap();
            builder.draw(Wire::new().attr(Attribute::Toy(At::Pin("U1".to_string(), "1".to_string())))).unwrap();
            builder.draw(Junction::new()).unwrap();
            builder.draw(Wire::new().attr(Attribute::Direction(Direction::Right)).attr(Attribute::Length(4.0 * 2.54))).unwrap();
            builder.draw(LocalLabel::new("Vout")).unwrap();
            builder.draw(
                Symbol::new("GND", "GND", "power:GND")
                    .attr(Attribute::At(At::Pin("U1".to_string(), "3".to_string())))
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

        #[test]
        fn draw_schema_left() {
            init();

            let mut builder = Schema::new();
            builder.move_to(At::Pt(Pt { x: 50.8, y: 50.8 }));
            builder.draw(LocalLabel::new("Vin").attr(Attribute::Rotate(180.0))).unwrap();
            builder.draw(Wire::new().attr(Attribute::Direction(Direction::Right)).attr(Attribute::Length(4.0 * 2.54))).unwrap();
            builder.draw(Wire::new().attr(Attribute::Direction(Direction::Up)).attr(Attribute::Length(8.0 * 2.54))).unwrap();
            builder.draw(Wire::new().attr(Attribute::Direction(Direction::Right)).attr(Attribute::Length(4.0 * 2.54))).unwrap();
            builder.draw(
                Symbol::new("R1", "100k", "Device:R")
                    .attr(Attribute::Rotate(90.0))
                    .attr(Attribute::Anchor(String::from("1")))
            ).unwrap();
            builder.draw(Wire::new().attr(Attribute::Direction(Direction::Right))).unwrap();
            builder.draw(Junction::new()).unwrap();
            builder.draw(
                Symbol::new("U1", "TL072", "Amplifier_Operational:LM2904")
                    .attr(Attribute::Anchor("2".to_string()))
                    .attr(Attribute::Mirror("x".to_string()))
            ).unwrap();
            builder.draw(Wire::new()
                .attr(Attribute::At(At::Pin("U1".to_string(), "1".to_string())))
                .attr(Attribute::Direction(Direction::Up))
                .attr(Attribute::Length(4.0 * 2.54))
            ).unwrap();
            builder.draw(
                Symbol::new("R2", "100k", "Device:R")
                    .attr(Attribute::Rotate(270.0))
                    .attr(Attribute::Anchor("1".to_string()))
                    .attr(Attribute::Tox(At::Pin("U1".to_string(), "2".to_string())))
            ).unwrap();
            builder.draw(Wire::new().attr(Attribute::Toy(At::Pin("U1".to_string(), "2".to_string())))).unwrap();
            builder.draw(Junction::new()).unwrap();
            //builder.draw(Wire::new().attr(Attribute::Direction(Direction::Right)).attr(Attribute::Length(4.0 * 2.54))).unwrap();
            //builder.draw(LocalLabel::new("Vout")).unwrap();
            //builder.draw(
            //    Symbol::new("GND", "GND", "power:GND")
            //        .attr(Attribute::At(At::Pin("U1".to_string(), "3".to_string())))
            //).unwrap();

            //builder.write(&mut std::io::stdout()).unwrap();
            let mut file = File::create("/tmp/test_builder_left.kicad_sch").unwrap();
            builder.write(&mut file).unwrap();

            let mut svg = SvgPlotter::new();
            builder
                .plot(&mut svg, &Theme::from(Themes::Kicad2020))
                .unwrap();
            let mut file = File::create("/tmp/test_builder_left.svg").unwrap();
            svg.write(&mut file).unwrap();
        }
    }
}
