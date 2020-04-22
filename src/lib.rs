#[cfg(test)]
mod tests {
    use crate::BlinkTree;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn blink_tree_insert() {
        let blink = BlinkTree::new();
    }
}

struct BlinkTree {
    root: Node
}

impl BlinkTree {
    pub fn new() -> Self {}
}

const SIZE_OF_NODE: i32 = 1024;

struct Node {
    level: u32,

    // todo : to have below variables.
    // node lock
    // data : (key, value) pairs
}

struct NonLeafNode {}

struct LeafNode {
    // indicate this is a leaf node or not
    marker: bool
}

struct NodeData {
    // data:
}

impl Node {
    fn lock(&self) {}

    fn unlock(&self) {}
}

