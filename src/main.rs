use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");

    // new rand
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}",secret_number);

    loop{
        // define guess:String
        println!("\nPlease input yor guess.");
        let mut guess = String::new();

        // input from keyboard
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert u32
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You geussed: {}",guess);

        // compare
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Yourwin!");
                break;
            }
        }
    }
}
