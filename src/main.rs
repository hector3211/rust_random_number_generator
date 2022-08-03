use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Guess the number!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("ooops somethings wrong!");
        // Here we used (match) instead of expect, to handle an error if it occurs
        // match will grab entered value and just pass it as an Ok value
        // then if parse isnt able to do its job it will throw an ERROR
        // and we made it where our program just continues its loop
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        // almost think of (match) as like a switch in JS
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                // this breaks out of the loop
                break;
            }
        }
    }
}
