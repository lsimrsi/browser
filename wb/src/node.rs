use std::collections::HashMap;
use std::fmt;

pub type AttrMap = HashMap<String, String>;

pub struct Node {
    // data common to all nodes:
    children: Vec<Node>,

    // data specific to each node type:
    node_type: NodeType,
}

impl Node {
    pub fn text(data: String) -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
        Node {
            children: children,
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.node_type).expect("nah");
        for node in &self.children {
            write!(f, "{}", node).expect("nah again");
        }
        write!(f, "")
    }
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeType::Text(s) => writeln!(f, "  text: {}", s),
            NodeType::Element(ele_data) => {
                writeln!(f, "{}, attrs: {:?}", ele_data.tag_name, ele_data.attributes)
            }
        }
    }
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}
