use libloading::{Library, Symbol, library_filename};
use loader::{Plugin, Foo, Bar};
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let plugin_path = {
        let plugins_path = Path::new("target/debug");
        plugins_path.join(library_filename("plugin"))
    };
 
    let lib = unsafe { Library::new(plugin_path)? };
    let plugin = unsafe {
        let plugin_api: Symbol<extern fn() -> *mut dyn Plugin> = lib.get(b"get_plugin")?;
        Box::from_raw(plugin_api())
    };

    let payload = Foo::A(Some(5));
    let Bar{i, j} = plugin.handle_command(payload);
    
    println!("Got i: {} and j: {}", i, j);
    Ok(())
}
