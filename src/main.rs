mod api;
use api::*;

use exitfailure::ExitFailure;
use std::{fmt, io};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    println!("Connected to DuckDuckGo!");
    let mut running = true;

    while running {
        let inp = input("Enter your question: ");
        match &inp.trim().to_lowercase()[..] {
            "exit" => {
                std::process::exit(1);
            }

            _ => {
                let res = DuckDuckGoQuery::new(&*inp).await;
                let res =
                    res.unwrap_or_else(|val| -> DuckDuckGoQuery { DuckDuckGoQuery::blank_query() });
                println!("{}", res)
            }
        }
    }

    Ok(())
}

fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush();
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    ret
}
