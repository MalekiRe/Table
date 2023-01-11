use crate::compiler::FileHolder;
use crate::compiler::parser::error::ErrorT;
use crate::compiler::parser::lexer::{lex, lexer, Token};
use crate::print_errors;



#[derive(Debug, PartialEq, Clone)]
pub enum Action<T> {
    /// If next iteration returns None, return T without advancing
    /// the cursor.
    Request(T),

    /// If the next iteration returns None, return None without advancing
    /// the cursor.
    Require,

    /// Immediately advance the cursor and return T.
    Return(T),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// An unexpected end-of-line has been found.
    EndOfLine,

    /// A syntax error at the indicated cursor position has been found.
    Character(usize),
}

#[derive(Debug)]
pub struct Scanner {
    cursor: usize,
    characters: Vec<Token>,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        let tokens = match lex(string) {
            (opt_tokens, errors) => {
                if errors.len() > 0 {
                    print_errors(errors, FileHolder::from(string.to_string()));
                }
                opt_tokens.expect("expected error to be thrown if no tokens were generated")
            }
        };
        Scanner {
            cursor: 0,
            characters: tokens.into_iter().map(|(token, span)| token).collect(),
        }
    }

    /// Returns the current cursor. Useful for reporting errors.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns the next Token without advancing the cursor.
    /// AKA "lookahead"
    pub fn peek(&self) -> Option<&Token> {
        self.characters.get(self.cursor)
    }

    /// Returns true if further progress is not possible.
    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    /// Returns the next Token (if available) and advances the cursor.
    pub fn pop(&mut self) -> Option<&Token> {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;

                Some(character)
            }
            None => None,
        }
    }

    /// Returns true if the `target` is found at the current cursor position,
    /// and advances the cursor.
    /// Otherwise, returns false leaving the cursor unchanged.
    pub fn take(&mut self, target: &Token) -> bool {
        match self.characters.get(self.cursor) {
            Some(character) => {
                if target == character {
                    self.cursor += 1;

                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Iteratively directs the advancement of the cursor and the return
    /// of translated values.
    pub fn scan<T>(
        &mut self,
        cb: impl Fn(&Vec<Token>) -> Option<Action<T>>,
    ) -> Result<Option<T>, Error> {
        let mut sequence = Vec::new();
        let mut require = false;
        let mut request = None;

        loop {
            match self.characters.get(self.cursor) {
                Some(target) => {
                    sequence.push(target.clone());

                    match cb(&sequence) {
                        Some(Action::Return(result)) => {
                            self.cursor += 1;

                            break Ok(Some(result));
                        }
                        Some(Action::Request(result)) => {
                            self.cursor += 1;
                            require = false;
                            request = Some(result);
                        }
                        Some(Action::Require) => {
                            self.cursor += 1;
                            require = true;
                        }
                        None => {
                            if require {
                                break Err(Error::Character(self.cursor));
                            } else {
                                break Ok(request);
                            }
                        }
                    }
                }
                None => {
                    if require {
                        break Err(Error::EndOfLine);
                    } else {
                        break Ok(request);
                    }
                }
            }
        }
    }
    /// Invoke `cb` once. If the result is not `None`, return it and advance
    /// the cursor. Otherwise, return None and leave the cursor unchanged.
    pub fn transform<T>(
        &mut self,
        cb: impl FnOnce(&Token) -> Option<T>,
    ) -> Option<T> {
        match self.characters.get(self.cursor) {
            Some(input) => match cb(input) {
                Some(output) => {
                    self.cursor += 1;

                    Some(output)
                }
                None => None,
            },
            None => None,
        }
    }
}