mod utils;
use utils::say;
use utils::cast;

fn main() {
    say::hello(None);
    cast::cast(Some("Care".to_string()));
}
