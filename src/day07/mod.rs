const INPUT: &str = include_str!("input.txt");
pub(crate) fn main() {
    let tree = build_filesystem(INPUT);
}

fn build_filesystem(input: &str) -> Vec<Node> {
    let mut tree = vec![];

    let mut current = 0;

    for line in input.lines() {
        if line == "$ ls" {
            continue;
        } else if line.starts_with("$ cd") {
            let dirname = line.split_whitespace().nth(2).unwrap();
            if let Some(Node::Dir {
                children, parent, ..
            }) = tree.get(current)
            {
                if dirname == ".." {
                    current = parent.unwrap();
                    continue;
                }

                if let Some(child) = children.iter().find(|c| match tree.get(**c) {
                    Some(Node::Dir { name, .. }) => name == dirname,
                    _ => false,
                }) {
                    current = *child;
                } else {
                    tree.push(Node::Dir {
                        children: Vec::new(),
                        name: dirname.to_string(),
                        parent: Some(current),
                    });
                    current = tree.len() - 1;
                }
            } else {
                tree.push(Node::Dir {
                    children: Vec::new(),
                    name: dirname.to_string(),
                    parent: Some(current),
                });
                current = tree.len() - 1;
            }
        } else {
            let mut n = line.split_whitespace();
            let first = n.next().unwrap();
            let second = n.next().unwrap();
            let node = if first == "dir" {
                Node::Dir {
                    name: second.to_string(),
                    parent: Some(current),
                    children: Vec::new(),
                }
            } else {
                Node::File {
                    name: second.to_string(),
                    parent: Some(current),
                    size: first.parse().unwrap(),
                }
            };
            let child_idx = tree.len();
            if let Some(Node::Dir { children, .. }) = tree.get_mut(current) {
                children.push(child_idx);
            }
            tree.push(node);
        }
    }
    tree
}

enum Node {
    Dir {
        children: Vec<usize>,
        name: String,
        parent: Option<usize>,
    },
    File {
        name: String,
        parent: Option<usize>,
        size: u32,
    },
}
impl Node {
    fn get_size(&self, tree: &[Node]) -> u32 {
        match self {
            Node::Dir { children, .. } => children.iter().map(|&c| tree[c].get_size(tree)).sum(),
            Node::File { size, .. } => *size,
        }
    }

    fn get_name(&self) -> &String {
        match self {
            Node::Dir { name, .. } => name,
            Node::File { name, .. } => name,
        }
    }
}
