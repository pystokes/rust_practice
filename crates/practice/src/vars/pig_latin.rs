pub fn practice_pig_latin() {
    let sentence = String::from("One Hamburger and a Coffee please");
    
    let mut ret_str = String::new();
    for word in sentence.split_whitespace() {

        let first_char = word.chars().next().unwrap();

        let sub_str = word.chars().skip(1).take(word.len()-1).collect::<String>();
        match &*first_char.to_lowercase().to_string() {
            "a" | "i" | "u" | "e" | "o" => ret_str = ret_str + " " + &*word + "-hay",
            _ => ret_str = ret_str + " " + &sub_str + "-" + &first_char.to_string() + "ay",
        }
    }
    println!("[Pig Latin] {}", ret_str);
}
