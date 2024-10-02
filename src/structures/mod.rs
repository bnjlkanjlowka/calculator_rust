mod my_extension;

use my_extension::MyStringExtension;

#[derive(PartialEq)]
pub struct Expression {
    current_operation: Operation,
}

impl Expression {
    fn get_current_operation(expression: &String) -> Result<Expression, String> {
        let mut parenthese_number: i32 = 0;
        let mut max_parenthese_number = 0;

        let mut operators_in_parentheses = 0;

        let mut max_priority = 0;
        let mut operator_index_max_priority = 0;
        let mut chr_operator_max_priotiry: char = '0';

        let mut negative_flag = false;

        for (index, chr) in expression.chars().enumerate() {
            let priority: usize;
            match chr {
                ')' => {
                    if parenthese_number > 1 {
                        operators_in_parentheses = 0;
                    }
                    if negative_flag {
                        negative_flag = false;
                    } else {
                        parenthese_number -= 1;
                    }
                    continue;
                }
                '(' => {
                    if Self::is_negative(expression, index) {
                        negative_flag = true;
                    } else {
                        parenthese_number += 1;
                    }
                    continue;
                }
                '+' => {
                    if parenthese_number > 1 {
                        operators_in_parentheses += 1;
                    }
                    priority = 1;
                }
                '-' => {
                    if parenthese_number > 1 {
                        operators_in_parentheses += 1;
                    }
                    if !negative_flag {
                        priority = 1;
                    } else {
                        continue;
                    }
                }
                '*' | '/' => {
                    if parenthese_number > 1 {
                        operators_in_parentheses += 1;
                    }
                    priority = 2;
                }
                '^' => {
                    if parenthese_number > 1 {
                        operators_in_parentheses += 1;
                    }
                    priority = 3;
                }
                '0'..='9' => continue,
                other => {
                    return Err(String::from(format!(
                        "unknown operator:{other}, index:{index}"
                    )))
                }
            }
            if operators_in_parentheses == 1 {
                let parenthese_number = parenthese_number - 1;
                if parenthese_number > max_parenthese_number {
                    max_parenthese_number = parenthese_number;
                    operator_index_max_priority = index;
                    chr_operator_max_priotiry = chr;
                    max_priority = priority;
                } else if parenthese_number == max_parenthese_number && priority > max_priority {
                    max_priority = priority;
                    operator_index_max_priority = index;
                    chr_operator_max_priotiry = chr;
                }
            } else {
                if parenthese_number > max_parenthese_number {
                    max_parenthese_number = parenthese_number;
                    operator_index_max_priority = index;
                    chr_operator_max_priotiry = chr;
                    max_priority = priority;
                } else if parenthese_number == max_parenthese_number && priority > max_priority {
                    max_priority = priority;
                    operator_index_max_priority = index;
                    chr_operator_max_priotiry = chr;
                }
            }
        }
        let (arg1, arg2) = Expression::get_args(&expression, operator_index_max_priority);
        let current_operation = Operation::get_operator_from_chr(
            chr_operator_max_priotiry,
            operator_index_max_priority,
            arg1,
            arg2,
        )
        .expect("later work");
        Ok(Expression { current_operation })
    }

    fn check_expression(expression: &String) -> Result<(), String> {
        if expression.len() == 0 {
            return Err(String::from("nothing entered"));
        }
        if Self::is_answer(expression) {
            return Err(String::from("input is just number"));
        }
        let mut parenthese_number: u32 = 0;
        for (index, chr) in expression.chars().enumerate() {
            match chr {
                '(' => parenthese_number += 1,
                ')' => {
                    if parenthese_number != 0 {
                        parenthese_number -= 1;
                    } else {
                        return Err(String::from(format!("Extra ')'")));
                    }
                }
                '+' | '-' | '*' | '/' | '^' => match Self::check_operation(expression, index) {
                    Ok(()) => continue,
                    Err(e) => return Err(e),
                },
                '0'..='9' => continue,
                other => return Err(format!("unknown operator:{other}, index:{index}")),
            }
        }
        if parenthese_number > 0 {
            return Err(format!("Extra '('"));
        }
        Ok(())
    }

    fn check_operation(expression: &String, index_operation: usize) -> Result<(), String> {
        match (
            Self::check_left_arg(expression, index_operation),
            Self::check_right_arg(expression, index_operation),
        ) {
            (Ok(()), Ok(())) => return Ok(()),
            (Ok(()), Err(e)) => return Err(e),
            (Err(e), Ok(())) => return Err(e),
            (Err(e1), Err(e2)) => return Err(format!("{e1}, {e2}")),
        }
    }

