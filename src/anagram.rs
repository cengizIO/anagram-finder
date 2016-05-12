use std::collections::HashMap;

pub fn anagrams_for(source: &str, inputs: &[&str]) -> Vec<String> {
    let anagrams = process_inputs(inputs);
    let source_key = construct_key_from_word(&source);

    match anagrams.get(&source_key) {
        Some(list) => list.clone(),
        None => vec![],
    }
}

fn process_inputs(inputs: &[&str]) -> HashMap<String, Vec<String>> {
    let mut anagrams = HashMap::<String, Vec<String>>::new();

    for word in inputs {
        let word_key = construct_key_from_word(word);
        let mut words_list = anagrams.entry(word_key).or_insert(Vec::<String>::new());
        words_list.push(String::from(word.clone()));
    }

    anagrams
}

fn construct_key_from_word(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars.dedup();

    let mut key_str = String::new();

    for c in chars {
        key_str.push(c);
    }

    key_str
}
