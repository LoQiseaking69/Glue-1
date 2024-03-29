use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1},
    combinator::{map, recognize},
    multi::{many0},
    sequence::{delimited, pair, tuple},
    IResult,
};

#[derive(Debug)]
enum Expr {
    Assignment(String, Box<Expr>),
    If(Box<Expr>, Vec<Expr>, Vec<Expr>),
    Identifier(String),
    Number(i32),
    Rune(String),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug)]
enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn parse_identifier(input: &str) -> IResult<&str, Expr> {
    map(recognize(pair(alpha1, many0(alt((alphanumeric1, char('_')))))),
        |id: &str| Expr::Identifier(id.to_string()))(input)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(digit1, |n: &str| Expr::Number(n.parse().unwrap()))(input)
}

fn parse_rune(input: &str) -> IResult<&str, Expr> {
    map(delimited(char('['), recognize(many0(char('.'))), char(']')),
        |rune: &str| Expr::Rune(rune.to_string()))(input)
}

fn parse_primary(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_identifier,
        parse_number,
        parse_rune,
        delimited(char('('), parse_expr, char(')')),
    ))(input)
}

fn parse_bin_op(input: &str) -> IResult<&str, BinOp> {
    alt((
        map(tag("+"), |_| BinOp::Add),
        map(tag("-"), |_| BinOp::Subtract),
        map(tag("*"), |_| BinOp::Multiply),
        map(tag("/"), |_| BinOp::Divide),
    ))(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, primary) = parse_primary(input)?;
    many0(tuple((parse_bin_op, parse_primary)))(input)
        .map(|(next_input, list)| {
            let expr = list.into_iter().fold(primary, |acc, (op, val)| {
                Expr::BinOp(Box::new(acc), op, Box::new(val))
            });
            (next_input, expr)
        })
}

fn parse_if(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("HORUS")(input)?;
    let (input, condition) = delimited(multispace1, parse_expr, multispace0)(input)?;
    let (input, if_branch) = many0(parse_statement)(input)?;
    let (input, _) = tag("PYRAMID")(input)?;
    let (input, else_branch) = many0(parse_statement)(input)?;

    Ok((input, Expr::If(Box::new(condition), if_branch, else_branch)))
}

fn parse_assignment(input: &str) -> IResult<&str, Expr> {
    let (input, id) = parse_identifier(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, expr) = parse_expr(input)?;

    Ok((input, Expr::Assignment(id.get_identifier(), Box::new(expr))))
}

fn parse_statement(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_assignment,
        parse_if,
        parse_expr,
    ))(input)
}

fn main() {
    let code = r#"
    HORUS x > 0 :
        VALKYRIE y = x + 1
    PYRAMID :
        VALKYRIE y = x - 1
    "#;

    match many0(parse_statement)(code) {
        Ok((_, statements)) => println!("Parsed Statements: {:?}", statements),
        Err(err) => println!("Error parsing code: {:?}", err),
    }
}