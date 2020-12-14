use serde::{Serialize, Deserialize};

fn main() {
    let mut b_tree = BTree::new(95);
    b_tree.add_value(11);
    b_tree.add_value(111);
    b_tree.add_value(112);
    b_tree.add_value(2);
    b_tree.add_value(12);
    b_tree.add_value(60);
    //b_tree.reverse();
    //let asd = b_tree.root_node.unwrap().left.unwrap().left.unwrap().value;
    println!("{:?}", b_tree);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BNodei32 {
    pub value: i32,
    pub left: Option<Box<BNodei32>>,
    pub right: Option<Box<BNodei32>>
}


impl BNodei32 {
    pub fn new(value: i32) -> Self {
        BNodei32 { value: value, left: None, right: None }
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
        //self.left.unwrap().swap();
        match &mut self.left {
            None => {},
            Some(left) => left.swap()
        }

        match &mut self.right {
            None => {},
            Some(right) => right.swap()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BTree {
    pub root_node: Option<Box<BNodei32>>,
    pub count: usize
}

impl BTree {
    pub fn add_value(&mut self, value: i32) {
        let root_node_opt = &mut self.root_node;
        let root_node = match root_node_opt {
            None => panic!("what what what???"),
            Some(node_ref) => node_ref
        };
        self.count += 1;
        add_value_to_tree_node_box(root_node, value);
    }

    pub fn new(initial_value: i32) -> Self {
        let root_node = Box::new(BNodei32 { value: initial_value, left: None, right: None }) ;
        BTree {root_node: Some(root_node), count: 0}
    }

    pub fn reverse(&mut self) {
        match &mut self.root_node {
            None => {},
            Some(root_node) => root_node.swap()
        }
    }
}


// Add
pub fn add_value_to_tree_node_box(node_box:&mut Box<BNodei32>, value: i32) {
    match node_box.value > value {
        true => {
            match &mut node_box.left {
                None => {
                    println!("adding value left: {}", &value);
                    node_box.left = Some(Box::new(BNodei32::new(value)))
                
                },
                Some(l) => {                
                    add_value_to_tree_node_box(l, value)
                }
            }
        },
        false => {
            match &mut node_box.right {
                None => {
                    println!("adding value right: {}", &value);
                    node_box.right = Some(Box::new(BNodei32::new(value)))
                },
                Some(r) => {
                    add_value_to_tree_node_box(r, value)
                }
            }
        }
    }
}