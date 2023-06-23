use std::collections::BTreeSet;

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

    let parsed = parsed.flatten().collect::<Vec<_>>();

    let parsed = parsed
        .into_iter()
        .filter(|p| matches!(p.as_rule(), Rule::expr_inner | Rule::expr))
        .map(|p| p.as_str().trim())
        .collect::<BTreeSet<_>>();

    println!("{parsed:#?}");

    dbg!(parsed.len());

    Ok(())
}
