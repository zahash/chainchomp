pub trait Parser<T, E, Ast> {
    fn parse(&self, tokens: &[T], pos: usize) -> Result<(Ast, usize), E>;
}

pub trait SyntaxError<C> {
    fn syntax_error(pos: usize, ctx: C) -> Self;
}

impl<T, E, ParsedValue, F, Ast> Parser<T, E, Ast> for F
where
    ParsedValue: Into<Ast>,
    F: Fn(&[T], usize) -> Result<(ParsedValue, usize), E>,
{
    fn parse(&self, tokens: &[T], pos: usize) -> Result<(Ast, usize), E> {
        match self(tokens, pos) {
            Ok((val, pos)) => Ok((val.into(), pos)),
            Err(e) => Err(e),
        }
    }
}

pub fn combine_parsers<T, E, C, Ast>(
    tokens: &[T],
    pos: usize,
    parsers: &[Box<dyn Parser<T, E, Ast>>],
    ctx: C,
) -> Result<(Ast, usize), E>
where
    E: SyntaxError<C>,
{
    for parser in parsers {
        match parser.parse(tokens, pos) {
            Ok((ast, pos)) => return Ok((ast, pos)),
            Err(_) => continue,
        };
    }

    Err(E::syntax_error(pos, ctx))
}
