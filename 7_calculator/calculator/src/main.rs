use std::io::{self, BufRead};
use calculator::*;
use pest::Parser;

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match CalculatorParser::parse(Rule::equation, &line?) {
            Ok(mut pairs) => {
                println!("Parsed: {:#?}", parse_expr(pairs.next().unwrap().into_inner()));
            },
            Err(e) => {
                println!("Parse failed: {:?}", e);
            }
        }
    }

    Ok(())
}
