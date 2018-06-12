use core::slice;
use std::io::{self, BufRead, BufReader};
use std::fmt;
use std::fs::File;
use std::num::ParseFloatError;
use std::path::Path;
use std::str::FromStr;
use sfml::system::Vector2f;
use attrib::Attrib;

pub struct WorldDef {
    attribs: Vec<Attrib>,
}

impl WorldDef {
    pub fn load<P: AsRef<Path>>(filepath: P) -> Result<WorldDef, WorldDefErr> {
        let mut reader = match File::open(filepath) {
            Ok(f) => BufReader::new(f),
            Err(io_error) => return Err(WorldDefErr::IoError(io_error)),
        };

        let mut attribs = Vec::new();

        let mut content = String::new();
        let mut line_counter = 0;
        loop {
            content.truncate(0);

            match reader.read_line(&mut content) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break
                    }

                    if content.starts_with('#') || content.trim() == ""{
                        continue
                    }

                    /*for i in content.split_whitespace() {
                        println!("i: {}", i);
                    }*/

                    //println!("--");

                    let mut components = Vec::new();
                    for i in content.split_whitespace() {
                        match f32::from_str(i) {
                            Ok(val) => {
                                if !val.is_finite() {
                                    return Err(WorldDefErr::ValueError)
                                }

                                //println!("v: {}", val);

                                components.push(val);
                            }
                            Err(parse_error) => return Err(WorldDefErr::ParseError(parse_error))
                        }
                    }

                    //println!("---------------- ({} components)", components.len());

                    if components.len() == 2 {
                        attribs.push(Attrib::Node(Vector2f::new(components[0], components[1])));
                    } else {
                        return Err(WorldDefErr::ComponentCountError)
                    }
                },
                Err(io_error) => {
                    return Err(WorldDefErr::IoError(io_error))
                },
            }

            line_counter += 1;
        }

        //println!("----------------\nfinal line_counter: {}", line_counter);

        if line_counter == 0 {
            return Err(WorldDefErr::NoDeclarationsError)
        }

        Ok(WorldDef {
            attribs,
        })
    }

    pub fn attribs<'a>(&'a self) -> slice::Iter<'a, Attrib> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        self.attribs.len()
    }
}

impl<'a> IntoIterator for &'a WorldDef {
    type Item = &'a Attrib;
    type IntoIter = slice::Iter<'a, Attrib>;

    fn into_iter(self) -> Self::IntoIter {
        self.attribs.iter()
    }
}

#[derive(Debug)]
pub enum WorldDefErr {
    IoError(io::Error),
    ParseError(ParseFloatError),
    ValueError,
    ComponentCountError,
    NoDeclarationsError,
}

impl fmt::Display for WorldDefErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
