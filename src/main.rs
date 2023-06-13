extern crate pest;
#[macro_use]
extern crate pest_derive;

use anyhow::anyhow;
use pest::Parser;

#[derive(Parser)]
#[grammar = "verus.pest"]
pub struct VerusParser;

fn main() -> anyhow::Result<()> {
    let unparsed_file =
        std::fs::read_to_string(std::env::args().nth(1).ok_or(anyhow!("need argument"))?)?;

    let parsed = VerusParser::parse(Rule::file, &unparsed_file)?;

    println!("{parsed:#?}");

    Ok(())
}
