use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use calc::Calc;

mod ast;
mod calc;
mod expr;
mod factor;
mod lexer;
mod parser;
mod term;
mod token;
mod token_vec;

fn main() -> Result<(), Box<dyn Error>> {
    let calc = Calc::new();

    loop {
        print!("> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        let res = calc.run(&input)?;
        println!("{}", res);
    }
}
