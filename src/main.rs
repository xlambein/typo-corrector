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

    let mut memo = vec![vec![vec![usize::MAX;needle.len()+1];haystack.len()+1];haystack.len()];

    for i in 0..haystack.len() {
        for la in 0..haystack.len()+1 {
            memo[i][la][0] = la;
        }
        for lb in 0..needle.len()+1 {
            memo[i][0][lb] = lb;
        }
    }

    for i in 0..haystack.len() {
        for la in 1..haystack.len()-i+1 {
            for lb in 1..needle.len()+1 {
                if haystack[i + la - 1] == needle[lb - 1] {
                    memo[i][la][lb] = memo[i][la-1][lb-1];
                } else {
                    memo[i][la][lb] = 1 + cmp::min(
                        cmp::min(
                            memo[i][la-1][lb],
                            memo[i][la][lb-1]
                        ),
                        memo[i][la-1][lb-1]
                    );
                }
            }
        }
    }

    let mut best_cost = usize::MAX;
    let mut best_pair = (usize::MAX, usize::MAX);
    for i in 0..haystack.len() {
        for j in i+1..haystack.len()+1 {
            let c = memo[i][j-i][needle.len()];
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
        println!("{:?}", pair);
        assert!(pair == (6, 11));
    }
}

