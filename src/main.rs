use std::io;
fn main() {
    println!("Enter the first operand");
    let input1: i32 = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();
    println!("Enter the second operand");
    let input2: i32 = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();
    println!("Enter the operation you want to perform");
    let result: i32;
    let mut operator: String = String::new();
    io::stdin()
        .read_line(&mut operator)
        .ok()
        .expect("failed to read line");
    operator=operator.trim().to_ascii_lowercase();

     match operator.as_str() {
        "addition" => {
            result = input1 + input2;
            println!("{}", result);
        }
        "subtraction" => {
            let result = input1 + input2;
            println!("{}", result);
        }
        "multiplication" => {
            let result = input1 + input2;
            println!("{}", result);
        }
        _ =>{
            println!("operation can't be performed");
        }
    }
    println!("End of program");
}
