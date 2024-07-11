use std::path::{Path, PathBuf};

use {
    pcb::{Footprint, Layer, Net, Segment},
    symbols::LibrarySymbol,
    sexp::{parser::SexpParser, SexpTree},
};

mod circuit;
pub mod draw;
pub mod gr;
mod math;
mod netlist;
pub mod pcb;
pub mod plot;
pub mod schema;
pub mod footprint;
pub mod symbols;
mod symbols_reader;
mod symbols_writer;
mod schema_reader;
mod schema_writer;
mod schema_ploter;
mod sexp;

///create an UUID.
#[macro_export]
macro_rules! uuid {
    () => {
        uuid::Uuid::new_v4().to_string()
    };
}

#[inline(always)]
fn round(n: f32) -> f32 {
    format!("{:.4}", n).parse().unwrap()
}

#[inline(always)]
fn yes_or_no(input: bool) -> String {
    if input {
        String::from(el::YES)
    } else {
        String::from(el::NO)
    }
}

///The Error struct used for all error handling.
#[derive(Debug)]
pub struct Error(pub String, pub String);

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self("io".to_string(), e.to_string())
    }
}

///The Circuit struct represents a spice netlist.
#[derive(Debug, Clone, PartialEq)]
pub struct Circuit {
    name: String,
    pathlist: Vec<String>,
    items: Vec<CircuitItem>,
    subcircuits: IndexMap<String, (Vec<String>, Circuit)>,
    pub controls: Vec<String>,
    pub options: IndexMap<String, String>,
}

#[derive(Debug, Default)]
///Define the `Schematic` file format.
pub struct Schema {
    /// The `version` defines the schematic version
    /// using the YYYYMMDD date format.
    pub version: String,
    /// The `uuid` defines the universally unique identifier for
    /// the schematic file.
    pub uuid: String,
    /// `generator` defines the program used
    /// to write the file.
    pub generator: String,
    /// `generator_version` specifies the program version for file writing
    pub generator_version: Option<String>,
    pub paper: gr::PaperSize,
    pub title_block: gr::TitleBlock,
    pub library_symbols: Vec<LibrarySymbol>,
    pub items: Vec<SchemaItem>,
    pub sheet_instances: Vec<Instance>,
    
    //attributes for the builder.
    grid: f32,
    last_pos: draw::At,
}

///Pcb file format for all versions of KiCad from 6.0.
#[derive(Default)]
pub struct Pcb {
    ///The version token attribute defines the pcb version
    ///using the YYYYMMDD date format.
    pub version: String,
    ///The UNIQUE_IDENTIFIER defines the universally unique identifier for
    ///the pcb file.
    pub uuid: String,
    ///The generator token attribute defines the program used to write the file.
    pub generator: String,
    ///The generator_version token attribute defines the program version
    ///used to write the file.
    pub generator_version: Option<String>,
    //
    //General
    //
    //Layers
    pub layers: Vec<Layer>,

    //Setup
    //
    //Properties
    ///The ```net``` token defines a net for the board. This section is
    ///required. <br><br>
    pub nets: Vec<Net>,
    //
    ///The footprints on the pcb.
    pub footprints: Vec<Footprint>,
    //
    //Graphic Items
    //
    //Images
    pub segments: Vec<Segment>,
    //Zones
    //
    //Groups
}

impl Pcb {
    ///Load a pcb from a path
    pub fn load(path: &Path) -> Self {
        let parser = crate::sexp::parser::SexpParser::load(path).unwrap();
        let tree = crate::sexp::SexpTree::from(parser.iter()).unwrap();
        tree.into()
    }
}

///implement the symbol lirarary.
pub struct SymbolLibrary {
    pathlist: Vec<PathBuf>,
}

use circuit::CircuitItem;
use draw::{At, Attribute};
use gr::Pt;
use indexmap::IndexMap;
use plot::{theme::Theme, Plotter};
use schema::{Instance, SchemaItem};
use sexp::{builder::Builder, constants::el, SexpValue};

