use super::constants::LITERAL_TOKENS;
use super::types::LexItem;
use crate::lex::constants::is_identifier_char;
use crate::lex::errors::LexResult;
use crate::lex::errors::Location;
use crate::lex::iterator_util::SourceString;
use crate::lex::Lexer;
use std::char;
use std::iter::Iterator;

impl<It> Lexer<It>
where
    It: Iterator<Item = char>,
{
    pub fn new(src: It) -> Lexer<It> {
        Lexer {
            source: src.enumerate(),
            lookahead: Vec::new(),
            character: Location { character: 0 },
            start_char: Location { character: 0 },
        }
    }

    fn next_regular_token(&mut self) -> Option<LexResult> {
        let mut token = SourceString::new();
        token.push(self.next_after_whitespace()?);

        loop {
            let partial_matches: Vec<&(&str, LexItem)> = LITERAL_TOKENS
                .iter()
                .filter(|(key, _)| key.trim_end_matches('\x00').starts_with(&token.to_string()))
                .collect();

            let returning_match = partial_matches.len() < 2
                && if let Some((match_, _)) = partial_matches.first() {
                    token.to_string().starts_with(match_)
                } else {
                    true
                };

            if !returning_match {
                if let Some(ch) = self.next_char() {
                    token.push(ch);
                    continue;
                }
            }

            let largest_match = LITERAL_TOKENS
                .iter()
                .filter(|(key, _)| {
                    token.to_string().starts_with(key.trim_end_matches('\x00'))
                        && if key.ends_with('\x00') {
                            token
                                .to_string()
                                .trim_start_matches(key.trim_end_matches('\x00'))
                                .chars()
                                .next()
                                .map(|ch| !is_identifier_char(ch))
                                .unwrap_or(true)
                        } else {
                            true
                        }
                })
                .max_by_key(|(key, _)| key.len());
            return if let Some((key, value)) = largest_match {
                self.nextnt_string(token);
                self.next_chars(key.trim_end_matches('\x00').len()).unwrap();
                Some(Ok(self.ok_token(value.clone())))
            } else {
                self.nextnt_string(token);
                None
            };
        }
    }
}

impl<It> Iterator for Lexer<It>
where
    It: Iterator<Item = char>,
{
    type Item = LexResult;

    fn next(&mut self) -> Option<LexResult> {
        self.set_start_pos()?;

        Some(if let Some(token) = self.next_regular_token() {
            token
        } else {
            let ch = self.next_after_whitespace()?;
            match ch.ch {
                '"' => self.parse_string_literal(),
                '0' => self.parse_numeric_zero_literal(),
                '1'..='9' => {
                    self.nextnt(ch);
                    self.read_numeric_literal(10)
                }
                '\'' => self.parse_char_literal(),
                _ => self.parse_identifier(ch.ch),
            }
        })
    }
}
