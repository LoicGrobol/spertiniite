use std::borrow::Borrow;

use lazy_static::lazy_static;
use regex::Regex;

pub fn tokenize(input: &str) -> Vec<String> {
    lazy_static! {
        static ref WESTERN: Regex = Regex::new(r"([\{-\~\[-\` -\&\(-\+\:-\@\/])").unwrap();
        static ref DIGIT_PERIOD_COMMA: Regex = Regex::new(r"([^0-9])([\.,])").unwrap();
        static ref PERIOD_COMMA_DIGIT: Regex = Regex::new(r"([\.,])([^0-9]").unwrap();
        static ref DIGIT_DASH: Regex = Regex::new(r"([0-9])(-)").unwrap();
    }
    // TODO: maybe this could be made faster using `regex::Regex.split`?
    let mut res = WESTERN.replace_all(&input, "$1 ").into_owned();
    res = DIGIT_PERIOD_COMMA.replace_all(res.as_ref(), "$1 $2 ").into_owned();
    res = PERIOD_COMMA_DIGIT.replace_all(res.as_ref(), " $1 $2").into_owned();
    res = DIGIT_DASH.replace_all(res.as_ref(), "$1 $2 ").into_owned();
    res.split_whitespace().map(|x| {x.to_owned()}).collect()
}