impl SymbolLibrary {
    ///Load a symbol from the symbol library, the name is the combination
    ///of the filename of the library and the symbol name. 
    pub fn load(&self, name: &str) -> Result<LibrarySymbol, Error> {
        let t: Vec<&str> = name.split(':').collect();
        for path in &self.pathlist {
            let filename = &format!("{}/{}.kicad_sym", path.to_str().unwrap(), t[0]);
            if let Ok(doc) = SexpParser::load(Path::new(filename)) {
                if let Ok(tree) = SexpTree::from(doc.iter()) {
                    for node in tree.root().unwrap().query(el::SYMBOL) {
                        let sym_name: String = node.get(0).unwrap();
                        if sym_name == t[1] {
                            let mut node: LibrarySymbol = Into::<Result<LibrarySymbol, Error>>::into(node)?;
                            node.lib_id = format!("{}:{}", t[0], t[1]);

                            if let Some(extends) = &node.extends {
                                if let Ok(mut ext_sym) = self.load(&format!("{}:{}", t.first().unwrap(), extends)) {
                                    ext_sym.props.clone_from(&node.props);
                                    ext_sym.lib_id = format!("{}:{}", t[0], t[1]);
                                    return Ok(ext_sym);
                                } else {
                                    return Err(Error("lib_symbol".to_string(), format!("unable to find extend symbol {}", extends)))
                                }
                            }

                            return Ok(node);
                        }
                    }
                }
            }
        }
        Err(Error(
            String::from("load_library"),
            format!("can not find library: {}", name),
        ))
    }
}


/// plot a [`Schema`] or [`Pcb`].
///
/// Available plotters:
///
/// - [`plot::SvgPlotter`] - plot to a [SVG](https://www.w3.org/TR/SVG11/) file.
///
/// Example usage:
/// 
/// ```
/// use std::{
///     io::Write,
///     path::Path,
/// };
///
/// use recad_core::{
///     Schema, Plot,
///     plot::{
///         Plotter,
///         theme::*
///     },
/// };
///
/// let path = Path::new("tests/summe.kicad_sch");
/// let schema = Schema::load(path).unwrap();
///
/// let mut svg = recad_core::plot::SvgPlotter::new();
/// schema.plot(&mut svg, &Theme::from(Themes::Kicad2020)).unwrap();
///
/// let mut file = std::fs::File::create("/tmp/summe.svg").unwrap();
/// let res = svg.write(&mut file);
/// assert!(res.is_ok());
/// ```
pub trait Plot {
    fn plot(&self, plotter: &mut impl Plotter, theme: &Theme) -> Result<(), Error>;
    fn move_to(&mut self, pt: At);
    fn get_pt(&self, at: &At) -> Pt;
}

trait SexpWrite {
    fn write(&self, builder: &mut Builder) -> Result<(), Error>;
}

/// Access attributes of the drawable elements.
pub trait Drawable<F> {
    fn attr(self, attr: Attribute) -> F;
}

///Creat a schema or pcb file from code.
pub trait Drawer<T> {
    fn draw(&mut self, item: T) -> Result<(), Error>;
}


#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::{
        schema::{SchemaItem, Symbol},
        Schema,
    };

    #[test]
    fn test_load_symbol() {
        let lib = super::SymbolLibrary {
            pathlist: vec![PathBuf::from("/usr/share/kicad/symbols")]
        };
        let sym = lib.load("Amplifier_Operational:LM2904");
        assert!(sym.is_ok());
    }

    #[test]
    fn test_load_extends_symbol() {
        let lib = super::SymbolLibrary {
            pathlist: vec![PathBuf::from("/usr/share/kicad/symbols")]
        };
        let sym = lib.load("Amplifier_Operational:TL072");
        assert!(sym.is_ok());
        assert_eq!(3, sym.as_ref().unwrap().units.len());
        assert_eq!("Amplifier_Operational:TL072", sym.unwrap().lib_id);
    }
}

