///
/// 
/// 
/// 
pub struct Node<T> {
    data: T,
    children: Vec<Node<T>>,
}

impl<T> Node<T> {
    ///
    /// create a Node
    /// 
    pub fn new(data: T) -> Self {
        Node {
            data,
            children: Vec::new(),
        }
    }

    ///
    /// add a node to this
    /// 
    pub fn add(&mut self, node: Node<T>) {
        self.children.push(node);
    }

    // get mut data
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    // get data
    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn child(&self, index: usize) -> Option<&Node<T>> {
        self.children.get(index)
    }

    pub fn last_child(&self) -> Option<&Node<T>> {
        self.children.last()
    }

    pub fn child_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        self.children.get_mut(index)
    }

    pub fn last_child_mut(&mut self) -> Option<&mut Node<T>> {
        self.children.last_mut()
    }

    pub fn children(&self) -> &Vec<Node<T>> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Node<T>> {
        &mut self.children
    }

    //
    // 得到深度是level的第index个只读节点
    // path: [0, 1, 3]
    //
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

    //
    // 得到深度是level的第index个可写节点
    //
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
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn data_works() {
        let s = format!("hello");
        let mut node = Node::new(s.clone());
        assert_eq!(node.data(), &s);
        let s2 = format!("world");
        let mut data = node.data_mut();
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
        assert_eq!(root.child(1).unwrap().child(0).unwrap().data(), &level2_1_data);

        let level2_2_data = String::from("level2_2");
        let level2_2 = Node::new(level2_2_data.clone());

        let level1_2_mut_opt = root.child_mut(1);
        assert!(level1_2_mut_opt.is_some());
        let level1_2_new_data = String::from("level1_2_new_data");
        let level1_2_mut = level1_2_mut_opt.unwrap();
        *(level1_2_mut.data_mut()) = level1_2_new_data.clone();
        assert_eq!(level1_2_mut.data(), &level1_2_new_data);
    }

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
        let mut root = get_tree();
        let root1: Option<&Node<String>> = root.child_by_path(&vec![0]);
        assert!(root1.is_some());
        assert_eq!(root1.unwrap().data(), &String::from("root"));
        let level2_1 = root.child_by_path(&vec![0, 1, 0]);
        assert!(level2_1.is_some());
        assert_eq!(level2_1.unwrap().data(), &String::from("level2_1"));
    }
}