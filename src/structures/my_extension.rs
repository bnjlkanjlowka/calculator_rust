pub trait MyStringExtension {
    fn is_float(&self) -> bool;
    fn reverse(&mut self);
    fn remove_parentheses(self) -> String;
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

    fn remove_parentheses(self) -> String {
        let mut string = String::new();
        for chr in self.chars() {
            match chr {
                '(' | ')' => continue,
                _ => string.push(chr),
            }
        }
        string
    }
}
