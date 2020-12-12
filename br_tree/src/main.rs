fn main() {
    let b_tree = BTree::new(95);


}

struct BNodei32 {
    pub value: i32,
    pub left: Option<Box<BNodei32>>,
    pub right: Option<Box<BNodei32>>
}

struct BTree {
    pub root_node: Option<BNodei32>
}

impl BTree {
    pub fn add_value(&self, value: i32) {

    }

    pub fn new(initial_value: i32) -> Self {
        let root_node = BNodei32 { value: initial_value, left: None, right: None };
        BTree {root_node: Some(root_node)}
    }
}