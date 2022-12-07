#![allow(unused)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

enum Descriptor {
    Root {
        children: Vec<Rc<RefCell<Descriptor>>>,
    },
    Dir {
        name: String,
        children: Vec<Rc<RefCell<Descriptor>>>,
        parent: Weak<RefCell<Descriptor>>,
    },
    File {
        name: String,
        size: usize,
        parent: Weak<RefCell<Descriptor>>,
    },
}

impl Descriptor {
    pub fn name(&self) -> String {
        match self {
            Descriptor::Root { .. } => "/".to_string(),
            Descriptor::Dir { name, .. } => name.clone(),
            Descriptor::File { name, .. } => name.clone(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Descriptor::Root { children } => children.iter().map(|c| c.borrow().size()).sum(),
            Descriptor::Dir { children, .. } => children.iter().map(|c| c.borrow().size()).sum(),
            Descriptor::File { size, .. } => *size,
        }
    }

    pub fn dirs(&self) -> Vec<Rc<RefCell<Descriptor>>> {
        match self {
            Descriptor::Root { children } => children
                .iter()
                .filter(|c| c.borrow().is_dir())
                .cloned()
                .collect(),
            Descriptor::Dir { children, .. } => children
                .iter()
                .filter(|c| c.borrow().is_dir())
                .cloned()
                .collect(),
            Descriptor::File { .. } => vec![],
        }
    }

    pub fn tree(&self) -> Vec<Rc<RefCell<Descriptor>>> {
        match self {
            Descriptor::Root { children } => {
                let mut result = vec![];
                for child in children {
                    result.push(child.clone());
                    result.extend(child.borrow().tree());
                }
                result
            }
            Descriptor::Dir { children, .. } => {
                let mut result = vec![];
                for child in children {
                    result.push(child.clone());
                    result.extend(child.borrow().tree());
                }
                result
            }
            Descriptor::File { .. } => vec![],
        }
    }

    pub fn files(&self) -> Vec<Rc<RefCell<Descriptor>>> {
        match self {
            Descriptor::Root { children } => children
                .iter()
                .filter(|c| c.borrow().is_file())
                .cloned()
                .collect(),
            Descriptor::Dir { children, .. } => children
                .iter()
                .filter(|c| c.borrow().is_file())
                .cloned()
                .collect(),
            Descriptor::File { .. } => vec![],
        }
    }

    pub fn child(&self, name: &str) -> Option<Rc<RefCell<Descriptor>>> {
        match self {
            Descriptor::Root { children } => {
                children.iter().find(|d| d.borrow().name() == name).cloned()
            }
            Descriptor::Dir { children, .. } => {
                children.iter().find(|d| d.borrow().name() == name).cloned()
            }
            Descriptor::File { .. } => None,
        }
    }

    pub fn parent(&self) -> Option<Rc<RefCell<Descriptor>>> {
        match self {
            Descriptor::Root { .. } => None,
            Descriptor::Dir { parent, .. } => parent.upgrade(),
            Descriptor::File { parent, .. } => parent.upgrade(),
        }
    }

    pub fn add_child(&mut self, child: Descriptor) -> Result<(), String> {
        match self {
            Descriptor::Root { children } => {
                children.push(Rc::new(RefCell::new(child)));
                Ok(())
            }
            Descriptor::Dir { children, .. } => {
                children.push(Rc::new(RefCell::new(child)));
                Ok(())
            }
            Descriptor::File { .. } => Err("Cannot add child to file".to_string()),
        }
    }

    pub fn is_root(&self) -> bool {
        matches!(self, Descriptor::Root { .. })
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, Descriptor::Dir { .. })
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Descriptor::File { .. })
    }
}

struct Filesystem {
    root: Rc<RefCell<Descriptor>>,
}

impl Filesystem {
    pub fn from_str(input: &str) -> Result<Self, String> {
        let root = Rc::new(RefCell::new(Descriptor::Root { children: vec![] }));
        let mut current_descriptor = root.clone();

        for line in input.lines() {
            current_descriptor =
                Self::update_fs_from_line(line.trim(), current_descriptor, root.clone())?;
        }

        Ok(Filesystem { root })
    }

    pub fn root(&self) -> Rc<RefCell<Descriptor>> {
        self.root.clone()
    }

