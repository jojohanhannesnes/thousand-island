pub fn hello_world() -> &'static str {
    "Hello, World!"
}

#[test]
fn it_works() {
    assert_eq!("Hello, World!", hello_world());
}
