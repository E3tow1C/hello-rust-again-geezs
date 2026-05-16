use super::say;

pub fn cast(s: &str) {
    say::hello(Some(String::from(s)));
}