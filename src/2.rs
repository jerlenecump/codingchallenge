
pub fn generate_random_code() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1..=10);
    match num {
        1 => println!("Hello World!"),
        2 => println!("I love to code!"),
        3 => println!("Coding is fun!"),
        4 => println!("Debugging is hard..."),
        5 => println!("But rewarding!"),
        6 => println!("Rust is the best language ever!"),
        7 => println!("Functional programming is the future!"),
        8 => println!("I'm a cat, hear me roar!"),
        9 => println!("Coding challenges are my favorite!"),
        10 => println!("I'm a Rustacean, hear me roar!"),
    }
}