pub fn reverse(input: &str) -> String {
    let mut r_string=String::new();
    for c in input.chars().rev()
        {
            r_string.push(c);
        }
    r_string  
}
