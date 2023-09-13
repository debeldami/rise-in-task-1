fn main() {
    let mut string1 = String::from("Hello ");
    let string2 = String::from("world");
    println!("{}", concatenate_strings(&mut string1, &string2));
}

fn concatenate_strings(string1: &mut String, string2: &String) -> String {
    let mut result = String::new();
    result.push_str(string1);
    result.push_str(string2);
    result
}
