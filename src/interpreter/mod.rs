use std::iter::Map;

pub mod parser;
pub mod tokenizer;

pub fn execute(source: String) {
    let expressions: Vec<parser::Expr> = parser::parse(tokenizer::tokenize(source));
    let mut variables: Map<String, f64>;

    for expression in expressions {
        eval(expression);
    }
}

fn eval(expression: parser::Expr) {
    match expression {
        parser::Expr::Print(expr) => {
            match *expr {
                parser::Expr::NumLiteral(value) => println!("{}", value),
                parser::Expr::StrLiteral(ref value) => println!("{}", value),

                _ => panic!("Print() may have a value")
            }
        }

        _ => ()
    }
}
