
use std::collections::VecDeque;

/// A tree node
pub struct Node<T> {
    data: T,
    children: Vec<Node<T>>,
}

impl<T> Node<T> {

    ///
    /// Create a Node
    ///
    pub fn new(data: T) -> Self {
        Node {
            data,
            children: Vec::new(),
        }
    }

    ///
    /// Add a node to self
    ///
    pub fn add(&mut self, node: Node<T>) {
        self.children.push(node);
    }

    /// Get mut data
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Get data
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Get child by index
    /// 
    pub fn child(&self, index: usize) -> Option<&Node<T>> {
        self.children.get(index)
    }

    /// Get last child
    /// 
    pub fn last_child(&self) -> Option<&Node<T>> {
        self.children.last()
    }

    /// Get mut child by index
    /// 
    pub fn child_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        self.children.get_mut(index)
    }

    /// Get last mut child
    /// 
    pub fn last_child_mut(&mut self) -> Option<&mut Node<T>> {
        self.children.last_mut()
    }

    /// Get children
    pub fn children(&self) -> &Vec<Node<T>> {
        &self.children
    }

    /// Get mut children
    pub fn children_mut(&mut self) -> &mut Vec<Node<T>> {
        &mut self.children
    }

    ///
    /// Get child by a vec path
    /// path: [0, 1, 3]
    ///
    pub fn child_by_path(&self, path: &Vec<usize>) -> Option<&Node<T>> {
        let mut node: Option<&Node<T>> = Some(self);
        let level = path.len();
        for i in 1..level {
            if node.is_some() {
                node = node.unwrap().child(*path.get(i).unwrap());
            }
        }
        return node;
    }

    ///
    /// Get mut child by a vec path
    /// path: [0, 1, 3]
    ///
    pub fn child_mut_by_path(&mut self, path: &Vec<usize>) -> Option<&mut Node<T>> {
        let mut node: Option<&mut Node<T>> = Some(self);
        let level = path.len();
        for i in 1..level {
            if node.is_some() {
                node = node.unwrap().child_mut(*path.get(i).unwrap());
            }
        }
        return node;
    }

    ///
    /// get right child by deepth
    /// 
    pub fn last_child_by_level(&self, level: usize) -> Option<&Node<T>> {
        let mut node: Option<&Node<T>> = Some(self);
        for _ in 0..level {
            if node.is_some() {
                node = node.unwrap().last_child()
            } else {
                node = None;
                break;
            }
        }
        node
    }

    ///
    /// get right mut child by deepth
    /// 
    pub fn last_child_mut_by_level(&mut self, level: usize) -> Option<&mut Node<T>> {
        let mut node: Option<&mut Node<T>> = Some(self);
        for _ in 0..level {
            if node.is_some() {
                node = node.unwrap().last_child_mut()
            } else {
                node = None;
                break;
            }
        }
        node
    }

    ///  Deepth first traversal (Preorder) of a tree
    ///  
    pub fn deepth_first_search<F: FnMut(&T)>(&self, mut f: F) {
        self.dfs_helper(&mut f);
    }

    fn dfs_helper<F: FnMut(&T)>(&self, f: &mut F) {
        f(self.data());
        for child in self.children() {
            child.dfs_helper(f);
        }
    }
    /// Breadth first traversal (Level Order) of a tree
    ///  
    pub fn breadth_first_search<F: FnMut(&T)>(&self, mut f: F) {
        let mut queue: VecDeque<&Node<T>> = VecDeque::new();
        queue.push_back(self);
        while let Some(node) = queue.pop_front() {
            f(node.data());
            for child in node.children() {
                queue.push_back(child);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn data_works() {
        let s = format!("hello");
        let mut node = Node::new(s.clone());
        assert_eq!(node.data(), &s);
        let s2 = format!("world");
        let data = node.data_mut();
        *data = s2.clone();
        assert_eq!(node.data(), &s2);
    }

    #[test]
    fn child_works() {
        let s = format!("root");
        let mut root = Node::new(s);
        let level1_data = String::from("level1_1");
        let level1_2_data = String::from("level1_2");
        let level2_1_data = String::from("level2_1");
        let level1 = Node::new(level1_data.clone());
        let mut level1_2 = Node::new(level1_2_data.clone());
        let level2_1 = Node::new(level2_1_data.clone());

        level1_2.add(level2_1);
        root.add(level1);
        root.add(level1_2);
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.child(0).unwrap().data(), &level1_data);
        assert_eq!(root.child(1).unwrap().data(), &level1_2_data);
        assert_eq!(root.child(0).unwrap().children.len(), 0);
        assert_eq!(root.child(1).unwrap().children.len(), 1);
        assert_eq!(
            root.child(1).unwrap().child(0).unwrap().data(),
            &level2_1_data
        );

        let level2_2_data = String::from("level2_2");
        let level2_2 = Node::new(level2_2_data.clone());

        let level1_2_mut_opt = root.child_mut(1);
        assert!(level1_2_mut_opt.is_some());
        let level1_2_new_data = String::from("level1_2_new_data");
        let level1_2_mut = level1_2_mut_opt.unwrap();
        *(level1_2_mut.data_mut()) = level1_2_new_data.clone();
        assert_eq!(level1_2_mut.data(), &level1_2_new_data);
    }

    //    ---o(root)--
    //   /            \
    //  o(level1_1)    o(level1_2)
    //                 /
    //                o(level2_1)
    fn get_tree() -> Node<String> {
        let s = format!("root");
        let mut root = Node::new(s);
        let level1_data = String::from("level1_1");
        let level1_2_data = String::from("level1_2");
        let level2_1_data = String::from("level2_1");
        let level1 = Node::new(level1_data.clone());
        let mut level1_2 = Node::new(level1_2_data.clone());
        let level2_1 = Node::new(level2_1_data.clone());
        level1_2.add(level2_1);
        root.add(level1);
        root.add(level1_2);
        return root;
    }

    #[test]
    fn child_path_works() {
        let root = get_tree();
        let root1: Option<&Node<String>> = root.child_by_path(&vec![0]);
        assert!(root1.is_some());
        assert_eq!(root1.unwrap().data(), &String::from("root"));
        let level2_1 = root.child_by_path(&vec![0, 1, 0]);
        assert!(level2_1.is_some());
        assert_eq!(level2_1.unwrap().data(), &String::from("level2_1"));
    }

    #[test]
    fn last_child_level_works() {
        let root = get_tree();
        assert_eq!(root.last_child_by_level(0).unwrap().data(), &String::from("root"));
        assert_eq!(root.last_child_by_level(1).unwrap().data(), &String::from("level1_2"));
        assert_eq!(root.last_child_by_level(2).unwrap().data(), &String::from("level2_1"));
        assert!(root.last_child_by_level(3).is_none());
        assert!(root.last_child_by_level(4).is_none());
    }
    //
    //       ------------------o(root)---------------
    //      /                                        \
    //     o(level1_1)                               o(level1_2)
    //    /           \                              /
    //   o(level2_2)   o(level2_3)                  o(level2_1)
    //
    fn get_tree2() -> Node<String> {
        let s = format!("root");
        let mut root = Node::new(s);
        let level1_data = String::from("level1_1");
        let level1_2_data = String::from("level1_2");
        let level2_1_data = String::from("level2_1");
        let level2_2_data = String::from("level2_2");
        let level2_3_data = String::from("level2_3");
        let mut level1 = Node::new(level1_data.clone());
        let mut level1_2 = Node::new(level1_2_data.clone());
        let level2_1 = Node::new(level2_1_data.clone());
        let level2_2 = Node::new(level2_2_data.clone());
        let level2_3 = Node::new(level2_3_data.clone());
        level1_2.add(level2_1);
        level1.add(level2_2);
        level1.add(level2_3);
        root.add(level1);
        root.add(level1_2);
        return root;
    }

    #[test]
    fn traversal_works() {
        let root = get_tree2();
        let mut dfs_str = String::new();
        let mut bfs_str = String::new();
        root.deepth_first_search(|d| {
            dfs_str = format!("{}-{}", dfs_str, d);
            // println!("{}", d);
        });
        assert_eq!(
            dfs_str,
            String::from("-root-level1_1-level2_2-level2_3-level1_2-level2_1")
        );
        root.breadth_first_search(|d| {
            bfs_str = format!("{}-{}", bfs_str, d);
        });

        assert_eq!(
            bfs_str,
            String::from("-root-level1_1-level1_2-level2_2-level2_3-level2_1")
        );
    }
}
