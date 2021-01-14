use std::io;
use std::io::Write;
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut last_message = "".to_owned();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Remove newline
                input = input[..input.len()-1].to_string();
                if input.starts_with("*") {
                    last_message = replace_closest(&last_message, &input[1..]);
                    println!("{}", last_message);
                } else {
                    last_message = input;
                }
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

    let haystack = UnicodeSegmentation::grapheme_indices(haystack, true).collect::<Vec<(usize, &str)>>();
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
                if haystack[i + la - 1].1 == needle[lb - 1] {
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
            if (i == 0 || haystack[i-1].1 == " ") && (i + la == haystack.len() || haystack[i + la].1 == " ") {
                let c = memo[la][needle.len()];
                if c < best_cost || (c == best_cost && la < best_length) {
                    best_cost = c;
                    best_length = la;
                    best_pair = (haystack[i].0, haystack[i+la-1].0+haystack[i+la-1].1.len());
                }
            }
        }
    }

    best_pair
}

fn replace_closest(haystack: &str, needle: &str) -> String {
    let slice = closest_substring(haystack, needle);
    haystack[..slice.0].to_owned() + needle + &haystack[slice.1..]
}

mod tests {
    use crate::*;

    #[test]
    fn test_closest_substring() {
        assert_eq!(closest_substring("Hello warld !", "world"), (6, 11));
        assert_eq!(closest_substring("Suub string", "Sub"), (0, 4));
        assert_eq!(closest_substring("This is the nd", "end"), (12, 14));
        assert_eq!(closest_substring("I didn't this so!", "think"), (9, 13));
        assert_eq!(closest_substring("Birb", "Bird"), (0, 4));
        assert_eq!(closest_substring("これは何です？", "これは何ですか？"), (0, 21));
    }

    #[test]
    fn test_replace_closest() {
        assert_eq!(replace_closest("Hello warld !", "world"), "Hello world !");
        assert_eq!(replace_closest("Suub string", "Sub"), "Sub string");
        assert_eq!(replace_closest("This is the nd", "end"), "This is the end");
        assert_eq!(replace_closest("I didn't this so!", "think"), "I didn't think so!");
        assert_eq!(replace_closest("Birb", "Bird"), "Bird");
        assert_eq!(replace_closest("これは何です？", "これは何ですか？"), "これは何ですか？");
    }

    #[test]
    #[ignore]
    fn test_closest_substring_fails() {
        // Word boundaries ignore punctuations
        assert_eq!(closest_substring("Hello warld!", "world"), (6, 11));
        // Replacing in the middle of a word
        assert_eq!(closest_substring("Sbstring", "Sub"), (0, 2));
    }
}

