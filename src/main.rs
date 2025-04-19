use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {

    let tup = (500.0, 6, "1");

    let (a,_b,_c) = tup;
    
    println!("{}", a);

    let y= {
        let x = 5;
        x+3
    };
    println!("{:?}",y);
    println!("{}",val_return(20));
    guessing_game();

}

fn val_return(a:i32) -> i32 {
    a +55
}


fn guessing_game(){
    println!(" gues a number");
    println!(" input a guess");

    let number_rand = rand::thread_rng().gen_range(1..=100);

    loop{


        let mut guess: String = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("failure");

        let guess : u32 = match guess.trim()
        .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!(" you guessed: {}", guess);
        println!(" sectre number: {}", number_rand);

        
        match guess.cmp(&number_rand) {
            Ordering::Less => println!("samll"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("correct");
                break;
            }
        }
    }
    println!("exiting game!");
}
