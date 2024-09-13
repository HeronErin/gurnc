use std::{alloc::System, iter::Enumerate, vec};

use super::tokenizer::{Token, TokenData};
#[derive(Debug, Clone)]
pub enum Match<'a> {
    IgnoreWhitespace,
    Whitespace,
    Of(&'a [TokenData]),
    OfType(&'a [TokenData]),
    Optional(&'a [Match<'a>]),
    PossibleCommaSeparated(&'a [Match<'a>]),
    PossibleWhitespaceSeparated(&'a [Match<'a>]),
    Either(&'a [Match<'a>], &'a [Match<'a>]),

    Glob,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MatchResult {
    IgnoreWhitespace,
    Whitespace,
    Of(Vec<Token>),
    OfType(Vec<Token>),
    Optional(Option<Vec<MatchResult>>),
    PossibleCommaSeparated(Vec<Vec<MatchResult>>),
    PossibleWhitespaceSeparated(Vec<Vec<MatchResult>>),
    Either(EitherSide<Vec<MatchResult>, Vec<MatchResult>>),

    Glob(Vec<Token>),
}

#[derive(Debug, PartialEq, Clone)]
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
    mut tokens: &'a [Token]
) -> Option<(&'a [Token], Vec<MatchResult>)> {
    let mut ret = Vec::with_capacity(test.len());
    for (test_index, method) in test.iter().enumerate() {

        match method {
            Match::IgnoreWhitespace => {
                tokens = consume_whitespace(tokens).1;
                ret.push(MatchResult::IgnoreWhitespace);
            }
            Match::Whitespace => {
                let (is_success, new_tokens) = consume_whitespace(tokens);
                if !is_success {
                    return None;
                }
                ret.push(MatchResult::IgnoreWhitespace);
                tokens = new_tokens;
            }
            Match::Of(condition) => {
                let mut found = Vec::with_capacity(condition.len());
                for i in 0..condition.len() {
                    let tok = tokens.get(i)?;
                    if tok.data != condition[i] {
                        return None;
                    }
                    found.push(tok.clone());
                }
                ret.push(MatchResult::Of(found));
                tokens = &tokens[condition.len()..];
            }
            Match::OfType(condition) => {
                let mut found = Vec::with_capacity(condition.len());
                for i in 0..condition.len() {
                    // Test if the enum varieties are the same
                    let tok = tokens.get(i)?;
                    if discriminant(&tok.data) != discriminant(&condition[i]) {
                        return None;
                    }
                    found.push(tok.clone());
                }
                tokens = &tokens[condition.len()..];
                ret.push(MatchResult::OfType(found))
            }
            Match::Optional(opt_test) => {
                let chained_ops = opt_test.iter()
                    .chain(test[test_index+1..].iter())
                    .cloned()
                    .collect::<Vec<_>>();
                if let Some((new_tokens, mut res)) = test_tokens_against(
                    chained_ops.as_slice(),
                    tokens
                ) {
                    let mut after_option_results = res.split_off(opt_test.len());
                    
                    
                    tokens = new_tokens;
                    ret.push(MatchResult::Optional(Some(res)));
                    ret.append(&mut after_option_results);
                    break;
                } else {
                    ret.push(MatchResult::Optional(None));
                }
            }
            Match::PossibleCommaSeparated(test) | Match::PossibleWhitespaceSeparated(test) => {
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

                    if matches!(method, Match::PossibleCommaSeparated(_))
                        && !matches!(next.data, TokenData::Colon)
                    {
                        break;
                    }
                    if matches!(method, Match::PossibleWhitespaceSeparated(_))
                        && !matches!(next.data, TokenData::Whitespace(_))
                    {
                        tokens = consume_whitespace(tokens).1;
                        break;
                    }
                    tokens = &tokens[1..];
                }
                if matches!(method, Match::PossibleCommaSeparated(_)) {
                    ret.push(MatchResult::PossibleCommaSeparated(values));
                } else {
                    ret.push(MatchResult::PossibleWhitespaceSeparated(values));
                }
            }
            Match::Either(this, that) => {
                let this_chained = this.iter().chain(test[test_index+1..].iter()).cloned().collect::<Vec<_>>();
                let that_chained = that.iter().chain(test[test_index+1..].iter()).cloned().collect::<Vec<_>>();
                if let Some((new_tokens, mut res)) = test_tokens_against(this_chained.as_slice(), tokens) {
                    tokens = new_tokens;
                    let mut after_either_results = res.split_off(this.len());
                    ret.push(MatchResult::Either(EitherSide::Left(res)));
                    ret.append(&mut after_either_results);
                    break;

                } else if let Some((new_tokens, mut res)) = test_tokens_against(that_chained.as_slice(), tokens) {
                    tokens = new_tokens;
                    let mut after_either_results = res.split_off(that.len());
                    ret.push(MatchResult::Either(EitherSide::Right(res)));
                    ret.append(&mut after_either_results);
                    break;
                } else {
                    return None;
                }
            }
            Match::Glob => {
                let mut itr = tokens;
                let test = &test[test_index + 1..];
                let mut found_glob = false;
                println!("Glob at {:?}\nFor {:?}\n", itr, test);

                while 0 != itr.len() {
                    if let Some((new_tokens, mut res)) = test_tokens_against(test, itr) {
                        found_glob = true;
                        ret.push(MatchResult::Glob(
                            // The shrink in itr size is the amount of loop iterations
                            tokens[..tokens.len() - itr.len()].to_vec(),
                        ));
                        tokens = new_tokens;

                        ret.append(&mut res);

                        break;
                    }
                    itr = &itr[1..];
                }
                if !found_glob {
                    return None;
                }
                break;
            }
        }
    }

    Some((tokens, ret))
}

