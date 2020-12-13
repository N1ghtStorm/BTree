fn main() {
    let mut b_tree = BTree::new(95);
    b_tree.add_value(11);
    b_tree.add_value(111);
    b_tree.add_value(112);
    b_tree.add_value(2);
    b_tree.add_value(12);
    b_tree.add_value(60);
    //println!("{}",b_tree.count);
    let asd = b_tree.root_node.unwrap().left.unwrap().left.unwrap().value;
    println!("{}", asd);
}

pub struct BNodei32 {
    pub value: i32,
    pub left: Option<Box<BNodei32>>,
    pub right: Option<Box<BNodei32>>
}

impl BNodei32 {
    pub fn new(value: i32) -> Self {
        BNodei32 { value: value, left: None, right: None }
    }
}

pub struct BTree {
    pub root_node: Option<Box<BNodei32>>,
    pub count: usize
}

impl BTree {
    pub fn add_value(&mut self, value: i32) {
        let root_node_opt = &mut self.root_node;
        let root_node = match root_node_opt {
            None => panic!("what what what??"),
            Some(node_ref) => node_ref
        };
        self.count += 1;
        add_value_to_tree_node_box(root_node, value);
    }

    pub fn new(initial_value: i32) -> Self {
        let root_node = Box::new(BNodei32 { value: initial_value, left: None, right: None }) ;
        BTree {root_node: Some(root_node), count: 0}
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
                    //println!("{}", left.value);
                    //let a = &mut node_box.left;
                    //let b= a.unwrap();
                
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
                    //println!("{}", left.value);
                    add_value_to_tree_node_box(r, value)
                }
            }
        }
    }
}