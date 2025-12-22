use std::io;
use std::process::Command;
use std::env;


pub fn main() -> Result<(), io::Error> {
    let port = 5555;
    if !create_rust_app::net::is_port_free(port) {
        println!("========================================================");
        println!(" ViteJS (the frontend compiler/bundler) needs to run on");
        let line_three = format!("Port {} but it seems to be in use.", port);
        println!("{line_three}");
        println!("========================================================");
        let line_five = format!("Port {} is taken but is required for development!", port);
        panic!("{line_five}")
    }
    let path = env::current_dir()?;
    println!("{}",path.display());
    Command::new("yarn")
        .arg("fullstack")
        .current_dir("frontend")
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    Ok(())
}
