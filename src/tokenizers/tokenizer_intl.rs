use lazy_static::lazy_static;
use regex::Regex;

pub fn tokenize(input: &str) -> Vec<&str> {
    lazy_static! {
        static ref punct_non_digit: Regex = Regex::new(r"(\P{N})(\p{P})").unwrap();
        static ref non_digit_punct : Regex = Regex::new(r"(\P{N})(\p{P})").unwrap();
        static ref symbols : Regex = Regex::new(r"(\P{S})").unwrap();
    }
    // TODO: maybe this could be made faster using `regex::Regex.split`?
    let res = punct_non_digit.replace_all(input, "$1 $2 ");
    res =non_digit_punct.replace_all(res.as_ref(), " $1 $2");
    res = symbols.replace_all(res.as_ref(), " $1 ");
    res.as_ref().split_whitespace().collect()
}