mod my_read_line;
mod structures;

use structures::Expression;

fn main() {
    loop {
        let mut expression = String::new();
        println!("Pls, input expression. press 'q' for quit");
        my_read_line::input(&mut expression);
        match Expression::solve_expression(expression) {
            Ok(result) => {
                println!("answer:{result}");
                break;
            }
            Err(e) => println!("error:{e}. Try again."),
        }
    }
}
