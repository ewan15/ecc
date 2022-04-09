use crate::tokens::AstState;
use crate::tokens::AstState::Root;

pub struct Tree {
    root: NodeHandle,
    pub(crate) nodes: Vec<Node>,
    count: usize,
}

pub(crate) type NodeHandle = usize;

pub struct Node {
    pub(crate) data: AstState,
    pub(crate) child: Vec<NodeHandle>,
    parent: NodeHandle,
}

impl Tree {
    pub fn new() -> Self {
        let mut s = Self {
            count: 1,
            root: 0,
            nodes: vec![],
        };
        s.nodes.push(
            Node{data: Root,
                child: vec![],
                parent: 0
            });
        s
    }

    /// Returns the node count of the tree.
    pub fn node_count(&self) -> usize {
        assert!(self.count != 0 || self.root == 0);
        self.count
    }

    // Insert a new item into the subtree rooted at `atnode`.
    pub fn insert_at(&mut self, atnode: NodeHandle, data: AstState) -> Option<NodeHandle> {
        let node_handle = self.alloc_node(data, atnode);
        self.nodes[atnode].child.push(node_handle);
        Some(node_handle)
    }

    pub fn parent_handle(&mut self, atnode: NodeHandle) -> NodeHandle {
        self.nodes[atnode].parent
    }

    // Allocates a new node in the tree and returns its handle.
    fn alloc_node(&mut self, data: AstState, parent: NodeHandle) -> NodeHandle {
        self.nodes.push(Node::new_with_parent(data, parent));
        self.nodes.len() - 1
    }

    pub fn at(&self, atnode: NodeHandle) -> &Node { &self.nodes[atnode] }

    // pub fn find(&self, data: AstState) -> bool {
    //     self.find_node(self.root, data).is_some()
    // }

    // Find a node with the given item starting from `fromnode`. Returns None if
    // `fromnode` is 0.
    // fn find_node(&self, fromnode: NodeHandle, data: AstState) -> Option<NodeHandle> {
    //     if fromnode == 0 {
    //         return None;
    //     }
    //
    //     let node = &self.nodes[fromnode];
    //     if node.data == data {
    //         Some(fromnode)
    //     } else if data < node.data {
    //         match node.left {
    //             0 => None,
    //             _ => self.find_node(node.left, data),
    //         }
    //     } else {
    //         match node.right {
    //             0 => None,
    //             _ => self.find_node(node.right, data),
    //         }
    //     }
    // }
    //
    // /// Returns a string representation of the tree for debugging.
    // pub fn display(&self) -> String {
    //     self.display_node(self.root, 0)
    // }
    //
    // // Returns a string representation of the `node` subtree, with an initial
    // // indentation level.
    // fn display_node(&self, fromnode: NodeHandle, indent: usize) -> String {
    //     let indent_str = " ".repeat(indent);
    //     if fromnode == 0 {
    //         indent_str + ".\n"
    //     } else {
    //         let mut s = format!("{}{}\n", indent_str, self.nodes[fromnode].data);
    //         s.push_str(&self.display_node(self.nodes[fromnode].left, indent + 2));
    //         s.push_str(&self.display_node(self.nodes[fromnode].right, indent + 2));
    //         s
    //     }
    // }
    //
    // /// Returns all data of the tree, visited inorder.
    // pub fn inorder(&self) -> Vec<AstState> {
    //     let mut v = vec![];
    //     if self.root != 0 {
    //         let mut node = self.leftmost_child(self.root);
    //         loop {
    //             v.push(self.nodes[node].data);
    //             match self.successor_of_node(node) {
    //                 Some(succ) => node = succ,
    //                 _ => break,
    //             };
    //         }
    //     }
    //     v
    // }

    // pub fn remove(&mut self, data: AstState) -> bool {
    //     if let Some(node) = self.find_node(self.root, data) {
    //         self.remove_node(node);
    //         self.count -= 1;
    //         true
    //     } else {
    //         false
    //     }
    // }

