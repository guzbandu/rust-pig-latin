use std::io;

fn main() {
    let mut sentence = String::from("start");
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    while sentence != "q".to_string() {	
        sentence = "".to_string();
        println!("Enter a sentence and it will be converted to Pig Latin!");
        println!("Type q to quit!");
        
        io::stdin()
             .read_line(&mut sentence)
             .expect("Failed to read line");

        sentence = sentence.trim().to_string();
        
        let mut pig_latin = String::new();
        if sentence != "q" {
            let mut first_word = true;
            for word in sentence.split(" ") {
                if first_word == true {
                    first_word = false;
                } else {
                    pig_latin.push_str(" ");
                }
                let char_array: Vec<char> = word.chars().collect();
                let first_char = char_array[0];
                let lower_first_char_array: Vec<char> = first_char.to_lowercase().collect();
                let lower_first_char = lower_first_char_array[0];
                //println!("{word} {first_char}");
                let mut consonent = true;
                for vwl in &vowels {
                    //println!("{vwl}");
                    if *vwl == lower_first_char {
                        consonent = false;
                        //println!("{first_char} is a vowel!!");
                    }
                }
                //println!("{first_char} is a consonent {consonent}");
                if consonent == true {
                    pig_latin.push_str(&word[1..]);
                    pig_latin.push_str("-");
                    pig_latin.push_str(&first_char.to_string());
                    pig_latin.push_str("ay");
                } else {
                    pig_latin.push_str(word);
                    pig_latin.push_str("-");
                    pig_latin.push_str("hay");
                }
            }
        }

        //println!("You entered:");
        //println!("{:?}",sentence);
        if sentence != "q" {
            println!("In Pig Latin that is:");
            println!("{:?}",pig_latin);
        }
    }
}
