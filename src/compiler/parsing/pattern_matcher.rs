use std::{iter::Enumerate, vec};

use super::tokenizer::{Token, TokenData};
#[derive(Debug)]
pub enum Match<'a> {
    IgnoreWhitespace,
    Whitespace,
    Of(&'a [TokenData]),
    OfType(&'a [TokenData]),
    Optional(&'a [Match<'a>]),
    PossibleCommaSeparated(&'a [Match<'a>]),
    Either(&'a [Match<'a>], &'a [Match<'a>]),

    Glob,
}
#[derive(Debug, PartialEq)]
pub enum EitherSide<A, B> {
    Left(A),
    Right(B),
}
impl<A, B> EitherSide<A, B> {
    #[inline]
    pub fn is_right(&self) -> bool {
        matches!(self, Self::Right(_))
    }
    #[inline]
    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left(_))
    }
}

#[derive(Debug, PartialEq)]
pub enum MatchResult {
    // We have no need of whitespace or Of as the results carry no data
    OfType(Vec<Token>),
    Optional(Option<Vec<MatchResult>>),
    PossibleCommaSeparated(Vec<Vec<MatchResult>>),
    Either(EitherSide<Vec<MatchResult>, Vec<MatchResult>>),

    Glob(Vec<Token>),
}
fn consume_whitespace<'a>(mut tokens: &'a [Token]) -> (bool, &'a [Token]) {
    let mut hasSeenWhitespace = false;
    while let Some(token) = tokens.get(0) {
        if !matches!(token.data, TokenData::Whitespace(_)) {
            break;
        }
        hasSeenWhitespace = true;
        tokens = &tokens[1..];
    }
    (hasSeenWhitespace, tokens)
}
use std::mem::discriminant;
pub fn test_tokens_against<'a>(
    test: &[Match],
    mut tokens: &'a [Token],
) -> Option<(&'a [Token], Vec<MatchResult>)> {
    let mut ret = Vec::with_capacity(test.len());
    for (i, method) in test.iter().enumerate() {
        match method {
            Match::IgnoreWhitespace => {
                tokens = consume_whitespace(tokens).1;
            }
            Match::Whitespace => {
                let (is_success, new_tokens) = consume_whitespace(tokens);
                if !is_success {
                    return None;
                }

                tokens = new_tokens;
            }
            Match::Of(condition) => {
                for i in 0..condition.len() {
                    if tokens.get(i)?.data != condition[i] {
                        return None;
                    }
                }
                tokens = &tokens[condition.len()..];
            }
            Match::OfType(condition) => {
                let mut found = Vec::with_capacity(condition.len());
                for i in 0..condition.len() {
                    // Test if the enum varieties are the same
                    if discriminant(&tokens.get(i)?.data) != discriminant(&condition[i]) {
                        return None;
                    }
                    found.push(tokens.get(i)?.clone());
                }
                tokens = &tokens[condition.len()..];
                ret.push(MatchResult::OfType(found))
            }
            Match::Optional(test) => {
                if let Some((new_tokens, res)) = test_tokens_against(test, tokens) {
                    tokens = new_tokens;
                    ret.push(MatchResult::Optional(Some(res)));
                } else {
                    ret.push(MatchResult::Optional(None));
                }
            }
            Match::PossibleCommaSeparated(test) => {
                let mut values = Vec::new();
                loop {
                    if let Some((new_tokens, res)) = test_tokens_against(test, tokens) {
                        tokens = new_tokens;
                        values.push(res);
                    } else {
                        break;
                    }
                    let next = tokens.get(0);
                    if next.is_none() {
                        break;
                    }
                    let next = next.unwrap();
                    if !matches!(next.data, TokenData::Colon) {
                        break;
                    }
                    tokens = &tokens[1..];
                }

                ret.push(MatchResult::PossibleCommaSeparated(values));
            }
            Match::Either(this, that) => {
                if let Some((new_tokens, res)) = test_tokens_against(this, tokens) {
                    tokens = new_tokens;
                    ret.push(MatchResult::Either(EitherSide::Left(res)));
                } else if let Some((new_tokens, res)) = test_tokens_against(that, tokens) {
                    tokens = new_tokens;
                    ret.push(MatchResult::Either(EitherSide::Right(res)));
                } else {
                    return None;
                }
            }
            Match::Glob => {
                let mut itr = tokens;
                let test = &test[i + 1..];

                while 0 != itr.len() {
                    if let Some((new_tokens, mut res)) = test_tokens_against(test, itr) {
                        ret.push(MatchResult::Glob(
                            // The shrink in ite size is the amount of loop iterations
                            tokens[..tokens.len() - itr.len()].to_vec(),
                        ));
                        tokens = new_tokens;

                        ret.append(&mut res);

                        break;
                    }
                    itr = &itr[1..];
                }
                break;
            }
        }
    }

    Some((tokens, ret))
}

