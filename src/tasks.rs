// Goal: Write a function calculate_length that takes an immutable reference to a String and returns its length. Then call this function from main and print both the original String and its length.

pub fn run(name: String) {
    calculate_length(&name);
}

fn calculate_length(s: &String) {
    println!("the original string is: {}", s);
    println!("Length of string is: {}", s.len());
}

//Goal: Write a function append_text that takes a mutable reference to a String and appends some text to it. For example, if the string is "Hello", the function could append ", World!".

pub fn append_text(s: &mut String) -> () {
    s.push_str(" Singla");
    println!("this is the string after appending the last name to it {}", s)
}