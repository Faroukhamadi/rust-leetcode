mod contains_duplicate_solution;
mod group_anagrams_solution;
mod implement_trie_solution;
mod longest_substring_without_repeating_characters_solution;

fn main() {
    let s: String = String::from("hello world");
    for c in s.into_bytes() {
        println!("c is: {}", c);
    }
}
