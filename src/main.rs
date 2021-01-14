use std::io;
use std::io::Write;
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut last_message = "".to_owned();
    loop {
        print!("> ");
        io::stdout().flush();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                last_message = input;
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn cost(haystack: &[&str], needle: &[&str], i: usize, la: usize, lb: usize) -> usize {
    if lb == 0 { la }
    else if la == 0 { lb }
    else if haystack[i + la - 1] == needle[lb - 1] {
        cost(haystack, needle, i, la - 1, lb - 1)
    } else {
        1 + cmp::min(
            cmp::min(
                cost(haystack, needle, i, la - 1, lb),
                cost(haystack, needle, i, la, lb - 1)
            ),
            cost(haystack, needle, i, la - 1, lb - 1)
        )
    }
}

fn closest_substring(haystack: &str, needle: &str) -> (usize, usize) {
    // We're looking for
    //      min_{i, j} f(i, j, |needle|)
    // where:
    //      f(i, la, lb) =
    //          la if lb == 0
    //          lb if la == 0
    //          f(i, la-1, lb-1) if haystack[i+la-1] == needle[lb-1]
    //          1 + min { f(i, la-1, lb), f(i, la, lb-1), f(i, la-1, lb-1) } otherwise

    let haystack = UnicodeSegmentation::graphemes(haystack, true).collect::<Vec<&str>>();
    let needle = UnicodeSegmentation::graphemes(needle, true).collect::<Vec<&str>>();
    let mut best_cost = usize::MAX;
    let mut best_pair = (0, 0);
    for i in 0..haystack.len()-1 {
        for j in i+1..haystack.len() {
            let c = cost(&haystack, &needle, i, j-i, needle.len());
            if c < best_cost {
                best_cost = c;
                best_pair = (i, j);
            }
        }
    }

    best_pair
}

mod tests {
    use crate::*;

    #[test]
    fn test_closest_substring() {
        let pair = closest_substring("Hello warld!", "world");
        assert!(pair == (6, 11));
    }
}

