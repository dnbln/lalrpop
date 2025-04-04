use std::iter;

use crate::grammar::parse_tree::*;
use crate::grammar::pattern::*;
use crate::tok;

#[rustfmt::skip]
#[allow(dead_code)]
#[allow(clippy::all)]
mod lrgrammar;


#[cfg(test)]
mod test;

pub enum Top {
    Grammar(Grammar),
    Pattern(Pattern<TypeRef>),
    MatchMapping(MatchMapping),
    TypeRef(TypeRef),
    GrammarWhereClauses(Vec<WhereClause<TypeRef>>),
}

pub type ParseError<'input> = lalrpop_util::ParseError<usize, tok::Tok<'input>, tok::Error>;

macro_rules! parser {
    ($input:expr, $offset:expr, $pat:ident, $tok:ident) => {{
        let input = $input;
        let tokenizer =
            iter::once(Ok((0, tok::Tok::$tok, 0))).chain(tok::Tokenizer::new(input, $offset));
        lrgrammar::TopParser::new()
            .parse(input, tokenizer)
            .map(|top| match top {
                Top::$pat(x) => x,
                _ => unreachable!(),
            })
    }};
}

pub fn parse_grammar(input: &str) -> Result<Grammar, ParseError<'_>> {
    let mut grammar = parser!(input, 0, Grammar, StartGrammar)?;

    // find a unique prefix that does not appear anywhere in the input
    while input.contains(&grammar.prefix) {
        grammar.prefix.push('_');
    }

    Ok(grammar)
}

fn parse_pattern(input: &str, offset: usize) -> Result<Pattern<TypeRef>, ParseError<'_>> {
    parser!(input, offset, Pattern, StartPattern)
}

fn parse_match_mapping(input: &str, offset: usize) -> Result<MatchMapping, ParseError<'_>> {
    parser!(input, offset, MatchMapping, StartMatchMapping)
}

#[cfg(test)]
pub fn parse_type_ref(input: &str) -> Result<TypeRef, ParseError<'_>> {
    parser!(input, 0, TypeRef, StartTypeRef)
}

#[cfg(test)]
pub fn parse_where_clauses(input: &str) -> Result<Vec<WhereClause<TypeRef>>, ParseError<'_>> {
    parser!(input, 0, GrammarWhereClauses, StartGrammarWhereClauses)
}