    // pub fn successor(&self, data: AstState) -> Option<AstState> {
    //     if let Some(node) = self.find_node(self.root, data) {
    //         if let Some(succ) = self.successor_of_node(node) {
    //             return Some(self.nodes[succ].data);
    //         }
    //     }
    //     None
    // }

    // // Find the successor of `node` in the tree.
    // fn successor_of_node(&self, node: NodeHandle) -> Option<NodeHandle> {
    //     match self.nodes[node].right {
    //         0 => self.parent_with_left(node),
    //         _ => Some(self.leftmost_child(self.nodes[node].right)),
    //     }
    // }
    //
    // // Find the leftmost child of `node`, or `node` itself in case it has no
    // // left child.
    // fn leftmost_child(&self, node: NodeHandle) -> NodeHandle {
    //     match self.nodes[node].left {
    //         0 => node,
    //         _ => self.leftmost_child(self.nodes[node].left),
    //     }
    // }

    // Find a parent in `node`'s ancestor chain that is reached through its
    // left child. For example, in the tree:
    //
    //         9
    //        / \
    //       4   11
    //      /
    //     2
    //      \
    //       3
    //
    // if `node` is 3, this will find 4, because 2 is reached through its right
    // child in the chain. If node is 2, it will find 4 also. If node is is 11,
    // it is None.
    // fn parent_with_left(&self, node: NodeHandle) -> Option<NodeHandle> {
    //     let parent = self.nodes[node].parent;
    //     if parent != 0 {
    //         if self.nodes[parent].left == node {
    //             Some(parent)
    //         } else {
    //             self.parent_with_left(parent)
    //         }
    //     } else {
    //         None
    //     }
    // }

    // // Remove the given node from the tree.
    // fn remove_node(&mut self, node: NodeHandle) {
    //     let lchild = self.nodes[node].left;
    //     let rchild = self.nodes[node].right;
    //     if lchild == 0 && rchild == 0 {
    //         // Node has no children, so it's safe to dispose.
    //         self.replace_node(node, 0);
    //     } else if lchild != 0 && rchild != 0 {
    //         // Node has both children.
    //         // We find the successor of this node, replace our node's data with
    //         // its data and then recursively remove the successor.
    //         if let Some(succ) = self.successor_of_node(node) {
    //             self.nodes[node].data = self.nodes[succ].data;
    //             self.remove_node(succ);
    //         }
    //     } else if lchild != 0 {
    //         // Node has only left child, so replace it with its only child.
    //         self.replace_node(node, lchild);
    //     } else if rchild != 0 {
    //         // Node has only right child, so replace it with its only child.
    //         self.replace_node(node, rchild);
    //     } else {
    //         panic!("unreachable");
    //     }
    // }

    // Replaces `node` with `r` in the tree, by setting `node`'s parent's
    // left/right link to `node` with a link to `r`, and setting `r`'s parent
    // link to `node`'s parent.
    // Note that this code doesn't actually deallocate anything. It just
    // makes self.nodes[node] unused (in the sense that nothing points to
    // it).
    // fn replace_node(&mut self, node: NodeHandle, r: NodeHandle) {
    //     let parent = self.nodes[node].parent;
    //     // Set the parent's appropriate link to `r` instead of `node`.
    //     if parent != 0 {
    //         if self.nodes[parent].left == node {
    //             self.nodes[parent].left = r;
    //         } else if self.nodes[parent].right == node {
    //             self.nodes[parent].right = r;
    //         }
    //     } else {
    //         self.root = r;
    //     }
    //     // r's parent is now node's parent.
    //     if r != 0 {
    //         self.nodes[r].parent = parent;
    //     }
    // }
}

impl Node {
    fn new(data: AstState) -> Self {
        Self {
            data,
            child: vec![],
            parent: 0,
        }
    }

    fn new_with_parent(data: AstState, parent: NodeHandle) -> Self {
        Self {
            data,
            child: vec![],
            parent,
        }
    }
}