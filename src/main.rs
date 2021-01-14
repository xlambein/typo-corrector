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

    let mut best_cost = usize::MAX;
    let mut best_length = usize::MAX;
    let mut best_pair = (usize::MAX, usize::MAX);

    for i in 0..haystack.len() {
        let mut memo = vec![vec![usize::MAX;needle.len()+1];haystack.len()+1];

        // Initialize memo
        for la in 0..haystack.len()+1 {
            memo[la][0] = la;
        }
        for lb in 0..needle.len()+1 {
            memo[0][lb] = lb;
        }

        // Compute memo
        // TODO prune
        for la in 1..haystack.len()-i+1 {
            for lb in 1..needle.len()+1 {
                if haystack[i + la - 1] == needle[lb - 1] {
                    memo[la][lb] = memo[la-1][lb-1];
                } else {
                    memo[la][lb] = 1 + cmp::min(
                        cmp::min(
                            memo[la-1][lb],
                            memo[la][lb-1]
                        ),
                        memo[la-1][lb-1]
                    );
                }
            }

            // Check if this is the best cost
            // For now, we only check at (sort-of) word boundaries
            if (i == 0 || haystack[i-1] == " ") && (i + la == haystack.len() || haystack[i + la] == " ") {
                let c = memo[la][needle.len()];
                if c < best_cost || (c == best_cost && la < best_length) {
                    best_cost = c;
                    best_length = la;
                    best_pair = (i, i+la);
                }
            }
        }
    }

    best_pair
}

mod tests {
    use crate::*;

    #[test]
    fn test_closest_substring() {
        let pair = closest_substring("Hello warld !", "world");
        println!("{:?}", pair);
        assert!(pair == (6, 11));

        let pair = closest_substring("Suub string", "Sub");
        println!("{:?}", pair);
        assert!(pair == (0, 4));

        let pair = closest_substring("This is the nd", "end");
        println!("{:?}", pair);
        assert!(pair == (12, 14));

        let pair = closest_substring("I didn't this so!", "think");
        println!("{:?}", pair);
        assert!(pair == (9, 13));
    }

    #[test]
    #[ignore]
    fn test_closest_substring_fails() {
        // Word boundaries ignore punctuations
        let pair = closest_substring("Hello warld!", "world");
        println!("{:?}", pair);
        assert!(pair == (6, 11));

        // Replacing in the middle of a word
        let pair = closest_substring("Sbstring", "Sub");
        println!("{:?}", pair);
        assert!(pair == (0, 2));
    }
}

