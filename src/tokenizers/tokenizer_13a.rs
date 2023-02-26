use lazy_static::lazy_static;
use regex::Regex;


fn tokenize(input: String) -> Vec<String> {
    lazy_static! {
        static ref delete: Regex = Regex::new(r"<skipped>|-\n").unwrap();
    }
    let res = delete.replace_all(input, "");
    super::tokenizer_re::tokenize(
        res.replace("\n", " ")
            .replace("&quot;", "\"")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
    )
}