    fn update_fs_from_line(
        line: &str,
        current: Rc<RefCell<Descriptor>>,
        root: Rc<RefCell<Descriptor>>,
    ) -> Result<Rc<RefCell<Descriptor>>, String> {
        let is_command = line.starts_with('$');

        if is_command {
            Self::update_fs_from_command(line.strip_prefix("$ ").unwrap_or(line), current, root)
        } else {
            Self::update_fs_from_ls(line, current.clone())?;
            Ok(current)
        }
    }

    fn update_fs_from_command(
        command: &str,
        current: Rc<RefCell<Descriptor>>,
        root: Rc<RefCell<Descriptor>>,
    ) -> Result<Rc<RefCell<Descriptor>>, String> {
        let mut command_words = command.split(' ');

        let command = command_words.next();
        match command {
            Some("cd") => {
                let path = command_words.next();
                match path {
                    Some("/") => Ok(root),
                    Some("..") => Ok(current.borrow().parent().unwrap_or(root)),
                    Some(path) => current
                        .borrow()
                        .child(path)
                        .ok_or(format!("No such directory: {}", path)),
                    None => Err("cd requires a path".to_string()),
                }
            }
            Some("ls") => Ok(current),
            _ => Err("unknown command")?,
        }
    }

    fn update_fs_from_ls(line: &str, mut current: Rc<RefCell<Descriptor>>) -> Result<(), String> {
        let mut line_words = line.split(' ');
        match line_words.next() {
            Some("dir") => {
                let name = line_words.next().ok_or("directory requires a name")?;
                if current.borrow().child(name).is_some() {
                    // skip, already exists
                    return Ok(());
                }

                let new_dir = Descriptor::Dir {
                    name: name.to_string(),
                    children: vec![],
                    parent: Rc::downgrade(&current),
                };
                current.borrow_mut().add_child(new_dir)?;
                Ok(())
            }
            Some(size) => {
                let size = size.parse::<usize>().map_err(|_| "invalid size")?;
                let name = line_words.next().ok_or("file requires a name")?;
                if current.borrow().child(name).is_some() {
                    // skip, already exists
                    return Ok(());
                }

                let new_file = Descriptor::File {
                    name: name.to_string(),
                    size,
                    parent: Rc::downgrade(&current),
                };
                current.borrow_mut().add_child(new_file)?;
                Ok(())
            }
            None => Err("ls line must start with dir or file size".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_the_root_dir() {
        let filesystem = Filesystem::from_str(input()).unwrap();

        assert_eq!(filesystem.root().borrow().name(), "/");
    }

    #[test]
    fn it_loads_the_dirs_in_the_root_path() {
        let filesystem = Filesystem::from_str(input()).unwrap();
        let root = filesystem.root();

        assert!(root.borrow().is_root());
        assert_eq!(root.borrow().dirs().len(), 2);
    }

    #[test]
    fn it_loads_the_files_in_the_root_path() {
        let filesystem = Filesystem::from_str(input()).unwrap();
        let root = filesystem.root();

        assert!(root.borrow().is_root());
        assert_eq!(root.borrow().files().len(), 2);
    }

    #[test]
    fn it_loads_the_file_with_the_size() {
        let filesystem = Filesystem::from_str(input()).unwrap();
        let root = filesystem.root();

        let file = root.borrow().files().first().unwrap().clone();
        assert_eq!(file.borrow().name(), "b.txt");
        assert_eq!(file.borrow().size(), 14848514);
    }

    #[test]
    fn it_returns_the_size_of_root() {
        let filesystem = Filesystem::from_str(input()).unwrap();
        let root = filesystem.root();

        assert_eq!(root.borrow().size(), 48381165);
    }

    #[test]
    fn it_returns_the_dirs_that_are_lower_or_equal_than_100000() {
        let filesystem = Filesystem::from_str(input()).unwrap();
        let root = filesystem.root();

        let dirs = root
            .borrow()
            .tree()
            .iter()
            .filter(|d| d.borrow().is_dir())
            .filter(|d| d.borrow().size() <= 100000)
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(dirs.len(), 2);
        assert_eq!(dirs.first().unwrap().borrow().name(), "a");
        assert_eq!(dirs.last().unwrap().borrow().name(), "e");
    }

    fn input() -> &'static str {
        "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
    }
}