    fn check_left_arg(expression: &String, index_operation: usize) -> Result<(), String> {
        let temp_string = &expression[0..index_operation];
        let expression_len = expression.len();
        let mut negative_flag = false;
        let temp = &expression[index_operation..index_operation + 1];
        if temp == "-" {
            negative_flag = true;
        }
        for (index, chr) in temp_string.chars().rev().enumerate() {
            let index = expression_len - index_operation - index - 1;
            match chr {
                '0'..='9' => return Ok(()),
                ')' => continue,
                '(' => {
                    if negative_flag {
                        return Ok(());
                    } else {
                        return Err(format!("Incorrect expression. {chr}, index:{index}"));
                    }
                }
                _ => return Err(format!("Incorrect expression. {chr}, index:{index}")),
            }
        }
        Ok(())
    }

    fn check_right_arg(expression: &String, index_operation: usize) -> Result<(), String> {
        let mut negative_flag = false;
        for (index, chr) in expression.chars().skip(index_operation + 1).enumerate() {
            let index = index_operation + 1 + index;
            match chr {
                '0'..='9' => return Ok(()),
                '(' => negative_flag = true,
                '-' => {
                    if negative_flag {
                        return Ok(());
                    } else {
                        return Err(format!("Incorrect expression. {chr}, index:{index}"));
                    }
                }
                _ => return Err(format!("Incorrect expression. {chr}, index:{index}")),
            }
        }
        Ok(())
    }

    fn check_for_extra_parentheses(expression: &mut String) {
        loop {
            let mut parentheses_number = 0;
            let mut first_parenthese = false;
            let mut last_parenthese = false;

            let mut index_first_opening_paranthesis: usize = 0;
            let mut index_last_closing_paranthesis: usize = 0;
            for (index, chr) in expression.chars().enumerate() {
                match chr {
                    '(' => {
                        if !first_parenthese && !last_parenthese {
                            first_parenthese = true;
                            index_first_opening_paranthesis = index;
                        }
                        parentheses_number += 1;
                    }
                    ')' => {
                        if first_parenthese && parentheses_number == 1 && !last_parenthese {
                            index_last_closing_paranthesis = index;
                            last_parenthese = true;
                        }
                        parentheses_number -= 1;
                    }
                    _ => {}
                }
            }
            if index_last_closing_paranthesis == 0 {
                return;
            }
            let mut modified_expression = expression.clone();
            modified_expression.remove(index_first_opening_paranthesis);
            modified_expression.remove(index_last_closing_paranthesis - 1);
            match (
                Self::get_current_operation(expression),
                Self::get_current_operation(&modified_expression),
            ) {
                (Ok(operation1), Ok(operation2)) => {
                    if operation1 == operation2 {
                        *expression = modified_expression.clone();
                    } else {
                        return;
                    }
                }
                _ => {}
            }
            println!("{expression} <- extra parentheses removed");
        }
    }

    fn is_negative(expression: &String, index: usize) -> bool {
        //let temp_string = &expression[0..index];
        let mut negative_flag = false;
        for chr in expression.chars().skip(index + 1) {
            match chr {
                '0'..='9' => {
                    if negative_flag {
                        return true;
                    }
                }
                '+' | '*' | '/' | '^' | '(' => return false,
                '-' => negative_flag = true,
                ')' => {
                    if negative_flag {
                        return true;
                    }
                }
                _ => continue,
            }
        }
        false
    }

    fn is_answer(expression: &String) -> bool {
        let mut negative_flag = false;
        for (index, chr) in expression.chars().enumerate() {
            match chr {
                '+' | '*' | '/' | '^' => return false,
                '-' => {
                    if !negative_flag {
                        return false;
                    }
                }
                '(' => {
                    if Self::is_negative(expression, index) {
                        negative_flag = true;
                    }
                }
                ')' => {
                    if negative_flag {
                        negative_flag = false;
                    }
                }
                _ => continue,
            }
        }
        return true;
    }

