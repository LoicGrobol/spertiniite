use tokenizer_re::tokenize;

const UCODE_RANGES: [(char, char)] = [
    ('\u{3400}', '\u{4db5}'), // CJK Unified Ideographs Extension A, release 3.0
    ('\u{4e00}', '\u{9fa5}'), // CJK Unified Ideographs, release 1.1
    ('\u{9fa6}', '\u{9fbb}'), // CJK Unified Ideographs, release 4.1
    ('\u{f900}', '\u{fa2d}'), // CJK Compatibility Ideographs, release 1.1
    ('\u{fa30}', '\u{fa6a}'), // CJK Compatibility Ideographs, release 3.2
    ('\u{fa70}', '\u{fad9}'), // CJK Compatibility Ideographs, release 4.1
    ('\u{20000}', '\u{2a6d6}'), // (UTF16) CJK Unified Ideographs Extension B, release 3.1
    ('\u{2f800}', '\u{2fa1d}'), // (UTF16) CJK Compatibility Supplement, release 3.1
    ('\u{ff00}', '\u{ffef}'), // Full width ASCII, full width of English punctuation,
    // half width Katakana, half wide half width kana, Korean alphabet
    ('\u{2e80}', '\u{2eff}'), // CJK Radicals Supplement
    ('\u{3000}', '\u{303f}'), // CJK punctuation mark
    ('\u{31c0}', '\u{31ef}'), // CJK stroke
    ('\u{2f00}', '\u{2fdf}'), // Kangxi Radicals
    ('\u{2ff0}', '\u{2fff}'), // Chinese character structure
    ('\u{3100}', '\u{312f}'), // Phonetic symbols
    ('\u{31a0}', '\u{31bf}'), // Phonetic symbols (Taiwanese and Hakka expansion)
    ('\u{fe10}', '\u{fe1f}'),
    ('\u{fe30}', '\u{fe4f}'),
    ('\u{2600}', '\u{26ff}'),
    ('\u{2700}', '\u{27bf}'),
    ('\u{3200}', '\u{32ff}'),
    ('\u{3300}', '\u{33ff}'),
];

fn is_chinese_char(uchar: &char) -> bool {
    UCODE_RANGES.iter().any(|range| {
        if let (start, end) = range {
            start <= uchar && uchar <= end
        }
    })
}

fn tokenize(input: str) -> Vec<&str> {
    input = input.trim();
    let res = input
        .chars()
        .flat_map(|&c| {
            if is_chinese_char(c) {
                [' ', c, ' ']
            } else {
                [c]
            }
        })
        .collect::<str>();
    tokenizer_re::tokenize(res)
}
