// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let trimmed_s1 = s1.trim();
    let trimmed_s2 = s2.trim();

    if trimmed_s1.chars().count() > trimmed_s2.chars().count() {
        return Some(s1);
    } else if trimmed_s1.chars().count() < trimmed_s2.chars().count() {
        return Some(s2);
    }
    None
}
