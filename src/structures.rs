mod my_extension;

use my_extension::MyStringExtension;

struct VecOperators(Vec<Symbol>, usize);

impl VecOperators {
    fn sort(&mut self) -> Vec<Symbol> {
        use Operation::*;
        use Symbol::*;
        let operators_matrix = &mut self.0;
        let max_parentheses_number = self.1 as isize;
        let mut sotred_operators_matrix: Vec<Symbol> = Vec::new();
        for parenthesis_number in (0..=max_parentheses_number).rev() {
            let mut all_exponentiation = false;
            let mut all_multi_divis = false;
            loop {
                for (index, operator) in operators_matrix.iter().enumerate() {
                    match operator {
                        Operator {
                            operation,
                            parenthese_number,
                        } if *parenthese_number == parenthesis_number => match operation {
                            Exponentiation { .. } => {
                                sotred_operators_matrix.push(operators_matrix.remove(index));
                                break;
                            }
                            Multiplication { .. } | Division { .. } if all_exponentiation => {
                                sotred_operators_matrix.push(operators_matrix.remove(index));
                                break;
                            }
                            Addition { .. } | Subtraction { .. }
                                if all_exponentiation && all_multi_divis =>
                            {
                                sotred_operators_matrix.push(operators_matrix.remove(index));
                                break;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                if !all_exponentiation {
                    all_exponentiation = true;
                } else if all_exponentiation && !all_multi_divis {
                    all_multi_divis = true;
                } else if all_exponentiation && all_multi_divis {
                    break;
                }
            }
        }
        sotred_operators_matrix
    }

    fn get_operators_matrix(expression: &VecSymbols) -> VecOperators {
        use Symbol::*;
        let expression = &expression.0;
        let mut operators_matrix: Vec<Symbol> = Vec::new();
        let mut max_parentheses_number = 0;
        for symbol in expression.iter() {
            match symbol {
                Operator {
                    parenthese_number, ..
                } => {
                    operators_matrix.push(symbol.clone());
                    if *parenthese_number > max_parentheses_number {
                        max_parentheses_number = *parenthese_number;
                    }
                }
                _ => continue,
            }
        }
        VecOperators(operators_matrix, max_parentheses_number as usize)
    }
}

#[derive(Clone)]
struct VecSymbols(Vec<Symbol>);

impl VecSymbols {
    fn string_to_symbol(string: String) -> Result<VecSymbols, String> {
        let mut vector_symbols: Vec<Symbol> = Vec::new();
        let mut parenthese_number: isize = 0;
        let mut negative_flag = false;
        let string = string.trim().to_string();

        for (index, chr) in string.chars().enumerate() {
            match chr {
                '0'..='9' | '.' => vector_symbols.push(Symbol::Digit { chr }),
                '+' => vector_symbols.push(Symbol::Operator {
                    operation: Operation::Addition {
                        index,
                        arg1: Argument::new(),
                        arg2: Argument::new(),
                    },
                    parenthese_number,
                }),
                '-' => {
                    if negative_flag {
                        vector_symbols.push(Symbol::Digit { chr });
                    } else {
                        vector_symbols.push(Symbol::Operator {
                            operation: Operation::Subtraction {
                                index,
                                arg1: Argument::new(),
                                arg2: Argument::new(),
                            },
                            parenthese_number,
                        })
                    }
                }
                '*' => vector_symbols.push(Symbol::Operator {
                    operation: Operation::Multiplication {
                        index,
                        arg1: Argument::new(),
                        arg2: Argument::new(),
                    },
                    parenthese_number,
                }),
                '/' => vector_symbols.push(Symbol::Operator {
                    operation: Operation::Division {
                        index,
                        arg1: Argument::new(),
                        arg2: Argument::new(),
                    },
                    parenthese_number,
                }),
                '^' => vector_symbols.push(Symbol::Operator {
                    operation: Operation::Exponentiation {
                        index,
                        arg1: Argument::new(),
                        arg2: Argument::new(),
                    },
                    parenthese_number,
                }),
                '(' => {
                    if string.is_negative_opening_parenthesis(index) {
                        negative_flag = true;
                    } else {
                        parenthese_number += 1;
                    }
                    vector_symbols.push(Symbol::OpeningParenthesis {
                        for_negative: negative_flag,
                        parenthesis_number: parenthese_number,
                        checked: false,
                    });
                }
                ')' => {
                    vector_symbols.push(Symbol::ClosingParenthesis {
                        for_negative: negative_flag,
                        parenthesis_number: parenthese_number,
                        checked: false,
                    });
                    if negative_flag {
                        negative_flag = false;
                    } else {
                        parenthese_number -= 1;
                    }
                }
                _ => return Err(format!("unknown symbol:{chr}, index:{index}")),
            }
        }
        Ok(VecSymbols(vector_symbols))
    }

    fn is_number(&self) -> bool {
        use Symbol::*;

        let expression = &self.0;
        for symbol in expression.iter() {
            match symbol {
                Operator { .. } => return false,
                OpeningParenthesis { for_negative, .. } => {
                    if !for_negative {
                        return false;
                    }
                }
                _ => {}
            }
        }
        return true;
    }

    fn check_extra_parentheses(&mut self) {
        use Symbol::*;
        loop {
            let mut check_parenthesis_number: isize = 0;
            let mut opening_parenthesis_index = 0;
            let mut closing_parenthesis_index = 0;
            for (index, symbol) in self.0.iter().enumerate() {
                match symbol {
                    OpeningParenthesis {
                        for_negative,
                        parenthesis_number,
                        checked,
                    } if !for_negative && !checked => {
                        check_parenthesis_number = *parenthesis_number;
                        opening_parenthesis_index = index;
                    }
                    ClosingParenthesis {
                        for_negative,
                        parenthesis_number,
                        checked,
                    } if !for_negative
                        && !checked
                        && *parenthesis_number == check_parenthesis_number =>
                    {
                        closing_parenthesis_index = index;
                        break;
                    }
                    _ => {}
                }
            }
            if closing_parenthesis_index == 0 {
                break;
            }
            let mut modify_expression = self.clone();
            modify_expression.0.remove(closing_parenthesis_index);
            modify_expression.0.remove(opening_parenthesis_index);
            let modify_expression = modify_expression.to_string();
            let modify_expression =
                VecSymbols::string_to_symbol(modify_expression).expect("just delete parentheses");
            if VecOperators::get_operators_matrix(self).sort()
                == VecOperators::get_operators_matrix(&modify_expression).sort()
            {
                *self = modify_expression;
                println!("{} <- remove extra parentheses", self);
            } else {
                match &mut self.0[opening_parenthesis_index] {
                    OpeningParenthesis { checked, .. } => *checked = true,
                    _ => panic!("big problem with parentheses index"),
                }
                match &mut self.0[closing_parenthesis_index] {
                    ClosingParenthesis { checked, .. } => *checked = true,
                    _ => panic!("big problem with parentheses index"),
                }
            }
        }
    }

    fn check_expression(&self) -> Result<(), String> {
        use Operation::*;
        use Symbol::*;
        if self.is_number() {
            return Err(String::from("Its just a number"));
        }

        let mut parenthesis_number: usize = 0;
        for symbol in self.0.iter() {
            match symbol {
                Operator {
                    operation,
                    parenthese_number: _,
                } => match operation {
                    Addition { index, .. }
                    | Subtraction { index, .. }
                    | Multiplication { index, .. }
                    | Division { index, .. }
                    | Exponentiation { index, .. } => match self.check_operator(*index) {
                        Ok(()) => continue,
                        Err(e) => return Err(e),
                    },
                    _ => return Err(String::from("unknown operator")),
                },
                OpeningParenthesis { for_negative, .. } if !for_negative => parenthesis_number += 1,
                ClosingParenthesis { for_negative, .. } if !for_negative => {
                    if parenthesis_number == 0 {
                        return Err(String::from("Extra ')'"));
                    } else {
                        parenthesis_number -= 1;
                    }
                }
                _ => continue,
            }
        }
        if parenthesis_number != 0 {
            return Err(String::from("Extra '('"));
        }
        Ok(())
    }

    fn check_operator(&self, index_operator: usize) -> Result<(), String> {
        match (
            self.check_left_arg(index_operator),
            self.check_right_arg(index_operator),
        ) {
            (Ok(()), Ok(())) => return Ok(()),
            (Ok(()), Err(e)) => return Err(e),
            (Err(e), Ok(())) => return Err(e),
            (Err(e1), Err(e2)) => return Err(format!("{} {}", e1, e2)),
        }
    }

    fn check_right_arg(&self, index_operator: usize) -> Result<(), String> {
        use Symbol::*;

        if index_operator == self.0.len() - 1 {
            return Err(format!(
                "incorrect expression. chr:{}, index:{index_operator}",
                self.0[index_operator]
            ));
        }

        for (index, symbol) in self.0.iter().skip(index_operator + 1).enumerate() {
            let index = index_operator + index + 1;
            match symbol {
                Digit { chr: _ } => return Ok(()),
                OpeningParenthesis { for_negative, .. } => {
                    if !*for_negative {
                        continue;
                    } else {
                        return Ok(());
                    }
                }
                _ => return Err(format!("incorrect expression. chr:{symbol}, index:{index}")),
            }
        }
        Ok(())
    }

    fn check_left_arg(&self, index_operator: usize) -> Result<(), String> {
        use Symbol::*;

        if index_operator == 0 {
            return Err(format!(
                "incorrect expression. chr:{} index:{index_operator}",
                self.0[0]
            ));
        }

        let skip = self.0.len() - index_operator;
        for (index, symbol) in self.0.iter().rev().skip(skip).enumerate() {
            let index = index_operator - index - 1;
            match symbol {
                Digit { chr: _ } => return Ok(()),
                ClosingParenthesis { for_negative, .. } => {
                    if !*for_negative {
                        continue;
                    } else {
                        return Ok(());
                    }
                }
                _ => return Err(format!("incorrect expression. chr:{symbol}, index:{index}")),
            }
        }
        Ok(())
    }

    fn get_current_operation(&self) -> Operation {
        let mut max_parenthese_number: isize = 0;
        let mut current_operation = Operation::Zero;
        let mut current_operation_index = 0;

        for (index, symbol) in self.0.iter().enumerate() {
            match symbol {
                Symbol::Operator {
                    operation,
                    parenthese_number,
                } => {
                    if parenthese_number > &max_parenthese_number
                        || (operation > &current_operation
                            && parenthese_number == &max_parenthese_number)
                    {
                        current_operation = operation.clone();
                        max_parenthese_number = *parenthese_number;
                        current_operation_index = index;
                    }
                }
                _ => {}
            }
        }
        current_operation.get_args(self, current_operation_index);
        current_operation.change_index(current_operation_index);
        current_operation
    }
}

impl std::fmt::Display for VecSymbols {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for symbol in self.0.iter() {
            write!(f, "{}", symbol).unwrap();
        }
        Ok(())
    }
}

pub struct Expression {
    expression: VecSymbols,
    current_operation: Operation,
}

impl Expression {
    fn new(expression: String) -> Result<Expression, String> {
        let mut vector_symbols = VecSymbols::string_to_symbol(expression)?;
        vector_symbols.check_expression()?;
        vector_symbols.check_extra_parentheses();
        let current_operation = vector_symbols.get_current_operation();
        Ok(Expression {
            expression: vector_symbols,
            current_operation,
        })
    }

    fn is_negative_result(&self) -> bool {
        use Symbol::*;

        for symbol in self.expression.0.iter() {
            match symbol {
                Digit { chr } if *chr == '-' => return true,
                _ => continue,
            }
        }
        false
    }

    pub fn solve_expression(expression: String) -> Result<String, String> {
        let mut expression = Self::new(expression)?;
        loop {
            let (result, start, stop) = expression.current_operation.calculate();
            let result = VecSymbols::string_to_symbol(result).expect("relust wrong");
            expression.expression.0.splice(start..stop, result.0);
            if expression.expression.is_number() {
                if expression.is_negative_result() {
                    expression
                        .expression
                        .0
                        .remove(expression.expression.0.len() - 1);
                    expression.expression.0.remove(0);
                }
                break;
            } else {
                println!("{}", expression.expression);
                expression.current_operation = expression.expression.get_current_operation();
            }
        }
        Ok(format!("{}", expression.expression))
    }
}

#[derive(Clone)]
enum Operation {
    Zero,
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
    fn calculate(&self) -> (String, usize, usize) {
        use Operation::*;

        match self {
            Addition { index, arg1, arg2 } => {
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
            Subtraction { index, arg1, arg2 } => {
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
            Multiplication { index, arg1, arg2 } => {
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
            Division { index, arg1, arg2 } => {
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
            Exponentiation { index, arg1, arg2 } => {
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
            _ => (String::from("lol"), 1, 1),
        }
    }

    fn get_left_arg(expression: &VecSymbols, index_operator: &usize) -> Argument {
        use Symbol::*;

        let expression = &expression.0;
        let skip = expression.len() - index_operator;
        let mut arg1 = String::new();

        let mut next_to_parenthesis = false;
        let mut negative = false;

        for symbol in expression.iter().rev().skip(skip) {
            match symbol {
                Digit { chr } => arg1.push(*chr),
                OpeningParenthesis { for_negative, .. } => {
                    if *for_negative {
                        negative = true;
                    } else {
                        next_to_parenthesis = true;
                        break;
                    }
                }
                ClosingParenthesis { for_negative, .. } => {
                    if *for_negative {
                        negative = true;
                    }
                }
                _ => break,
            }
        }
        arg1.reverse();
        Argument {
            arg: arg1,
            next_to_parenthesis,
            negative,
        }
    }

    fn get_right_arg(expression: &VecSymbols, index_operator: &usize) -> Argument {
        use Symbol::*;
        let expression = &expression.0;
        let mut arg2 = String::new();
        let mut negative = false;
        let mut next_to_parenthesis = false;
        for symbol in expression.iter().skip(index_operator + 1) {
            match symbol {
                Digit { chr } => arg2.push(*chr),
                ClosingParenthesis { for_negative, .. } => {
                    if *for_negative {
                        negative = true;
                    } else {
                        next_to_parenthesis = true;
                        break;
                    }
                }
                OpeningParenthesis { for_negative, .. } if *for_negative => negative = true,
                _ => break,
            }
        }
        Argument {
            arg: arg2,
            next_to_parenthesis,
            negative,
        }
    }

    fn get_args(&mut self, expression: &VecSymbols, index_operator: usize) {
        use Operation::*;

        match self {
            Addition { arg1, arg2, .. }
            | Subtraction { arg1, arg2, .. }
            | Multiplication { arg1, arg2, .. }
            | Division { arg1, arg2, .. }
            | Exponentiation { arg1, arg2, .. } => {
                *arg1 = Self::get_left_arg(expression, &index_operator);
                *arg2 = Self::get_right_arg(expression, &index_operator);
            }
            _ => {}
        }
    }

    fn change_index(&mut self, index_operator: usize) {
        use Operation::*;

        match self {
            Addition { index, .. }
            | Subtraction { index, .. }
            | Multiplication { index, .. }
            | Division { index, .. }
            | Exponentiation { index, .. } => {
                *index = index_operator;
            }
            _ => {}
        }
    }
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        use Operation::*;
        match (self, other) {
            (
                Addition {
                    arg1: a1, arg2: b1, ..
                },
                Addition {
                    arg1: a2, arg2: b2, ..
                },
            ) => {
                if a1 == a2 && b1 == b2 {
                    true
                } else {
                    false
                }
            }
            (
                Subtraction {
                    arg1: a1, arg2: b1, ..
                },
                Subtraction {
                    arg1: a2, arg2: b2, ..
                },
            ) => {
                if a1 == a2 && b1 == b2 {
                    true
                } else {
                    false
                }
            }
            (
                Multiplication {
                    arg1: a1, arg2: b1, ..
                },
                Multiplication {
                    arg1: a2, arg2: b2, ..
                },
            ) => {
                if a1 == a2 && b1 == b2 {
                    true
                } else {
                    false
                }
            }
            (
                Division {
                    arg1: a1, arg2: b1, ..
                },
                Division {
                    arg1: a2, arg2: b2, ..
                },
            ) => {
                if a1 == a2 && b1 == b2 {
                    true
                } else {
                    false
                }
            }
            (
                Exponentiation {
                    arg1: a1, arg2: b1, ..
                },
                Exponentiation {
                    arg1: a2, arg2: b2, ..
                },
            ) => {
                if a1 == a2 && b1 == b2 {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl PartialOrd for Operation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        use Operation::*;
        match (self, other) {
            (
                Addition { .. } | Subtraction { .. },
                Multiplication { .. } | Division { .. } | Exponentiation { .. },
            ) => return Some(Less),
            (
                Multiplication { .. } | Division { .. } | Exponentiation { .. },
                Addition { .. } | Subtraction { .. },
            ) => Some(Greater),
            (Exponentiation { .. }, Multiplication { .. } | Division { .. }) => Some(Greater),
            (Multiplication { .. } | Division { .. }, Exponentiation { .. }) => Some(Less),
            (Zero, _) => Some(Less),
            (_, Zero) => Some(Greater),
            _ => Some(Equal),
        }
    }
}

#[derive(Clone)]
enum Symbol {
    Digit {
        chr: char,
    },
    Operator {
        operation: Operation,
        parenthese_number: isize,
    },
    OpeningParenthesis {
        for_negative: bool,
        parenthesis_number: isize,
        checked: bool,
    },
    ClosingParenthesis {
        for_negative: bool,
        parenthesis_number: isize,
        checked: bool,
    },
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        use Symbol::*;
        match (self, other) {
            (Digit { chr: c1 }, Digit { chr: c2 }) => c1 == c2,
            (Operator { operation: o1, .. }, Operator { operation: o2, .. }) => o1 == o2,
            (
                OpeningParenthesis {
                    for_negative: f1,
                    parenthesis_number: p1,
                    ..
                },
                OpeningParenthesis {
                    for_negative: f2,
                    parenthesis_number: p2,
                    ..
                },
            ) => f1 == f2 && p1 == p2,
            (
                ClosingParenthesis {
                    for_negative: f1,
                    parenthesis_number: p1,
                    ..
                },
                ClosingParenthesis {
                    for_negative: f2,
                    parenthesis_number: p2,
                    ..
                },
            ) => f1 == f2 && p1 == p2,
            _ => false,
        }
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operation::*;
        use Symbol::*;
        match self {
            Digit { chr } => write!(f, "{}", chr),
            Operator { operation, .. } => match operation {
                Addition { .. } => write!(f, "+"),
                Subtraction { .. } => write!(f, "-"),
                Multiplication { .. } => write!(f, "*"),
                Division { .. } => write!(f, "/"),
                Exponentiation { .. } => write!(f, "^"),
                _ => write!(f, ""),
            },
            OpeningParenthesis { .. } => write!(f, "("),
            ClosingParenthesis { .. } => write!(f, ")"),
        }
    }
}

#[derive(Clone)]
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

impl PartialEq for Argument {
    fn eq(&self, other: &Self) -> bool {
        if self.arg == other.arg {
            true
        } else {
            false
        }
    }
}