    fn get_args(expression: &String, index: usize) -> (Argument, Argument) {
        let temp_string = &expression[0..index];
        let mut arg1 = Argument::new();
        let mut arg2 = Argument::new();
        let mut negative_flag = false;

        //arg1
        let mut value = String::new();
        for chr in temp_string.chars().rev() {
            match chr {
                '0'..='9' | '.' => value.push(chr),
                ')' => negative_flag = true,
                '-' => {
                    if negative_flag {
                        value.push(chr);
                    } else {
                        break;
                    }
                }
                '(' => {
                    if negative_flag {
                        negative_flag = false;
                        arg1.negative = true;
                    } else {
                        arg1.next_to_parenthesis = true;
                    }
                }
                _ => break,
            }
        }
        value.reverse();
        arg1.arg = value;

        //arg2
        let mut value = String::new();
        for chr in expression.chars().skip(index + 1) {
            match chr {
                '0'..='9' | '.' => value.push(chr),
                '(' => negative_flag = true,
                '-' => {
                    if negative_flag {
                        value.push(chr);
                    } else {
                        break;
                    }
                }
                ')' => {
                    if negative_flag {
                        negative_flag = false;
                        arg2.negative = true;
                    } else {
                        arg2.next_to_parenthesis = true;
                        break;
                    }
                }
                _ => break,
            }
        }
        arg2.arg = value;
        (arg1, arg2)
    }

    pub fn solve_expression(mut expression: String) -> Result<String, String> {
        expression = expression.trim().to_string();
        match Self::check_expression(&expression) {
            Ok(()) => {}
            Err(e) => return Err(e),
        }
        Self::check_for_extra_parentheses(&mut expression);
        loop {
            match Self::get_current_operation(&expression) {
                Ok(expression_type) => {
                    let (result, range_start, range_stop) =
                        expression_type.current_operation.calculate();
                    expression.replace_range(range_start..range_stop, &result);
                    if Self::is_answer(&expression) {
                        break;
                    }
                    println!("{} <- ", &expression);
                }
                Err(e) => return Err(e),
            }
        }
        let expression = expression.remove_parentheses();
        Ok(expression)
    }
}

#[derive(PartialEq, Clone)]
struct Argument {
    arg: String,
    next_to_parenthesis: bool,
    negative: bool,
}

impl Argument {
    fn new() -> Argument {
        Argument {
            arg: String::from(""),
            next_to_parenthesis: false,
            negative: false,
        }
    }
}

//Parentheses скобки

#[derive(Clone)]
enum Operation {
    Addition {
        index: usize,
        arg1: Argument,
        arg2: Argument,
    },
    Subtraction {
        index: usize,
        arg1: Argument,
        arg2: Argument,
    },
    Multiplication {
        index: usize,
        arg1: Argument,
        arg2: Argument,
    },
    Division {
        index: usize,
        arg1: Argument,
        arg2: Argument,
    },
    Exponentiation {
        index: usize,
        arg1: Argument,
        arg2: Argument,
    },
}

impl Operation {
    fn get_operator_from_chr(
        chr: char,
        index: usize,
        arg1: Argument,
        arg2: Argument,
    ) -> Result<Operation, String> {
        match chr {
            '+' => return Ok(Self::Addition { index, arg1, arg2 }),
            '-' => return Ok(Self::Subtraction { index, arg1, arg2 }),
            '*' => return Ok(Self::Multiplication { index, arg1, arg2 }),
            '/' => return Ok(Self::Division { index, arg1, arg2 }),
            '^' => return Ok(Self::Exponentiation { index, arg1, arg2 }),
            '0'..='9' => return Err(String::from(format!("{chr} is digit"))),
            _ => return Err(String::from("unknown operator")),
        }
    }

