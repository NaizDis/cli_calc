mod calculator;
use calculator::{Calculator, Error};

fn main() -> Result<(), Error> {
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let tokens = Calculator::parse(input);
                if tokens.is_err() {
                    println!("{:?}", tokens.err().unwrap());
                    continue;
                }
                let expr = Calculator::post_fix(tokens?);
                if let Some(v) = Calculator::evaluate(expr) {
                    println!("Ans : {}", v);
                }
            }
            Err(error) => println!("error : {}", error),
        }
    }
}
