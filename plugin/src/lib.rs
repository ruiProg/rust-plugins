use loader::{Plugin, Foo, Bar};

struct PluginImpl;

impl Plugin for PluginImpl {
    fn handle_command(&self, command: Foo) -> Bar {
        match command {
            Foo::A(Some(x)) => println!("Got some a with value: {}", x),
            Foo::B{data: ch} => println!("Got b with char: {}", ch),
            Foo::C => println!("C ftw"),
            _ => println!("Unknown data"),
        }
        Bar{i: true, j: "Fado"}
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
extern "C" fn get_plugin() -> *mut dyn Plugin {
    println!("Running pluginImpl");

    // Return a raw pointer to an instance of our plugin
    Box::into_raw(Box::new(PluginImpl {}))
}

