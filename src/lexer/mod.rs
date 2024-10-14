pub struct Lexer<'a> {
    raw_nenyr: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(raw_nenyr: &'a str) -> Self {
        Self {
            raw_nenyr,
            position: 0,
            line: 1,
            column: 1,
        }
    }
}
