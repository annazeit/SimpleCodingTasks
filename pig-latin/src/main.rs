pub enum CharType {
    Vowel,
    Consonant,
}

pub fn get_char_type(c: &char) -> CharType {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    for i in vowels {
        if *c == i {
            return CharType::Vowel;
        }
    }
    CharType::Consonant
}

pub fn pig_latin(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let first_letter = chars[0];

    let result = {
        match get_char_type(&first_letter) {
            CharType::Vowel => {
                format!("{s}-hay")
            },
            CharType::Consonant => {
                let last_index = chars.len();
                let cut_chars = &s[1..last_index];
                format!("{cut_chars}-{first_letter}ay")
            }
        }
    };
    result
}

fn main() {
    let word = "first";
    let result = pig_latin(word);
    println!("Result: {result}");
}

