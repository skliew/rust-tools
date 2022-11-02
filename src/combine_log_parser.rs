use std::io;
use std::io::Write;
use combine::{choice, attempt, many};
use combine::parser::char::{space, spaces, char};
use combine::parser::range::{take, take_while1, take_while};
use combine::{skip_many, Parser, satisfy, sep_by};
use combine::{RangeStream, EasyParser};
use serde_json;
use std::collections::HashMap;
use std::iter::{Extend, FromIterator};

fn alphanum_dash<'a, I>() -> impl Parser<I, Output = &'a str>
where I: RangeStream<Token = char, Range = &'a str>,
{
    take_while1(|x: char| x.is_alphanumeric() || x == '-' || x == '.')
}

fn ip<'a, I>() -> impl Parser<I>
where I: RangeStream<Token = char, Range = &'a str>,
{
    take_while1(|x: char| x.is_numeric() || x == '.')
}

fn escaped_character<'a, I>() -> impl Parser<I, Output = char>
where I: RangeStream<Token = char>,
{
    (char('\\'),
     satisfy(|_x| true)).map(|(_,y)| y)
}

fn escaped_characters<'a, I>() -> impl Parser<I, Output = String> + 'a
where I: RangeStream<Token = char, Range = &'a str> + 'a,
{
    (escaped_character(),
     take_while1(|x: char| x != '"' && x != '\\')
    ).map(|(x, y): (char, &'a str)| {
        let mut result = String::from(y);
        result.insert(0, x);
        result
     })
}

fn parse_string<'a, I>() -> impl Parser<I, Output = String> + 'a
where I: RangeStream<Token = char, Range = &'a str> + 'a,
{
    (
        char('"'),
        take_while1(|x: char| x != '"' && x != '\\'),
        many(escaped_characters()),
        char('"'),
    ).map(|(_, x, y, _): (_, &str, Vec<String>, _)| {
        let mut result = String::from(x);
        let string2 : String = y.into_iter().collect();
        result.push_str(&string2);
        result
    })
}

fn parse_kv<'a, I>() -> impl Parser<I, Output = (&'a str, String)> + 'a
where I: RangeStream<Token = char, Range = &'a str> + 'a,
{
    (alphanum_dash(), char('='), parse_string()).map(|(x,_,z)| (x, z))
}

fn parse_kvmap<'a, I>() -> impl Parser<I, Output = Vec<(&'a str, String)>>
where I: RangeStream<Token = char, Range = &'a str> + 'a,
{
    sep_by(parse_kv(), spaces())
}

fn parse_rest_of_string<'a, I>() -> impl Parser<I, Output = Vec<(&'a str, String)>>
where I: RangeStream<Token = char, Range = &'a str>,
{
    take_while(|_x| true).map(|x: &'a str| Vec::from([("msg", x.to_string())]))
}

fn parse_rest<'a, I>() -> impl Parser<I, Output = HashMap<&'a str, String>>
where I: RangeStream<Token = char, Range = &'a str> + 'a,
{
    (choice((attempt(parse_kvmap()), parse_rest_of_string()))).map(|x| HashMap::from_iter(x))
}

fn parse_log<'a, I>() -> impl Parser<I, Output = HashMap<&'a str, String>>
where I: RangeStream<Token = char, Range = &'a str> + 'a,
{
    (take(20).with(take(19).skip((take(21), take_while1(|x: char| x.is_alphanumeric()), skip_many(space())))),
     alphanum_dash().skip(spaces()),
     (ip(), spaces(), alphanum_dash(), spaces(), alphanum_dash(), spaces()).with(alphanum_dash()),
     spaces().with(parse_rest())
     ).map(|(x,y,z,kv):(&'a str, &'a str, &'a str, HashMap<&'a str, String>)| {
        let mut h = HashMap::from([("ptDate", x.to_string()), ("serviceName", y.to_string()), ("podName", z.to_string())]);
        h.extend(kv);
        h
    })
}

fn to_json<'a>(input: HashMap<&'a str, String>) -> Result<String, String> {
    match serde_json::to_string(&input) {
        Ok(result) => Ok(result),
        Err(err) => Err(format!("{}", err)),
    }
}

fn parse<'a>(input: &'a str) -> Result<String, String> {
    match parse_log().easy_parse(input) {
        Ok((result, _)) => Ok(to_json(result)?),
        Err(err) => {
            panic!("{:?}, {}", err.to_string(), input);
        },
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lines() {
        let input = line?;
        let result = parse(&input)?;
        match writeln!(stdout, "{}", result) {
            Err(err) => {
                if err.kind() == std::io::ErrorKind::BrokenPipe {
                    return Ok(());
                } else {
                    panic!("GG");
                }
            },
            Ok(_) => (),
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() -> Result<(), Box<dyn std::error::Error>> {
        let input = "\"def\\\"abc\"";
        let result = parse_string().easy_parse(input)?;
        assert_eq!("def\"abc", result.0);
        Ok(())
    }
}
