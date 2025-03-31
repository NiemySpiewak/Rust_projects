use rand::prelude::*;
use std::io;

fn generate_password(n: usize, charsets: &[&str]) -> String {
    let mut chars: Vec<char> = Vec::new(); //tworzymy wektor, aby móc dodawć do niego elementy (dynamiczna tablica)

    let uppercase:[char;20] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','R','S','T','W'];
    let lowercase: [char;20] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p','r', 's', 't', 'w'];
    let special: [char;10] = ['!','@','#','$','%','^','&','*','(',')'];
    let digits: [char;10] = ['1','2', '3', '4', '5', '6', '7', '8', '9', '0'];

    if charsets.contains(&"uppercase"){
        chars.extend(&uppercase);
    }
    if charsets.contains(&"lowercase"){
        chars.extend(&lowercase);
    }
    if charsets.contains(&"special"){
        chars.extend(&special);
    }
    if charsets.contains(&"digits"){
        chars.extend(&digits);
    }
    else{
        chars.extend(&uppercase);
        chars.extend(&lowercase);
        chars.extend(&special);
        chars.extend(&digits);
    }

    let mut rng: ThreadRng = rand::rng();
    let mut password: String = String::from("");
    for _ in 0..n
    {
        let rand_char = chars.choose(&mut rng).unwrap();
        password.push(*rand_char);
    }
    password

}

fn main()
{
    let mut input_number: String = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("failed to read from stdin");
    let x: usize = input_number.trim().parse().expect("Input not an integer");

    let mut input_text: String = String::new();
    io::stdin()
    .read_line(&mut input_text)
    .expect("failed to read from stdin");

    let mut input_parts= input_text.trim().split_whitespace();

    let charsets: [&str; 4] = [
        input_parts.next().unwrap_or(""),
        input_parts.next().unwrap_or(""),
        input_parts.next().unwrap_or(""),
        input_parts.next().unwrap_or(""),
    ];

    for _ in 0..5
    {
        let password = generate_password(x, &charsets);
        println!("{}",password);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_chars() {
        let length: usize = 12; 
        let password = generate_password(length,&[&"lowercase"]);
        assert_eq!(password.len(), length);
    }
    #[test]
    fn if_right_chars(){
        let length: usize = 8;
        let password = generate_password(length,&[&"lowercase"]); 
        assert!(password.chars().any(|c| c.is_lowercase()));
    }
}