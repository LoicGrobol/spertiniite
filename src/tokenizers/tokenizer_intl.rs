use lazy_static::lazy_static;
use regex::Regex;

pub fn tokenize(input: &str) -> Vec<String> {
    lazy_static! {
        static ref PUNCT_NON_DIGIT: Regex = Regex::new(r"(\P{N})(\p{P})").unwrap();
        static ref NON_DIGIT_PUNCT: Regex = Regex::new(r"(\P{N})(\p{P})").unwrap();
        static ref SYMBOLS: Regex = Regex::new(r"(\P{S})").unwrap();
    }
    // TODO: maybe this could be made faster using `regex::Regex.split`?
    let mut res = PUNCT_NON_DIGIT.replace_all(input, "$1 $2 ").into_owned();
    res = NON_DIGIT_PUNCT.replace_all(res.as_ref(), " $1 $2").into_owned();
    res = SYMBOLS.replace_all(res.as_ref(), " $1 ").into_owned();
    res.split_whitespace().map(|x| {x.to_owned()}).collect()
}