#[cfg(test)]
mod tests {
    use super::super::tokenizer::*;
    use super::*;
    use crate::compiler::parsing::number_parser::NumberLiteral;
    use crate::compiler::parsing::Keyword;
    #[test]
    fn basic_pattern_matching_usage() {
        assert_eq!(
            Some((&[] as &[Token], vec![])),
            test_tokens_against(
                &[Match::Of(&[TokenData::Keyword(Keyword::If)])],
                tokenize_text("if".to_string()).unwrap().as_slice(),
            )
        );

        assert_eq!(
            Some((
                &[] as &[Token],
                vec![MatchResult::OfType(vec![Token {
                    index: 3,
                    length: 3,
                    data: TokenData::TextCluster("foo".to_string())
                }])]
            )),
            test_tokens_against(
                &[
                    Match::Of(&[TokenData::Keyword(Keyword::If)]),
                    Match::IgnoreWhitespace,
                    Match::OfType(&[TokenData::TextCluster("".to_string())])
                ],
                tokenize_text("if foo".to_string()).unwrap().as_slice(),
            )
        );
        assert_eq!(
            Some((
                &[] as &[Token],
                vec![MatchResult::Glob(
                    tokenize_text(" foo + bar".to_string()).unwrap()
                )]
            )),
            test_tokens_against(
                &[
                    Match::Of(&[TokenData::Keyword(Keyword::If)]),
                    Match::Glob,
                    Match::Of(&[TokenData::Semicolon])
                ],
                tokenize_text("if foo + bar;".to_string())
                    .unwrap()
                    .as_slice(),
            )
        );
        assert_eq!(
            Some((
                &[] as &[Token],
                vec![MatchResult::Glob(
                    tokenize_text("foo + bar".to_string()).unwrap()
                )]
            )),
            test_tokens_against(
                &[
                    Match::Of(&[TokenData::Keyword(Keyword::If)]),
                    // This time we consume the whitespace
                    Match::IgnoreWhitespace,
                    Match::Glob,
                    Match::Of(&[TokenData::Semicolon])
                ],
                tokenize_text("if foo + bar;".to_string())
                    .unwrap()
                    .as_slice(),
            )
        );
        assert!(matches!(
            test_tokens_against(
                &[
                    Match::Of(&[TokenData::Keyword(Keyword::If)]),
                    Match::Whitespace,
                    Match::Either(
                        &[Match::Of(&[TokenData::Keyword(Keyword::Comptime)])],
                        &[Match::Of(&[TokenData::Keyword(Keyword::Match)])],
                    )
                ],
                tokenize_text("if match".to_string()).unwrap().as_slice(),
            )
            .unwrap()
            .1[0],
            MatchResult::Either(EitherSide::Right(_))
        ));
        assert!(matches!(
            test_tokens_against(
                &[
                    Match::Of(&[TokenData::Keyword(Keyword::If)]),
                    Match::Whitespace,
                    Match::Either(
                        &[Match::Of(&[TokenData::Keyword(Keyword::Comptime)])],
                        &[Match::Of(&[TokenData::Keyword(Keyword::Match)])],
                    )
                ],
                tokenize_text("if comptime".to_string()).unwrap().as_slice(),
            )
            .unwrap()
            .1[0],
            MatchResult::Either(EitherSide::Left(_))
        ));
        assert_eq!(
            Some((&[] as &[_], vec![MatchResult::Optional(None)])),
            test_tokens_against(
                &[
                    Match::Of(&[TokenData::Keyword(Keyword::If)]),
                    Match::IgnoreWhitespace,
                    Match::Optional(&[Match::OfType(&[TokenData::NumberLiteral(
                        NumberLiteral::DUMMY()
                    )])]),
                    Match::IgnoreWhitespace,
                    Match::Of(&[TokenData::Keyword(Keyword::Var)]),
                ],
                tokenize_text("if var".to_string()).unwrap().as_slice()
            )
        );
        assert_eq!(
            Some((
                &[] as &[_],
                vec![MatchResult::Optional(Some(vec![MatchResult::OfType(
                    vec![Token {
                        index: 0,
                        length: 0,
                        data: TokenData::NumberLiteral(NumberLiteral::new("5").unwrap().1)
                    }]
                )]))]
            )),
            test_tokens_against(
                &[
                    Match::Of(&[TokenData::Keyword(Keyword::If)]),
                    Match::IgnoreWhitespace,
                    Match::Optional(&[Match::OfType(&[TokenData::NumberLiteral(
                        NumberLiteral::DUMMY()
                    )])]),
                    Match::IgnoreWhitespace,
                    Match::Of(&[TokenData::Keyword(Keyword::Var)]),
                ],
                tokenize_text("if 5 var".to_string()).unwrap().as_slice()
            )
        );
        assert_eq!(
            MatchResult::PossibleCommaSeparated(vec![
                vec![MatchResult::OfType(vec![Token {
                    index: 0,
                    length: 0,
                    data: TokenData::Keyword(Keyword::Else)
                }])],
                vec![MatchResult::OfType(vec![Token {
                    index: 0,
                    length: 0,
                    data: TokenData::Keyword(Keyword::Var)
                }])],
                vec![MatchResult::OfType(vec![Token {
                    index: 0,
                    length: 0,
                    data: TokenData::Keyword(Keyword::Defer)
                }])],
            ]),
            test_tokens_against(
                &[
                    Match::Of(&[TokenData::Keyword(Keyword::If)]),
                    Match::IgnoreWhitespace,
                    Match::PossibleCommaSeparated(&[
                        Match::IgnoreWhitespace,
                        Match::OfType(&[TokenData::Keyword(Keyword::DUMMY)])
                    ])
                ],
                tokenize_text("if  else, var,defer,".to_string())
                    .unwrap()
                    .as_slice()
            )
            .unwrap()
            .1[0]
        );
        assert_eq!(
            MatchResult::PossibleCommaSeparated(vec![]),
            test_tokens_against(
                &[
                    Match::Of(&[TokenData::Keyword(Keyword::If)]),
                    Match::IgnoreWhitespace,
                    Match::PossibleCommaSeparated(&[
                        Match::IgnoreWhitespace,
                        Match::OfType(&[TokenData::Keyword(Keyword::DUMMY)])
                    ])
                ],
                tokenize_text("if ".to_string()).unwrap().as_slice()
            )
            .unwrap()
            .1[0]
        );
    }
}
