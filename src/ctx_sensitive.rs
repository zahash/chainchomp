pub trait Parser<T, E, C, Ast> {
    fn parse(&self, tokens: &[T], pos: usize, ctx: &mut C) -> Result<(Ast, usize), E>;
}

impl<T, E, C, ParsedValue, F, Ast> Parser<T, E, C, Ast> for F
where
    ParsedValue: Into<Ast>,
    F: Fn(&[T], usize, &mut C) -> Result<(ParsedValue, usize), E>,
{
    fn parse(&self, tokens: &[T], pos: usize, ctx: &mut C) -> Result<(Ast, usize), E> {
        match self(tokens, pos, ctx) {
            Ok((val, pos)) => Ok((val.into(), pos)),
            Err(e) => Err(e),
        }
    }
}

pub fn combine_parsers<T, E, C, Ast>(
    tokens: &[T],
    pos: usize,
    ctx: &mut C,
    parsers: &[Box<dyn Parser<T, E, C, Ast>>],
    e: E,
) -> Result<(Ast, usize), E> {
    for parser in parsers {
        match parser.parse(tokens, pos, ctx) {
            Ok((ast, pos)) => return Ok((ast, pos)),
            Err(_) => continue,
        };
    }

    Err(e)
}

pub fn maybe<T, E, C, Ast>(
    tokens: &[T],
    pos: usize,
    ctx: &mut C,
    parser: impl Fn(&[T], usize, &mut C) -> Result<(Ast, usize), E>,
) -> (Option<Ast>, usize) {
    match parser(tokens, pos, ctx) {
        Ok((ast, pos)) => (Some(ast), pos),
        Err(_) => (None, pos),
    }
}
