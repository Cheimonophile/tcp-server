mod server;

use std::thread;
use std::io;
use std::io::BufRead;


fn main() -> io::Result<()> {

    // spawns the thread with the server
    thread::spawn(server::start);

    // handles input from the shell
    for line in io::stdin().lock().lines(){
        let line = line?;

        // executes a command depending on
        match line.as_str() {
            "/exit" => break,
            _ => (),
        }
    }

    return Ok(());
}
