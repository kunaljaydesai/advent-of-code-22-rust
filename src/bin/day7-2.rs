use std::{
    borrow::Borrow,
    cell::RefCell,
    io::BufRead,
    ops::{AddAssign, Index},
    rc::Rc,
};

struct INode {
    name: String,
    size: RefCell<i32>,
    children: RefCell<Vec<Rc<INode>>>,
    parent: Option<Rc<INode>>,
}

impl INode {
    pub fn dir_size(&self) -> i32 {
        let mut ds = self.size.borrow().clone();
        for child in self.children.borrow().iter() {
            ds += child.dir_size();
        }
        return ds;
    }

    pub fn goto(node: Option<Rc<INode>>, relative_path: &str) -> Rc<INode> {
        if node.is_none() {
            let root = Rc::new(INode {
                name: relative_path.to_string(),
                size: RefCell::new(0),
                children: RefCell::new(vec![]),
                parent: None,
            });
            return root;
        } else {
            if relative_path == ".." {
                let parent = Rc::clone(node.unwrap().parent.as_ref().unwrap());
                return parent;
            } else {
                for child in node.as_ref().unwrap().children.borrow().iter() {
                    if child.name == relative_path {
                        return Rc::clone(child);
                    }
                }
                let child_node = Rc::new(INode {
                    name: relative_path.to_string(),
                    size: RefCell::new(0),
                    children: RefCell::new(vec![]),
                    parent: Some(Rc::clone(node.as_ref().unwrap())),
                });
                node.unwrap()
                    .children
                    .borrow_mut()
                    .push(Rc::clone(&child_node));
                return child_node;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day7.txt");
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut current_directory = None;
    for result_line in reader.lines() {
        let line = result_line.unwrap();
        let is_new_command = line.chars().nth(0).unwrap() == '$';
        let args: Vec<&str> = line.split(" ").collect();
        if is_new_command {
            let command = *args.index(1);
            if command.eq("cd") {
                let dir = *args.index(2);
                current_directory = Some(INode::goto(current_directory, dir));
            }
        } else {
            // ls output
            let is_dir = *args.index(0) == "dir";
            if !is_dir {
                let file_size = args.index(0).parse::<i32>();
                current_directory
                    .as_ref()
                    .unwrap()
                    .size
                    .borrow_mut()
                    .add_assign(file_size?);
            }
        }
    }

    // set current_directory to root
    while current_directory
        .borrow()
        .as_ref()
        .unwrap()
        .parent
        .is_some()
    {
        current_directory = Some(Rc::clone(
            current_directory.as_ref().unwrap().parent.as_ref().unwrap(),
        ));
    }

    let mut queue = vec![current_directory];
    let mut total_size = 0;
    while queue.len() > 0 {
        let el = Rc::clone(queue.pop().unwrap().as_ref().unwrap());
        if el.dir_size() <= 100000 {
            total_size += el.dir_size();
        }
        for child in el.children.borrow().iter() {
            queue.push(Some(Rc::clone(child)));
        }
    }
    println!("Total size {}", total_size);
    Ok(())
}
