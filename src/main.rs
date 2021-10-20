// use std::env::{ args };
// use std::path;


fn test_calculate_two_number<F>(one: i32, two:i32, fun:F) -> i32 
where F: Fn(i32) -> i32 {
    fun(one + two)
}

// struct Cli {

//     pattern: String,
//     path: std::path::PathBuf,
// }


fn main() {

    println!("Testing future CLI !!!");
    
    println!(
        "The result of one number plus anothers is : {}", 
        test_calculate_two_number(10,11, |one|{ one + 1 })
    );

    // let pattern = args().nth(1).expect("no pattern given");
    // let path = args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern: pattern,
    //     path: path::PathBuf::from(path),
    // };
    just_say_hello();

}

fn just_say_hello(){

    println!("Hey there may dude!!");
    get_primos(100);
}



fn get_primos(from:i32){
    for n in 1.. from{
        if n % 2 == 0 {
            println!("{}",n);
        }
    }

}