// #[cfg(test)]
// mod tests {
//     use super::super::tokenizer::*;
//     use super::*;
//     use crate::compiler::parsing::number_parser::NumberLiteral;
//     use crate::compiler::parsing::Keyword;
//     #[test]
//     fn basic_pattern_matching_usage() {
//         assert_eq!(
//             Some((&[] as &[Token], vec![])),
//             test_tokens_against(
//                 &[Match::Of(&[TokenData::Keyword(Keyword::If)])],
//                 tokenize_text("if".to_string()).unwrap().as_slice(),
//             )
//         );

//         assert_eq!(
//             Some((
//                 &[] as &[Token],
//                 vec![MatchResult::OfType(vec![Token {
//                     index: 3,
//                     length: 3,
//                     data: TokenData::TextCluster(Some("foo".to_string()))
//                 }])]
//             )),
//             test_tokens_against(
//                 &[
//                     Match::Of(&[TokenData::Keyword(Keyword::If)]),
//                     Match::IgnoreWhitespace,
//                     Match::OfType(&[TokenData::TextCluster(Some("".to_string()))])
//                 ],
//                 tokenize_text("if foo".to_string()).unwrap().as_slice(),
//             )
//         );
//         assert_eq!(
//             Some((
//                 &[] as &[Token],
//                 vec![MatchResult::Glob(
//                     tokenize_text(" foo + bar".to_string()).unwrap()
//                 )]
//             )),
//             test_tokens_against(
//                 &[
//                     Match::Of(&[TokenData::Keyword(Keyword::If)]),
//                     Match::Glob,
//                     Match::Of(&[TokenData::Semicolon])
//                 ],
//                 tokenize_text("if foo + bar;".to_string())
//                     .unwrap()
//                     .as_slice(),
//             )
//         );
//         assert_eq!(
//             Some((
//                 &[] as &[Token],
//                 vec![MatchResult::Glob(
//                     tokenize_text("foo + bar".to_string()).unwrap()
//                 )]
//             )),
//             test_tokens_against(
//                 &[
//                     Match::Of(&[TokenData::Keyword(Keyword::If)]),
//                     // This time we consume the whitespace
//                     Match::IgnoreWhitespace,
//                     Match::Glob,
//                     Match::Of(&[TokenData::Semicolon])
//                 ],
//                 tokenize_text("if foo + bar;".to_string())
//                     .unwrap()
//                     .as_slice(),
//             )
//         );
//         assert!(matches!(
//             test_tokens_against(
//                 &[
//                     Match::Of(&[TokenData::Keyword(Keyword::If)]),
//                     Match::Whitespace,
//                     Match::Either(
//                         &[Match::Of(&[TokenData::Keyword(Keyword::Comptime)])],
//                         &[Match::Of(&[TokenData::Keyword(Keyword::Match)])],
//                     )
//                 ],
//                 tokenize_text("if match".to_string()).unwrap().as_slice(),
//             )
//             .unwrap()
//             .1[0],
//             MatchResult::Either(EitherSide::Right(_))
//         ));
//         assert!(matches!(
//             test_tokens_against(
//                 &[
//                     Match::Of(&[TokenData::Keyword(Keyword::If)]),
//                     Match::Whitespace,
//                     Match::Either(
//                         &[Match::Of(&[TokenData::Keyword(Keyword::Comptime)])],
//                         &[Match::Of(&[TokenData::Keyword(Keyword::Match)])],
//                     )
//                 ],
//                 tokenize_text("if comptime".to_string()).unwrap().as_slice(),
//             )
//             .unwrap()
//             .1[0],
//             MatchResult::Either(EitherSide::Left(_))
//         ));
//         assert_eq!(
//             Some((&[] as &[_], vec![MatchResult::Optional(None)])),
//             test_tokens_against(
//                 &[
//                     Match::Of(&[TokenData::Keyword(Keyword::If)]),
//                     Match::IgnoreWhitespace,
//                     Match::Optional(&[Match::OfType(&[TokenData::NumberLiteral(
//                         NumberLiteral::DUMMY()
//                     )])]),
//                     Match::IgnoreWhitespace,
//                     Match::Of(&[TokenData::Keyword(Keyword::Var)]),
//                 ],
//                 tokenize_text("if var".to_string()).unwrap().as_slice()
//             )
//         );
//         assert_eq!(
//             Some((
//                 &[] as &[_],
//                 vec![MatchResult::Optional(Some(vec![MatchResult::OfType(
//                     vec![Token {
//                         index: 0,
//                         length: 0,
//                         data: TokenData::NumberLiteral(NumberLiteral::new("5").unwrap().1)
//                     }]
//                 )]))]
//             )),
//             test_tokens_against(
//                 &[
//                     Match::Of(&[TokenData::Keyword(Keyword::If)]),
//                     Match::IgnoreWhitespace,
//                     Match::Optional(&[Match::OfType(&[TokenData::NumberLiteral(
//                         NumberLiteral::DUMMY()
//                     )])]),
//                     Match::IgnoreWhitespace,
//                     Match::Of(&[TokenData::Keyword(Keyword::Var)]),
//                 ],
//                 tokenize_text("if 5 var".to_string()).unwrap().as_slice()
//             )
//         );
//         assert_eq!(
//             MatchResult::PossibleCommaSeparated(vec![
//                 vec![MatchResult::OfType(vec![Token {
//                     index: 0,
//                     length: 0,
//                     data: TokenData::Keyword(Keyword::Else)
//                 }])],
//                 vec![MatchResult::OfType(vec![Token {
//                     index: 0,
//                     length: 0,
//                     data: TokenData::Keyword(Keyword::Var)
//                 }])],
//                 vec![MatchResult::OfType(vec![Token {
//                     index: 0,
//                     length: 0,
//                     data: TokenData::Keyword(Keyword::Defer)
//                 }])],
//             ]),
//             test_tokens_against(
//                 &[
//                     Match::Of(&[TokenData::Keyword(Keyword::If)]),
//                     Match::IgnoreWhitespace,
//                     Match::PossibleCommaSeparated(&[
//                         Match::IgnoreWhitespace,
//                         Match::OfType(&[TokenData::Keyword(Keyword::DUMMY)])
//                     ])
//                 ],
//                 tokenize_text("if  else, var,defer,".to_string())
//                     .unwrap()
//                     .as_slice()
//             )
//             .unwrap()
//             .1[0]
//         );
//         assert_eq!(
//             MatchResult::PossibleCommaSeparated(vec![]),
//             test_tokens_against(
//                 &[
//                     Match::Of(&[TokenData::Keyword(Keyword::If)]),
//                     Match::IgnoreWhitespace,
//                     Match::PossibleCommaSeparated(&[
//                         Match::IgnoreWhitespace,
//                         Match::OfType(&[TokenData::Keyword(Keyword::DUMMY)])
//                     ])
//                 ],
//                 tokenize_text("if ".to_string()).unwrap().as_slice()
//             )
//             .unwrap()
//             .1[0]
//         );
//     }
// }
