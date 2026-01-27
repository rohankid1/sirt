mod cli;
mod gui;
mod sirt;
mod tui;

use clap::Parser;
use color_eyre::{
    Result, Section, SectionExt,
    eyre::{ensure, eyre},
};
use libsirt::{
    Block,
    error::ParseError,
    parse_input,
    types::{Float, Int},
};

use crate::cli::{Sirt, SirtCommand, Using};

fn run(using: Using, blocks: Vec<Block>) -> Result<()> {
    match using {
        Using::Tui => ratatui::run(|term| tui::App::new(blocks).run(term)),
        Using::Gui => gui::run(blocks).map_err(|err| err.into()),
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Sirt::parse();

    match args.command {
        SirtCommand::From { input, using } => {
            let blocks = parse_input(&input);

            match blocks {
                Ok(blocks) => run(using, blocks)?,
                Err(err) => {
                    return Err(eyre!("Error returned during parsing")
                        .with_section(|| err.to_string().header("Parse Error:"))
                        .with_note(|| match err {
                            ParseError::Bool(_) => {
                                "supported boolean values: [true, false, yes, no]".to_string()
                            }
                            ParseError::Int(_) => {
                                format!("int must be between {} and {}", Int::MIN, Int::MAX)
                            }
                            d => format!("Check for syntax errors and try again: {d}"),
                        }));
                }
            }
        }
        SirtCommand::File { path, using } => {
            ensure!(
                path.is_file(),
                "\"{}\" does not exist or is not a file",
                path.to_string_lossy()
            );

            let file = std::fs::read_to_string(path)?;
            let blocks = parse_input(&file);

            match blocks {
                Ok(blocks) => run(using, blocks)?,
                Err(err) => {
                    return Err(eyre!("Error returned during parsing")
                        .with_section(|| err.to_string().header("Parse Error:"))
                        .with_note(|| match err {
                            ParseError::Bool(_) => {
                                "supported boolean values: [true, false, yes, no]".to_string()
                            }
                            ParseError::Int(_) => {
                                format!("int must be between {} and {}", Int::MIN, Int::MAX)
                            }
                            ParseError::Float(_) => {
                                format!("float must be between {} and {}", Float::MIN, Float::MAX)
                            }
                            _ => "Check for syntax errors and try again".to_owned(),
                        }));
                }
            }
        }
    }

    Ok(())
}
