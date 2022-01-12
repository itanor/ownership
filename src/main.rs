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

    // slices...
    let phrase = String::from("some phrase to be processed");
    let first_word_of_phrase = first_word(&phrase);
    println!("{}", first_word_of_phrase);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word_slice(&my_string[0..6]);
    let _word = first_word_slice(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word_slice(&my_string);

    // a string literal is a slice
    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word_slice(&my_string_literal[0..6]);
    let _word = first_word_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word_slice(my_string_literal);
    
    integer_slice();
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

// slices (with *String type as param)
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// slices (with string slices as param)
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn integer_slice() {
    let a = [1,2,3,4,5];

    let slice = &a[1..a.len()];
    match slice.iter().nth(4) {
        Some(val) => println!("{}", val),
        None      => println!("without value"),
    };

    println!("{:?}", slice);
}

