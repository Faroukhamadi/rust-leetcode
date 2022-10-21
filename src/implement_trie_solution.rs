#[derive(Default)]
struct TrieNode {
    next: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

#[derive(Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;
        for ch in word.bytes() {
            let i = (ch - b'a') as usize;
            let next = &mut cur.next[i];
            cur = next.get_or_insert_with(Box::<TrieNode>::default);
        }
        cur.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        self.get_trie_node(word).map_or(false, |cur| cur.is_end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.get_trie_node(prefix).is_some()
    }

    fn get_trie_node(&self, prefix: String) -> Option<&TrieNode> {
        let mut cur = &self.root;
        for ch in prefix.bytes() {
            let i = (ch - b'a') as usize;
            match cur.next[i].as_ref() {
                Some(next) => cur = next,
                None => return None,
            }
        }
        return Some(cur);
    }
}
