use std::io;
use std::cmp::Ordering;
use rand::Rng;
// use std::str;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}

#[derive(Debug)]
enum SubscriberModel {
    Basic,
    Silver,
    Gold,
    Diamond
}

fn main() {
    
    let user1 = SubscriberModel::Gold;
    println!("{}", type_of_subscriber(user1));
   
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(&rect1);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    

    let this_num : &u32 = &12;

    let this_num2 = add_mutable_uint(this_num);


    println!("first num {}, and second num {}", this_num, this_num2);




    let tup = (500.0, 6, "1");

    let (a,_b,_c) = tup;
    
    println!("{}", a);

    let y= {
        let x = 5;
        x+3
    };

    for num in 0..y+1 {
        println!("{num}");
    }
    println!("finsihed loop");
    println!("{:?}",y);
    println!("{}",val_return(20));
    println!("{}", tertiary(false));
    guessing_game();

}

fn val_return(a:i32) -> i32 {
    a +55
}

fn tertiary(x:bool) -> String {
    let mut x : String = if x {String::from("this is true")} else { String::from("false my guy")};
    x.push_str("!");
    x
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


fn add_mutable_uint(x: &u32) -> u32{
    x + 30
}


fn type_of_subscriber(sub : SubscriberModel) -> String{
    match sub {
        SubscriberModel::Basic => String::from("you are basic"),
        SubscriberModel::Silver => String::from("Silver kid"),
        SubscriberModel::Gold=> String::from("stay golden Pony Boy"),
        SubscriberModel::Diamond => String::from("shine bright like a dimond")
    }
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
