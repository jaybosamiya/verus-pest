use anyhow::anyhow;
use pest::Parser;
use pest_derive::Parser;

// Turns out, pest_derive (for some unknown reason, on our grammar, but not on smaller examples)
// requires us to specify `extern crate alloc`; since we're already in std land, this is perfectly
// fine, but weird that it is needed.
extern crate alloc;

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
