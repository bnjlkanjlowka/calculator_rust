pub trait MyStringExtension {
    fn is_float(&self) -> bool;
    fn reverse(&mut self);
    fn is_negative_opening_parenthesis(&self, index_parenthesis: usize) -> bool;
}

impl MyStringExtension for String {
    fn is_float(&self) -> bool {
        for chr in self.chars() {
            if chr == '.' {
                return true;
            }
        }
        false
    }

    fn reverse(&mut self) {
        *self = self.chars().rev().collect::<String>();
    }

    fn is_negative_opening_parenthesis(&self, index_parenthesis: usize) -> bool {
        let mut negative_flag = false;
        for chr in self.chars().skip(index_parenthesis + 1) {
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
}
