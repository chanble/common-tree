common tree 树结构
================================================

通用树结构库， 是现实了深度遍历(preorder, 先序遍历)和广度优先遍历

## usage 用法

更多可以查看examples

```rust

use common_tree::Node;

struct NodeData {
    id: usize,
    name: String,
}

impl NodeData {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
        }
    }

    pub fn print(&self) {
        println!("id: {}, name: {}", self.id, self.name);
    }
}
type HellNode = Node<NodeData>;

fn main() {

    let root = HellNode::new(NodeData::new(0, format!("root")));

    root.data().print();

}

```

## run examples

```
cargo run --example hello
```