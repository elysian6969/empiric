use core::str::CharIndices;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Lexme<'input> {
    Fn,
    ParenLeft,
    ParenRight,
    BraceLeft,
    BraceRight,
    DoubleColon,
    Ident(&'input str),
    Newline,
    Space(&'input str),
    Integer(u64),
    String(&'input str),
    Comma,
}

impl<'input> Lexme<'input> {
    pub fn display(&self) -> String {
        match self {
            Lexme::Fn => format!("\x1b[38;5;1mfn\x1b[m"),
            Lexme::ParenLeft => format!("("),
            Lexme::ParenRight => format!(")"),
            Lexme::BraceLeft => format!("{{"),
            Lexme::BraceRight => format!("}}"),
            Lexme::Ident(ident) => format!("\x1b[38;5;4m{ident}\x1b[m"),
            Lexme::Newline => format!("\n"),
            Lexme::Space(space) => format!("{space}"),
            Lexme::Integer(integer) => format!("\x1b[38;5;11m{integer}\x1b[m"),
            Lexme::DoubleColon => format!("::"),
            Lexme::String(string) => format!("\x1b[38;5;2m{string}\x1b[m"),
            Lexme::Comma => format!(","),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lexer<'input> {
    input: &'input str,
    chars: CharIndices<'input>,
    peek0: Option<(usize, char)>,
    peek1: Option<(usize, char)>,
}

impl<'input> Lexer<'input> {
    #[inline]
    pub fn new(input: &'input str) -> Self {
        let mut chars = input.char_indices();
        let peek0 = chars.next();
        let peek1 = chars.next();

        Lexer {
            input,
            chars,
            peek0,
            peek1,
        }
    }

    #[inline]
    fn step(&mut self) {
        self.peek0 = self.peek1;
        self.peek1 = self.chars.next();
    }

    #[inline]
    fn stepn(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    #[inline]
    fn peek(&self) -> Option<(usize, char)> {
        self.peek0
    }

    #[inline]
    fn peek2(&self) -> Option<(usize, char, char)> {
        self.peek0
            .and_then(|(start, char0)| self.peek1.map(|(_, char1)| (start, char0, char1)))
    }

    #[inline]
    fn ident(&mut self, start: usize) -> Option<Lexme<'input>> {
        let mut end = start;

        loop {
            match self.peek() {
                Some((_start, 'a'..='z' | 'A'..='Z' | '_')) => self.step(),
                Some((start, _char0)) => {
                    end = start;
                    break;
                }
                None => return None,
            }
        }

        let input = &self.input[start..end];
        let lexme = match input {
            "fn" => Lexme::Fn,
            ident => Lexme::Ident(ident),
        };

        Some(lexme)
    }

    #[inline]
    fn integer(&mut self, start: usize) -> Option<Lexme<'input>> {
        let mut end = start;

        loop {
            match self.peek() {
                Some((_start, '0'..='9')) => self.step(),
                Some((start, _char0)) => {
                    end = start;
                    break;
                }
                None => return None,
            }
        }

        let input = &self.input[start..end];

        Some(Lexme::Integer(input.parse::<u64>().unwrap()))
    }

    #[inline]
    fn space(&mut self, start: usize) -> Option<Lexme<'input>> {
        let mut end = start;

        loop {
            match self.peek() {
                Some((_start, char0)) if char0.is_whitespace() => self.step(),
                Some((start, _char0)) => {
                    end = start;
                    break;
                }
                None => return None,
            }
        }

        Some(Lexme::Space(&self.input[start..end]))
    }

    #[inline]
    fn string(&mut self, start: usize) -> Option<Lexme<'input>> {
        let mut end = start;

        loop {
            match self.peek() {
                Some((_start, char0)) if char0 != '\'' => self.step(),
                Some((_start, _char0)) => match self.peek() {
                    Some((start, '\'')) => {
                        end = start + 1;
                        self.step();
                        break;
                    }
                    _ => return None,
                },
                None => return None,
            }
        }

        Some(Lexme::String(&self.input[start..end]))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Lexme<'input>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let lexme = match self.peek2() {
            Some((_start, char0, char1)) => match (char0, char1) {
                (':', ':') => Some(Lexme::DoubleColon),
                _ => None,
            },
            _ => None,
        };

        if let Some(lexme) = lexme {
            self.stepn(2);

            return Some(lexme);
        }

        let lexme = match self.peek() {
            Some((start, char0)) => match char0 {
                ',' => Some(Lexme::Comma),
                '(' => Some(Lexme::ParenLeft),
                ')' => Some(Lexme::ParenRight),
                '{' => Some(Lexme::BraceLeft),
                '}' => Some(Lexme::BraceRight),
                '\n' => Some(Lexme::Newline),
                '\'' => {
                    self.step();
                    return self.string(start);
                }
                '0'..='9' => {
                    self.step();
                    return self.integer(start);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.step();
                    return self.ident(start);
                }
                char0 if char0.is_whitespace() => {
                    self.step();
                    return self.space(start);
                }
                _ => None,
            },
            _ => None,
        };

        if let Some(lexme) = lexme {
            self.step();

            return Some(lexme);
        }

        None
    }
}
