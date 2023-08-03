extern crate ptree;
use ptree::{print_tree, TreeBuilder};

#[derive(Debug)]
struct Node {
    name: String,
    children: Vec<Node>,
}
impl TreeItem for Node {
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        if let Some(n) = self.0.file_name() {
            write!(f, "{}", style.paint(n.to_string_lossy()))
        } else {
            Ok(())
        }
    }

    fn children(&self) -> Cow<[Self::Child]> {
        let v = if let Ok(list) = fs::read_dir(&self.0) {
            list.filter_map(|item| item.ok())
                .map(|entry| entry.path())
                .map(PathItem)
                .collect()
        } else {
            Vec::new()
        };

        Cow::from(v)
    }
}

pub fn build_tree_from_paths(paths: Vec<String>) -> Node {
    let mut root = Node {
        name: String::from("/"), // Root node
        children: Vec::new(),
    };

    for path in paths {
        let mut current_node = &mut root;
        for component in path.split('/') {
            if component.is_empty() {
                continue; // Skip empty components (e.g., leading slash)
            }

            // Check if the component is already a child of the current node
            if let Some(child) = current_node
                .children
                .iter_mut()
                .find(|c| c.name == component)
            {
                current_node = child; // Move to the existing child
            } else {
                // Create a new child node if not found
                let new_child = Node {
                    name: String::from(component),
                    children: Vec::new(),
                };
                current_node.children.push(new_child);
                current_node = current_node.children.last_mut().unwrap();
            }
        }
    }

    root
}

fn build_tree(root: Node) -> TreeBuilder {}
