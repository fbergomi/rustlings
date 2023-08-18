//test1.rs

fn modify_string(s: &mut String) {
    s.push_str("_|_");
}

fn print_string(s: &String) {
    println!("{}", s);
}

fn main() {
    println!("hello");
    let mut s1 = String::from("hellololo");
    modify_string(&mut s1);
    print_string(&s1);


    modify_string(&mut s1);
    print_string(&s1);
}
