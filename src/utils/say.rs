pub fn hello(s: Option<String>) {
    println!("Hello, {}, {}", s.unwrap_or("YOU DID NO SPECIFY ANY ARGS BITCH!".to_string()), "this is the second argument");
}