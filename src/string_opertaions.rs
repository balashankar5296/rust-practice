use std::io;
pub fn string_manipulation() {
    println!("Enter the string for manipulation");
    let mut sample: String = String::new();
    io::stdin()
        .read_line(&mut sample)
        .ok()
        .expect("failed to read line");
    sample = sample.trim().to_lowercase();
    println!("length of the substring {}", sample.len());
    let mut new_sample = String::new();
    for c in sample.chars().rev() {
        new_sample.push(c);
    }
    println!("reverse of string {}", new_sample);
    println!("all uppercase of string {}", sample.to_uppercase());
    println!("substring of the string {}", &sample[0..5]);
}
