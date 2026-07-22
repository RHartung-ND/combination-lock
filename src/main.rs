use std::io;

fn main() {
    println!("Input your combination:");
    print!("==> ");

    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{input}");
        }
        Err(error) => println!("error: {error}"),
    }
}
