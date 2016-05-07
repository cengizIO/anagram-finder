pub fn anagrams_for(source: &str, inputs: &[&str]) -> Vec<String> {
	inputs
		.into_iter()
		.filter(|x| x.len() == source.len())
		.map(|x| x.to_string())
		.collect::<Vec<_>>()
}

