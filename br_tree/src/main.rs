fn main() {
    let b_tree = BTree::new(95);


}

struct BNodei32 {
    pub value: i32,
    pub left: Option<Box<BNodei32>>,
    pub right: Option<Box<BNodei32>>
}

struct BTree {
    pub root_node: Option<Box<BNodei32>>
}

impl BTree {
    pub fn add_value(&self, value: i32) {
        let root_node_opt = &self.root_node;
        let root_value = match root_node_opt {
            None => panic!("what what what??"),
            Some(asd) => asd.value
        };

        let uuu = match root_value > value {
            true => {},
            false => {println!("asdsadad")}
        };

    }

    fn compare_node_and_value(node: BNodei32, ) {

    }

    pub fn new(initial_value: i32) -> Self {
        let root_node = Box::new(BNodei32 { value: initial_value, left: None, right: None }) ;
        BTree {root_node: Some(root_node)}
    }
}