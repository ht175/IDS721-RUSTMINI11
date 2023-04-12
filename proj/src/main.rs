use std::collections::HashMap;

fn main() {
    let translations = create_translation_mappings();

    // Input English text to be translated
    let input_text = "Hello, how are you?";

    // Translate the input text
    let translated_text = translate_text(&input_text, &translations);

    println!("Input Text: {}", input_text);
    println!("Translated Text: {}", translated_text);
}

fn create_translation_mappings() -> HashMap<String, String> {
    // Create a HashMap to store translation mappings
    let mut translations = HashMap::new();

    // Add translation mappings for English to Chinese
    translations.insert(String::from("Hello"), String::from("你好"));
    translations.insert(String::from("how"), String::from("怎么"));
    translations.insert(String::from("are"), String::from("是"));
    translations.insert(String::from("you"), String::from("你"));

    translations
}



fn translate_text(input_text: &str, translations: &HashMap<String, String>) -> String {
    // Split input text into words
    let words: Vec<&str> = input_text.split_whitespace().collect();

    // Translate each word using the translation mappings
    let mut translated_words = Vec::new();
    for word in words {
        let word = word.trim_matches(|c: char| !c.is_alphanumeric());
        
        if let Some(translated_word) = translations.get(word) {
            translated_words.push(translated_word.to_string());
        } else {
            translated_words.push(word.to_string());
        }
    }

    // Join translated words back into a single string
    translated_words.join(" ")
}
