mod constants;
mod tags;
mod token;

use compiler::token::{Expression, Scanner, Token};
use constants::{E, E_, T, T_, F};


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
    let expr_text = "43*(2 + 34) - 123 / (2 - 34) #";
    let mut expr = Expression::new(expr_text);

    loop {
        let token = expr.next_token();
        println!("Token: {:?}", token);

        if matches!(token, Token::EndMark(_)) {
            break;
        }
    }
}