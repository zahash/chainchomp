pub trait Parser<T, E, Ast> {
    fn parse(&self, tokens: &[T], pos: usize) -> Result<(Ast, usize), E>;
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

pub fn combine_parsers<T, E, Ast>(
    tokens: &[T],
    pos: usize,
    parsers: &[&dyn Parser<T, E, Ast>],
    e: E,
) -> Result<(Ast, usize), E> {
    for parser in parsers {
        match parser.parse(tokens, pos) {
            Ok((ast, pos)) => return Ok((ast, pos)),
            Err(_) => continue,
        };
    }

    Err(e)
}

pub fn many<T, E, Ast>(
    tokens: &[T],
    mut pos: usize,
    parser: impl Fn(&[T], usize) -> Result<(Ast, usize), E>,
) -> (Vec<Ast>, usize) {
    let mut list = vec![];

    while let Ok((ast, next_pos)) = parser(tokens, pos) {
        list.push(ast);
        pos = next_pos;
    }

    (list, pos)
}

pub fn many_delimited<T, E, Ast>(
    tokens: &[T],
    mut pos: usize,
    parser: impl Fn(&[T], usize) -> Result<(Ast, usize), E>,
    delimiter: &T,
) -> (Vec<Ast>, usize)
where
    T: PartialEq,
{
    let mut list = vec![];

    while let Ok((ast, next_pos)) = parser(tokens, pos) {
        list.push(ast);
        pos = next_pos;

        match tokens.get(pos) {
            Some(token) if token == delimiter => pos += 1,
            _ => break,
        };
    }

    (list, pos)
}

pub fn maybe<T, E, Ast>(
    tokens: &[T],
    pos: usize,
    parser: impl Fn(&[T], usize) -> Result<(Ast, usize), E>,
) -> (Option<Ast>, usize) {
    match parser(tokens, pos) {
        Ok((ast, pos)) => (Some(ast), pos),
        Err(_) => (None, pos),
    }
}
