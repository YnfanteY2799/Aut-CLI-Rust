// Imports
// use 


fn test_calculate_two_number(one: i32, two:i32, fun:fn) -> i32{
    return fun(one + two);
}

struct Cli {

    pattern: String,
    path: std::path::PathBuf,
}


fn main() {

    println!("Testing future CLI !!!");
    
    println!("The result of one number plus anothers is : {}", test_calculate_two_number(10,11));

    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };


}
