use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hash_to_words: HashMap<String, Vec<String>> = HashMap::new();

    for str in strs {
        let freqs: Vec<u8> = {
            let mut tmp: Vec<u8> = vec![0; 26];
            for c in str.chars() {
                let idx = c as u8 - 'a' as u8;
                tmp[idx as usize] += 1
            }
            tmp
        };

        let hash = format!("{:?}", freqs);
        hash_to_words.entry(hash).or_insert(vec![]).push(str)
    }
    hash_to_words.values().cloned().collect()
}
