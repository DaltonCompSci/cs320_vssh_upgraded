use std::env;
use std::path::Path;
use std::io::{stdout, stdin, Write};
use nix::unistd::{fork, ForkResult,execvp};
use nix::sys::wait::waitpid;
use std::ffi::CString;


fn main() -> std::io::Result<()> {
    println!("Please give an input");
    println!("Type 'exit' to end the program, cd [dir] to change directories,\
    or type another command");

    loop {
        let path = env::current_dir()?;
        println!("The current directory is {}", path.display());
         let usr = get_input("String");
         if usr == "exit" {
             break;
         }
         else if usr.starts_with("cd"){
             let path = Path::new(usr.split_whitespace().skip(1).next().unwrap());
             let changed_dir = env::set_current_dir(&path).is_ok();
         } else {
             run(usr.as_str())
         }
        }
    Ok(())
}
fn get_input(prompt: &str) -> String {
    print!("{} ", prompt);
    stdout().flush().expect("Can't flush");
    let mut line = String::new();
    stdin().read_line(&mut line).expect("trouble!");
    line
}
fn run(command: &str) {
    match unsafe {fork()}.unwrap() {
        ForkResult::Parent {child} => {
            println!("The Parent is running fine!");
            waitpid(child, None).unwrap();
            println!("the child has finished!")
        }
        ForkResult::Child => {
            println!("The child is running fine!");
            let externalized = externalize(command);
            execvp(&externalized[0], &externalized).unwrap();
        }
    }
}
fn externalize(command: &str) -> Box<[CString]> {
    let converted = command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect::<Vec<_>>();
    converted.into_boxed_slice()
}