

use nom::{
    character::complete::{multispace1, digit1, alphanumeric0},
    multi::many0_count,
    character::{is_alphabetic, is_alphanumeric},
    branch::alt,
    IResult,
    bytes::complete::tag,
    bytes::complete::take_until,
    bytes::complete::take_till,
};
pub fn get_string<'a>(input: & 'a str) -> IResult<&str, &str>{
    let(input, first) = alt((tag("'"), tag("\"")))(input)?;
    let (input, m_string) = take_until(first)(input)?;
    let (input, _) = tag(first)(input)?;
    Ok((input, m_string))
}
pub fn get_figure(input: &str) -> IResult<&str, &str>{
    let (input, figure) = digit1(input)?;
    Ok((input, figure))
}
pub fn get_reserved( input: &str) -> IResult<&str, &str>{
    let (input, reserved) = alt((tag("="), tag("*"), tag("/"), tag("-"), tag("+")))(input)?;
    Ok((input, reserved))
}
pub fn get_word(input: & str)->IResult<&str, & str> {
    let first = input;
    let mut input = input;
    let c = get_str_first(input);
    if is_alphabetic(c) || c == b'_'{}
    else {
        let error : IResult<&str, &str> =
        Err(nom::Err::Error(nom::error::Error::new(first, nom::error::ErrorKind::AlphaNumeric)));
        return error;
    }
    let _get_word = |input| -> IResult<& str, & str>{
        let ch = get_str_first(input);
        if is_alphanumeric(ch){
            let (input, word) = alphanumeric0(input)?;
            Ok((input, word))
        }else if ch == b'_'{
            let (input, _) = many0_count(tag("_"))(input)?;
            // このあと計算イラン get_wordでする get_word で計算している
            Ok((input, "_"))
        }else {
            Ok((input, ""))
        }
    };
    loop{
        // get under vars or alphanum
        let (_input, word) = _get_word(input)?;
        input = _input;
        if word == ""{break}
    }
    //wordの計算
    let word = get_str_between(first, input);
    Ok((input, word))
}

pub fn get_str_between<'a >(pfirst: & 'a str, pend: &'a str)-> & 'a str{
    let word = unsafe{std::slice::from_raw_parts(
        pfirst.as_ptr(),
        pend.as_ptr().offset_from(pfirst.as_ptr()) as usize
    )};
    std::str::from_utf8(word).unwrap()
}
pub fn ignore_space_or_return(input: & str) -> IResult<& str, & str>{
    multispace1(input)
}
pub fn ignore_comment(input: &str) -> IResult<&str, &str>{
    let (input, first) = alt((tag("//"), tag("*/")))(input)?;
    if first == "*/"{
        let (input, _) = take_until("*/")(input)?;
        let (input, _) = tag("*/")(input)?;
        Ok((input, "" ))
    }else {
        let (input, _) = take_until("\n")(input)?;
        let (input, _) = tag("\n")(input)?;
        Ok((input, "" ))
    }
}
pub fn wrapper_pos(origin: &str, branch: &str) -> isize{
    unsafe{branch.as_ptr().offset_from(origin.as_ptr())}
}
pub fn get_str_first( input: & str) -> u8{
    unsafe{*input.as_ptr()}
}
