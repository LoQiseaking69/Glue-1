use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::{map, opt, recognize},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
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
    map(recognize(pair(
        alpha1,
        many0(alt((alphanumeric1, char('_'))),
    ))), |id: &str| Expr::Identifier(id.to_string()))(input)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(digit1, |n: &str| Expr::Number(n.parse().unwrap()))(input)
}

fn parse_rune(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(char('['), recognize(many0(char('.'))), char(']')),
        |rune: &str| Expr::Rune(rune.to_string()),
    )(input)
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
    let (input, expr) = parse_primary(input)?;
    let (input, bin_ops) = many0(pair(parse_bin_op, parse_primary))(input)?;

    let mut result = expr;
    for (op, operand) in bin_ops {
        result = Expr::BinOp(Box::new(result), op, Box::new(operand));
    }

    Ok((input, result))
}

fn parse_assignment(input: &str) -> IResult<&str, Expr> {
    let (input, _) = multispace0(input)?;
    let (input, identifier) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = parse_expr(input)?;

    Ok((
        input,
        Expr::Assignment(identifier.get_identifier(), Box::new(expr)),
    ))
}

fn parse_if(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("HORUS")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, condition) = parse_expr(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, if_branch) = many0(terminated(parse_statement, multispace0))(input)?;
    let (input, _) = tag("PYRAMID")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, else_branch) = many0(terminated(parse_statement, multispace0))(input)?;

    Ok((
        input,
        Expr::If(Box::new(condition), if_branch, else_branch),
    ))
}

fn parse_statement(input: &str) -> IResult<&str, Expr> {
    alt((parse_assignment, parse_if, parse_expr))(input)
}

fn main() {
    let code = r#"
VALKYRIE x = 5
HORUS if x > 0:
    ENIGMA y = x + 1
PYRAMID else:
    ENIGMA y = x - 1
"#;

    let (_, statements) = many0(terminated(parse_statement, multispace0))(code).unwrap();

    println!("{:?}", statements);
}

