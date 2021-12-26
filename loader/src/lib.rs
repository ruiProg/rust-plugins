pub enum Foo {
    A(Option<i32>),
    B{ data: char},
    C,
}

pub struct Bar<'a> {
    pub i: bool,
    pub j: &'a str,
}

pub trait Plugin {
    fn handle_command(&self, command: Foo) -> Bar;
}
