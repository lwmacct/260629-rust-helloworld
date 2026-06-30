use std::env;
use std::process;

use lwmacct_260629_rust_helloworld::{ArithmeticError, Operation, calculate};

const USAGE: &str = "Usage: lwmacct-260629-rust-helloworld <add|sub|mul|div> <left> <right>";

fn main() {
    match run(env::args()) {
        Ok(output) => println!("{output}"),
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{USAGE}");
            process::exit(2);
        }
    }
}

fn run<I>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let _program = args.next();
    let operation = parse_operation(args.next().ok_or_else(|| "missing operation".to_string())?)?;
    let left = parse_number(
        args.next()
            .ok_or_else(|| "missing left number".to_string())?,
    )?;
    let right = parse_number(
        args.next()
            .ok_or_else(|| "missing right number".to_string())?,
    )?;

    if args.next().is_some() {
        return Err("too many arguments".to_string());
    }

    calculate(operation, left, right)
        .map(format_number)
        .map_err(format_arithmetic_error)
}

fn parse_operation(value: String) -> Result<Operation, String> {
    match value.as_str() {
        "add" | "+" => Ok(Operation::Add),
        "sub" | "subtract" | "-" => Ok(Operation::Subtract),
        "mul" | "multiply" | "x" | "*" => Ok(Operation::Multiply),
        "div" | "divide" | "/" => Ok(Operation::Divide),
        _ => Err(format!("unknown operation: {value}")),
    }
}

fn parse_number(value: String) -> Result<f64, String> {
    value
        .parse::<f64>()
        .map_err(|_| format!("invalid number: {value}"))
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

fn format_arithmetic_error(error: ArithmeticError) -> String {
    error.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_args(args: &[&str]) -> Result<String, String> {
        run(args.iter().map(|value| value.to_string()))
    }

    #[test]
    fn runs_cli_operations() {
        assert_eq!(run_args(&["calc", "add", "1", "2"]), Ok("3".to_string()));
        assert_eq!(run_args(&["calc", "sub", "1", "2"]), Ok("-1".to_string()));
        assert_eq!(run_args(&["calc", "mul", "3", "2"]), Ok("6".to_string()));
        assert_eq!(run_args(&["calc", "div", "3", "2"]), Ok("1.5".to_string()));
    }

    #[test]
    fn reports_cli_errors() {
        assert!(run_args(&["calc"]).is_err());
        assert!(run_args(&["calc", "pow", "1", "2"]).is_err());
        assert!(run_args(&["calc", "add", "one", "2"]).is_err());
        assert_eq!(
            run_args(&["calc", "div", "1", "0"]),
            Err("division by zero".to_string())
        );
    }
}
