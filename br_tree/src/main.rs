fn main() {
    let mut b_tree = BTree::new(95);
    b_tree.add_value(11);
    b_tree.add_value(111);
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
    pub root_node: Option<Box<BNodei32>>
}

impl BTree {
    pub fn add_value(&mut self, value: i32) {
        let root_node_opt = &mut self.root_node;
        let root_node = match root_node_opt {
            None => panic!("what what what??"),
            Some(node_ref) => node_ref
        };
        add_value_to_tree_node_box(root_node, value);
        // match root_node.value > value {
        //     true => {
        //         match &root_node.left {
        //             None => root_node.left = Some(Box::new(BNodei32::new(value))),
        //             Some(left) => {
        //                 println!("{}", left.value);
        //             }
        //         }
        //     },
        //     false => {
        //         println!("asdsadad")
        //     }
        // }
    }

    fn compare_node_and_value(node: BNodei32, ) {

    }

    pub fn new(initial_value: i32) -> Self {
        let root_node = Box::new(BNodei32 { value: initial_value, left: None, right: None }) ;
        BTree {root_node: Some(root_node)}
    }
}


// Add
pub fn add_value_to_tree_node_box(node_box:&mut Box<BNodei32>, value: i32) {
    match node_box.value > value {
        true => {
            match &node_box.left {
                None => {
                    println!("adding value left: {}", &value);
                    node_box.left = Some(Box::new(BNodei32::new(value)))
                
                },
                Some(_) => {
                    //println!("{}", left.value);
                    let a = &mut node_box.left;
                    //let b= a.unwrap();
                    //add_value_to_tree_node_box(&mut node_box.left.unwrap(), value)
                }
            }
        },
        false => {
            match &node_box.right {
                None => {
                    println!("adding value right: {}", &value);
                    node_box.right = Some(Box::new(BNodei32::new(value)))
                },
                Some(_) => {
                    //println!("{}", left.value);
                    //add_value_to_tree_node_box(&mut node_box.right.unwrap(), value)
                }
            }
        }
    }
}