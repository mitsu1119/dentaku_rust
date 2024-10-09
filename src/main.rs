use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use calc::Calc;

mod calc;
mod lexer;
mod token;

fn main() -> Result<(), Box<dyn Error>> {
    let calc = Calc::new();

    loop {
        print!("> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        let res = calc.run(&input);

        println!("input: {}", input);
        println!("res  : {}", res);
    }
}
