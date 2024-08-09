use std::rc::Rc;

use anyhow::Result;

fn main() -> Result<()> {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let node3 = Node::new(3);

    let rc_node3 = Rc::new(node3);
    node1.update_downstream(rc_node3.clone());
    node2.update_downstream(rc_node3);

    println!(
        "node1: {:?}, node2: {:?}, node3: {:?}",
        node1,
        node2,
        node1.get_downstream(),
    );

    Ok(())
}

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream)
    }

    fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}
