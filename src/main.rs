fn main() {
    let result = is_balanced("({[][]})");
    let result2 = is_balanced("({{[)]}})");
    println!("{}", result);
    println!("{}", result2);
}

fn is_balanced(candidate_str: &str) -> bool {
    let mut vec = Vec::new();
    for c in candidate_str.chars() {
        match c {
            '[' | '(' | '{' => vec.push(c),
            ')' => if vec[vec.len() - 1] == '(' { vec.pop(); } else { return false; },
            '}' => if vec[vec.len() - 1] == '{' { vec.pop(); } else { return false; },
            ']' => if vec[vec.len() - 1] == '[' { vec.pop(); } else { return false; },
            _ => println!("Nothing")
        }
    }

    vec.len() == 0
}

#[test]
fn test_is_balanced() {
    assert_eq!(is_balanced("({[][]})"), true);
    assert_eq!(is_balanced("({{[)]}})"), false);
}