    fn calculate(&self) -> (String, usize, usize) {
        match self {
            Operation::Addition { index, arg1, arg2 } => {
                let mut result: String;
                if arg1.arg.is_float() || arg2.arg.is_float() {
                    let arg1 = arg1.arg.trim().parse::<f64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<f64>().expect("smth wrong");
                    result = (arg1 + arg2).to_string();
                } else if arg1.negative || arg2.negative {
                    let arg1 = arg1.arg.trim().parse::<i64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<i64>().expect("smth wrong");
                    result = (arg1 + arg2).to_string();
                } else {
                    let arg1 = arg1.arg.trim().parse::<u64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<u64>().expect("smth wrong");
                    result = (arg1 + arg2).to_string();
                }
                if &result[0..1] == "-" {
                    result.insert(0, '(');
                    result.push(')');
                }

                let arg1_len = arg1.arg.len();
                let arg2_len = arg2.arg.len();

                let mut range_start = index - arg1_len;
                let mut range_stop: usize = index + arg2_len + 1;

                if arg1.next_to_parenthesis && arg2.next_to_parenthesis {
                    range_start -= 1;
                    range_stop += 1;
                }

                if arg1.negative {
                    range_start -= 2;
                }

                if arg2.negative {
                    range_stop += 2;
                }
                return (result, range_start, range_stop);
            }
            Operation::Subtraction { index, arg1, arg2 } => {
                let mut result: String;
                if arg1.arg.is_float() || arg2.arg.is_float() {
                    let arg1 = arg1.arg.trim().parse::<f64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<f64>().expect("smth wrong");
                    result = (arg1 - arg2).to_string();
                } else if arg1.negative || arg2.negative {
                    let arg1 = arg1.arg.trim().parse::<i64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<i64>().expect("smth wrong");
                    result = (arg1 - arg2).to_string();
                } else {
                    if arg1.arg >= arg2.arg {
                        let arg1 = arg1.arg.trim().parse::<u64>().expect("smth wrong");
                        let arg2 = arg2.arg.trim().parse::<u64>().expect("smth wrong");
                        result = (arg1 - arg2).to_string();
                    } else {
                        let arg1 = arg1.arg.trim().parse::<i64>().expect("smth wrong");
                        let arg2 = arg2.arg.trim().parse::<i64>().expect("smth wrong");
                        result = (arg1 - arg2).to_string();
                    }
                }
                if &result[0..1] == "-" {
                    result.insert(0, '(');
                    result.push(')');
                }

                let arg1_len = arg1.arg.len();
                let arg2_len = arg2.arg.len();

                let mut range_start = index - arg1_len;
                let mut range_stop: usize = index + arg2_len + 1;

                if arg1.next_to_parenthesis && arg2.next_to_parenthesis {
                    range_start -= 1;
                    range_stop += 1;
                }

                if arg1.negative {
                    range_start -= 2;
                }

                if arg2.negative {
                    range_stop += 2;
                }
                return (result, range_start, range_stop);
            }
            Operation::Multiplication { index, arg1, arg2 } => {
                let mut result: String;
                if arg1.arg.is_float() || arg2.arg.is_float() {
                    let arg1 = arg1.arg.trim().parse::<f64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<f64>().expect("smth wrong");
                    result = (arg1 * arg2).to_string();
                } else if arg1.negative || arg2.negative {
                    let arg1 = arg1.arg.trim().parse::<i64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<i64>().expect("smth wrong");
                    result = (arg1 * arg2).to_string();
                } else {
                    let arg1 = arg1.arg.trim().parse::<u64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<u64>().expect("smth wrong");
                    result = (arg1 * arg2).to_string();
                }
                if &result[0..1] == "-" {
                    result.insert(0, '(');
                    result.push(')');
                }

                let arg1_len = arg1.arg.len();
                let arg2_len = arg2.arg.len();

                let mut range_start = index - arg1_len;
                let mut range_stop: usize = index + arg2_len + 1;

                if arg1.next_to_parenthesis && arg2.next_to_parenthesis {
                    range_start -= 1;
                    range_stop += 1;
                }

                if arg1.negative {
                    range_start -= 2;
                }

                if arg2.negative {
                    range_stop += 2;
                }
                return (result, range_start, range_stop);
            }
            Operation::Division { index, arg1, arg2 } => {
                let mut result: String;
                if arg1.arg.is_float() || arg2.arg.is_float() {
                    let arg1 = arg1.arg.trim().parse::<f64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<f64>().expect("smth wrong");
                    result = (arg1 / arg2).to_string();
                } else if arg1.negative || arg2.negative {
                    let arg1 = arg1.arg.trim().parse::<i64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<i64>().expect("smth wrong");
                    if arg1 % arg2 == 0 {
                        result = (arg1 / arg2).to_string();
                    } else {
                        let arg1: f64 = arg1 as f64;
                        let arg2: f64 = arg2 as f64;
                        result = (arg1 / arg2).to_string();
                    }
                } else {
                    let arg1 = arg1.arg.trim().parse::<u64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<u64>().expect("smth wrong");
                    if arg1 % arg2 == 0 {
                        result = (arg1 / arg2).to_string();
                    } else {
                        let arg1: f64 = arg1 as f64;
                        let arg2: f64 = arg2 as f64;
                        result = (arg1 / arg2).to_string();
                    }
                }
                if &result[0..1] == "-" {
                    result.insert(0, '(');
                    result.push(')');
                }

                let arg1_len = arg1.arg.len();
                let arg2_len = arg2.arg.len();

                let mut range_start = index - arg1_len;
                let mut range_stop: usize = index + arg2_len + 1;

                if arg1.next_to_parenthesis && arg2.next_to_parenthesis {
                    range_start -= 1;
                    range_stop += 1;
                }

                if arg1.negative {
                    range_start -= 2;
                }

                if arg2.negative {
                    range_stop += 2;
                }
                return (result, range_start, range_stop);
            }
            Operation::Exponentiation { index, arg1, arg2 } => {
                let mut result: String;
                if arg1.arg.is_float() || arg2.arg.is_float() {
                    let arg1 = arg1.arg.trim().parse::<f64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<f64>().expect("smth wrong");
                    result = (arg1.powf(arg2)).to_string();
                } else if arg1.negative {
                    if arg2.negative {
                        let arg1 = arg1.arg.trim().parse::<f64>().expect("smth wrong");
                        let arg2 = arg2.arg.trim().parse::<f64>().expect("smth wrong");
                        result = (arg1.powf(arg2)).to_string();
                    } else {
                        let arg1 = arg1.arg.trim().parse::<i64>().expect("smth wrong");
                        let arg2 = arg2.arg.trim().parse::<u32>().expect("smth wrong");
                        result = (arg1.pow(arg2)).to_string();
                    }
                } else if !arg1.negative {
                    if arg2.negative {
                        let arg1 = arg1.arg.trim().parse::<f64>().expect("smth wrong");
                        let arg2 = arg2.arg.trim().parse::<f64>().expect("smth wrong");
                        result = (arg1.powf(arg2)).to_string();
                    } else {
                        let arg1 = arg1.arg.trim().parse::<i64>().expect("smth wrong");
                        let arg2 = arg2.arg.trim().parse::<u32>().expect("smth wrong");
                        result = (arg1.pow(arg2)).to_string();
                    }
                } else {
                    let arg1 = arg1.arg.trim().parse::<u64>().expect("smth wrong");
                    let arg2 = arg2.arg.trim().parse::<u32>().expect("smth wrong");
                    result = (arg1.pow(arg2)).to_string();
                }
                if &result[0..1] == "-" {
                    result.insert(0, '(');
                    result.push(')');
                }

                let arg1_len = arg1.arg.len();
                let arg2_len = arg2.arg.len();

                let mut range_start = index - arg1_len;
                let mut range_stop: usize = index + arg2_len + 1;

                if arg1.next_to_parenthesis && arg2.next_to_parenthesis {
                    range_start -= 1;
                    range_stop += 1;
                }

                if arg1.negative {
                    range_start -= 2;
                }

                if arg2.negative {
                    range_stop += 2;
                }
                return (result, range_start, range_stop);
            }
        }
    }
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        use Operation::*;
        match (self, other) {
            (
                Addition {
                    index: _i1,
                    arg1: a1,
                    arg2: a2,
                },
                Addition {
                    index: _i2,
                    arg1: b1,
                    arg2: b2,
                },
            ) => (a1.arg == b1.arg) && (a2.arg == b2.arg),
            (
                Subtraction {
                    index: _i1,
                    arg1: a1,
                    arg2: a2,
                },
                Subtraction {
                    index: _i2,
                    arg1: b1,
                    arg2: b2,
                },
            ) => (a1.arg == b1.arg) && (a2.arg == b2.arg),
            (
                Multiplication {
                    index: _i1,
                    arg1: a1,
                    arg2: a2,
                },
                Multiplication {
                    index: _i2,
                    arg1: b1,
                    arg2: b2,
                },
            ) => (a1.arg == b1.arg) && (a2.arg == b2.arg),
            (
                Division {
                    index: _i1,
                    arg1: a1,
                    arg2: a2,
                },
                Division {
                    index: _i2,
                    arg1: b1,
                    arg2: b2,
                },
            ) => (a1.arg == b1.arg) && (a2.arg == b2.arg),
            (
                Exponentiation {
                    index: _i1,
                    arg1: a1,
                    arg2: a2,
                },
                Exponentiation {
                    index: _i2,
                    arg1: b1,
                    arg2: b2,
                },
            ) => (a1.arg == b1.arg) && (a2.arg == b2.arg),
            _ => false,
        }
    }
}

impl ToString for Operation {
    #[allow(unused_variables)]
    fn to_string(&self) -> String {
        use Operation::*;
        match self {
            Addition { index, arg1, arg2 } => String::from("Addition"),
            Subtraction { index, arg1, arg2 } => String::from("Subtraction"),
            Multiplication { index, arg1, arg2 } => String::from("Multiplication"),
            Division { index, arg1, arg2 } => String::from("Division"),
            Exponentiation { index, arg1, arg2 } => String::from("Exponentiation"),
        }
    }
}
