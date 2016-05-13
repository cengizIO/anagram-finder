use std::collections::HashMap;

pub fn anagrams_for<'a>(source: &str, inputs: &'a[&'a str]) -> Vec<&'a str> {
    let anagrams = process_inputs(inputs);
    let source_key = construct_key_from_word(&source);

    match anagrams.get(&source_key) {
        Some(list) => list.clone(),
        None => vec![],
    }
}

fn process_inputs<'a>(inputs: &'a [&'a str]) -> HashMap<String, Vec<&'a str>> {
    let mut anagrams = HashMap::new();

    for word in inputs {
        let word_key = construct_key_from_word(word);
        let mut words_list = anagrams.entry(word_key).or_insert(Vec::new());
        words_list.push(*word);
    }

    anagrams
}

fn construct_key_from_word(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars.dedup();
    chars.into_iter().collect()
}
