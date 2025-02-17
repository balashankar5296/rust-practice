use std::io;
fn main() {
    println!("Enter the first operand");
    let input1:i32=io::stdin().lines().next().unwrap().unwrap().parse().unwrap();
    println!("Enter the second operand");
    let input2:i32=io::stdin().lines().next().unwrap().unwrap().parse().unwrap();
    println!("Enter the operation you want to perform");
    let sum:i32;
    let mut operator:String=String::new();
    io::stdin().read_line(&mut operator).ok().expect("failed to read line");

    match &operator{
        addition=>{println!("{}",input1+input2)},
        subtraction=> {println!("{}", input1-input2)},
        multiplication=>{
                        println!("{}",input1*input2);
                        }
    }
    
    
  


}
