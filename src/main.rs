fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);

    let s3 = String::from("on");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    let some_str = String::from("str");
    takes_ownership(some_str);

    let s = String::from("hhh");
    let (s5, len) = calculate_length(s);
    println!("The length of '{}' is {}", s5, len);

    let a = String::from("aaabbbccc");
    let len = calculate_len(&a);
    println!("The length of '{}' is {}", a, len);

    let mut mut_string = String::from("mutable hello");
    mutable_reference(&mut mut_string);
    println!("string mutated: {}", mut_string);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn mutable_reference(some_string: &mut String) {
    some_string.push_str(", world");
}

