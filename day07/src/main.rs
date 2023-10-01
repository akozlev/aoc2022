// lots of help from fasterthanlime's articles
use std::{fs, cell::RefCell, rc::Rc, collections::HashMap, fmt};

#[derive(Debug)]
enum Command {
    Ls(Vec<LsItem>),
    Cd(String)
}

#[derive(Debug)]
enum LsItem {
    Directory(String),
    File(String, usize)
}

type NodeHandle = Rc<RefCell<Node>>;
#[derive(Default)]
struct Node {
    size: usize,
    parent: Option<NodeHandle>,
    children: HashMap<String, NodeHandle>
}
struct PrettyNode<'a>(&'a NodeHandle);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();
        if this.size == 0 {
            writeln!(f, "(dir)")?;
        }
        else {
            writeln!(f, "(file, size={})", this.size)?;
        }

        for (name, child) in &this.children {
            for (index, line) in format!("{:?}", PrettyNode(child)).lines().enumerate() {
                if index == 0 {
                    writeln!(f, "{name} {line}")?;
                }
                else {
                    writeln!(f, "  {line}")?;
                }
            }
        }

        Ok(())
    }
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn total_size(&self) -> u64 {
        self.children
            .values()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
            + self.size as u64
    }
}

fn main() {
    let input = fs::read_to_string("./data/in").expect("Something went wrong");
    let commands = input.split("\n$ ").skip(1).map(|cmd| {
        match &cmd[0..2] {
            "ls" => Command::Ls(cmd[2..].split('\n').filter(|x| !x.is_empty()).map(|y| {
                match y.split_once(' ') {
                    Some(("dir", b)) => LsItem::Directory(b.into()),
                    Some((a,b)) => LsItem::File(b.into(), a.parse().unwrap()),
                    None => panic!()
                }
            }
                ).collect()),
            "cd" => Command::Cd(cmd[3..].into()),
            _ => panic!()
        }
    });

    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();

    commands.for_each(|c| match c {
        Command::Cd(arg) => match arg.as_str() {
            "/" => {
                // Nothing happens
            },
            ".." => {
                let parent = node.borrow().parent.clone().unwrap();
                node = parent;
            }
            _ => {
                let child = node.borrow_mut().children.entry(arg).or_default().clone();
                node = child;
            }
        },
        Command::Ls(args) => {
            for arg in args {
                match arg {
                    LsItem::Directory(name) => {
                        let entry = node.borrow_mut().children.entry(name).or_default().clone();
                        entry.borrow_mut().parent = Some(node.clone());
                    }
                    LsItem::File(name, size) => {
                        let entry = node.borrow_mut().children.entry(name).or_default().clone();
                        entry.borrow_mut().size = size;
                        entry.borrow_mut().parent = Some(node.clone());
                    }
                }
            }

        }
    });

    let free_space = 70_000_000 - root.borrow().total_size();
    let mut all_dir = all_dirs(root.clone())
        .map(|d| d.borrow().total_size())
        .filter(|&x| x+free_space>=30_000_000)
        .collect::<Vec<_>>();
    all_dir.sort();

    println!("{:#?}", all_dir);
    // println!("{:#?}", to_del);
}

fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(
        std::iter::once(n).chain(
            children
            .into_iter()
            .filter_map(|x|{
                if x.borrow().is_dir() {
                    Some(all_dirs(x))
                } else {
                    None
                }
            })
            .flatten(),
            ),
        )
}
