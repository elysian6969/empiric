use super::lexer::{Lexer, Lexme};

#[derive(Clone, Debug)]
pub enum Argument {
    None,
    U64(u64),
    String(String),
}

impl From<Option<u64>> for Argument {
    #[inline]
    fn from(from: Option<u64>) -> Self {
        match from {
            Some(arg) => Argument::U64(arg),
            None => Argument::None,
        }
    }
}

impl From<Option<String>> for Argument {
    #[inline]
    fn from(from: Option<String>) -> Self {
        match from {
            Some(arg) => Argument::String(arg),
            None => Argument::None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Syscall {
    pub id: u64,
    pub arg0: Argument,
    pub arg1: Argument,
    pub arg2: Argument,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub ident: String,
    pub body: Vec<Syscall>,
}

impl Function {
    pub fn new(ident: String) -> Self {
        Self {
            ident,
            body: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Source {
    pub functions: Vec<Function>,
}

impl Source {
    pub fn new() -> Self {
        Self { functions: vec![] }
    }
}

#[derive(Clone, Debug)]
pub struct Parser<'input> {
    lexer: Lexer<'input>,
    lexme0: Option<Lexme<'input>>,
}

impl<'input> Parser<'input> {
    #[inline]
    pub fn new(input: &'input str) -> Self {
        let mut lexer = Lexer::new(input);
        let lexme0 = lexer.next();

        Self { lexer, lexme0 }
    }

    #[inline]
    fn step(&mut self) {
        self.lexme0 = self.lexer.next();
    }

    #[inline]
    fn peek(&self) -> Option<Lexme<'input>> {
        self.lexme0
    }

    #[inline]
    pub fn do_space(&mut self) {
        loop {
            match self.peek() {
                Some(Lexme::Space(_)) | Some(Lexme::Newline) => {
                    self.step();
                }
                _ => break,
            }
        }
    }

    #[inline]
    pub fn do_paren_left(&mut self) -> Option<()> {
        match self.peek() {
            Some(Lexme::ParenLeft) => {
                self.step();

                Some(())
            }
            _ => None,
        }
    }

    #[inline]
    pub fn do_paren_right(&mut self) -> Option<()> {
        match self.peek() {
            Some(Lexme::ParenRight) => {
                self.step();

                Some(())
            }
            _ => None,
        }
    }

    #[inline]
    pub fn do_brace_left(&mut self) -> Option<()> {
        match self.peek() {
            Some(Lexme::BraceLeft) => {
                self.step();

                Some(())
            }
            _ => None,
        }
    }

    #[inline]
    pub fn do_brace_right(&mut self) -> Option<()> {
        match self.peek() {
            Some(Lexme::BraceRight) => {
                self.step();

                Some(())
            }
            _ => None,
        }
    }

    #[inline]
    pub fn do_double_colon(&mut self) -> Option<()> {
        match self.peek() {
            Some(Lexme::DoubleColon) => {
                self.step();

                Some(())
            }
            _ => None,
        }
    }

    #[inline]
    pub fn do_comma(&mut self) -> Option<()> {
        match self.peek() {
            Some(Lexme::Comma) => {
                self.step();

                Some(())
            }
            _ => None,
        }
    }

    #[inline]
    pub fn do_ident(&mut self) -> Option<String> {
        match self.peek() {
            Some(Lexme::Ident(ident)) => {
                self.step();

                Some(ident.to_string())
            }
            _ => None,
        }
    }

    #[inline]
    pub fn do_string(&mut self) -> Option<String> {
        match self.peek() {
            Some(Lexme::String(string)) => {
                self.step();

                Some(string.to_string())
            }
            _ => None,
        }
    }

    #[inline]
    pub fn do_integer(&mut self) -> Option<u64> {
        match self.peek() {
            Some(Lexme::Integer(integer)) => {
                self.step();

                Some(integer)
            }
            _ => None,
        }
    }

    #[inline]
    pub fn do_argument(&mut self) -> Argument {
        match Argument::from(self.do_integer()) {
            Argument::None => Argument::from(self.do_string()),
            argument => argument,
        }
    }

    #[inline]
    pub fn do_path(&mut self) -> Vec<String> {
        let mut path = vec![];

        if self.do_double_colon().is_some() {
            path.push("::".to_string());
        }

        loop {
            if let Some(ident) = self.do_ident() {
                path.push(ident);
                self.do_double_colon();
            } else {
                break;
            }
        }

        path
    }

    #[inline]
    pub fn do_parameters(&mut self) -> Option<()> {
        self.do_space();
        self.do_paren_left()?;
        self.do_space();
        self.do_paren_right()?;

        Some(())
    }

    #[inline]
    pub fn do_body(&mut self) -> Vec<Syscall> {
        #[inline]
        pub fn do_body(parser: &mut Parser) -> Option<Vec<Syscall>> {
            parser.do_space();
            parser.do_brace_left()?;

            let mut syscalls = vec![];

            loop {
                parser.do_space();
                let path = parser.do_path();

                if path.is_empty() {
                    break;
                }

                if path.get(0) == Some(&"sys".to_string())
                    && path.get(1) == Some(&"syscall".to_string())
                {
                    parser.do_space();
                    parser.do_paren_left()?;

                    let id = parser.do_integer()?;

                    parser.do_space();
                    parser.do_comma();
                    parser.do_space();

                    let arg0 = parser.do_argument();

                    parser.do_space();
                    parser.do_comma();
                    parser.do_space();

                    let arg1 = parser.do_argument();

                    parser.do_space();
                    parser.do_comma();
                    parser.do_space();

                    let arg2 = parser.do_argument();

                    parser.do_space();
                    parser.do_comma();

                    syscalls.push(Syscall {
                        id,
                        arg0,
                        arg1,
                        arg2,
                    });

                    parser.do_space();
                    parser.do_paren_right()?;
                }
            }

            parser.do_space();
            parser.do_brace_right()?;

            Some(syscalls)
        }

        do_body(self).unwrap_or_else(Vec::new)
    }

    #[inline]
    pub fn do_fn(&mut self) -> Option<Function> {
        self.do_space();

        let ident = self.do_ident()?;

        self.do_parameters()?;
        let body = self.do_body();

        Some(Function { ident, body })
    }

    #[inline]
    pub fn parse(&mut self) -> Source {
        let mut source = Source::new();

        loop {
            self.do_space();

            match self.peek() {
                Some(Lexme::Fn) => {
                    self.step();

                    if let Some(function) = self.do_fn() {
                        source.functions.push(function);
                    }
                }
                None => break,
                _ => {
                    self.step();
                }
            }
        }

        source
    }
}
