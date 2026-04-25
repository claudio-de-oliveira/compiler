
mod variable;
mod tags;
mod token;

use compiler::token::{Expression, Scanner, Token};
use variable::{E, E_, T, T_, F};


fn main() {
    // test_constants();
    // test_tags();
    test_scanner();
}

fn test_constants() {
    println!("Constante E: {:?}", E);
    println!("Constante E_: {:?}", E_);
    println!("Constante T: {:?}", T);
    println!("Constante T_: {:?}", T_);
    println!("Constante F: {:?}", F);
}

fn test_tags() {
    let tag1 = E;
    let tag2 = T;

    println!("Tag1: {:?}", tag1);
    println!("Tag2: {:?}", tag2);

    println!("Tag1 is variable? {}", tag1.is_variable());
    println!("Tag2 is variable? {}", tag2.is_variable());
}

fn test_scanner() {
    // let expr_text = "let x  43 * (y + 34) - foo / (if 2 - 34) #";
    let expr_text = "as break const continue crate else enum extern     fn     if     in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn id";
    let mut expr = Expression::new(expr_text);

    loop {
        let token = expr.next_token();
        println!("Token: {:?}", token.clone());

        if matches!(token, Token::EndMark(_)) {
            break;
        }

        if matches!(token, Token::Error(_, _)) {
            break;
        }
    }
}