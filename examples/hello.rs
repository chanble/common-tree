use common_tree::Node;

struct NodeData {
    id: usize,
    name: String,
}

impl NodeData {
    pub fn new(id: usize, name: String) -> Self {
        Self { id, name }
    }

    pub fn print(&self) {
        println!("id: {}, name: {}", self.id, self.name);
    }
}
type HellNode = Node<NodeData>;

fn main() {
    let mut root = HellNode::new(NodeData::new(0, format!("root")));

    root.data().print();

    let mut level1_0 = HellNode::new(NodeData::new(1, format!("level1_0")));
    let mut level1_1 = HellNode::new(NodeData::new(2, format!("level1_1")));

    let level2_0_0 = HellNode::new(NodeData::new(3, format!("level2_0_0")));
    let level2_0_1 = HellNode::new(NodeData::new(4, format!("level2_0_1")));

    level1_0.add(level2_0_0);
    level1_0.add(level2_0_1);

    let level2_1_0 = HellNode::new(NodeData::new(3, format!("level2_1_0")));

    level1_1.add(level2_1_0);

    root.add(level1_0);
    root.add(level1_1);

    root.deepth_first_search(|nd| nd.print());

    root.breadth_first_search(|nd| nd.print());
}
