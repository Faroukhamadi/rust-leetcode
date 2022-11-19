mod a132_pattern_solution;
mod balanced_binary_tree_solution;

fn main() {
    let my_str = String::from("Hello world");

    for c in my_str.into_bytes() {
        println!("c is {}", c as char);
    }

    for i in 1..=10 {
        println!("This is a basic for loop: {}", i);
    }
}
