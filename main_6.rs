use std::{cell::RefCell, rc::Rc};

fn print_structure(direntry: &DirEntry, leading: String) {
    match direntry {
        DirEntry::File(name, size) => {
            println!("{}- {} (file, size={}", leading, name, size);
        }
        DirEntry::Directory(name, entries) => {
            println!("{}- {} (dir)", leading, name);
            for e in entries {
                print_structure(&*e.borrow(), leading.clone() + "  ");
            }
        }
    }
}

fn print_sizes(direntry: &DirEntry, leading: String, small_dirs: &mut Vec<i64>) -> i64 {
    if let DirEntry::File(_, size) = direntry {
        return *size;
    }

    if let DirEntry::Directory(name, entries) = direntry {
        let mut entry_sizes = 0;
        for e in entries {
            let entry_size = print_sizes(&*e.borrow(), leading.clone() + "  ", small_dirs);
            entry_sizes += entry_size;
        }
        println!("{} {} - {}", leading, name, entry_sizes);
        if entry_sizes <= 100000 && name != "/" {
            small_dirs.push(entry_sizes);
        }
        return entry_sizes;
    }
    return 0;
}

fn get_sizes(direntry: &DirEntry, dir_sizes: &mut Vec<i64>) -> i64 {
    if let DirEntry::File(_, size) = direntry {
        return *size;
    }

    if let DirEntry::Directory(name, entries) = direntry {
        let mut entry_sizes = 0;
        for e in entries {
            let entry_size = get_sizes(&*e.borrow(), dir_sizes);
            entry_sizes += entry_size;
        }

        dir_sizes.push(entry_sizes);

        return entry_sizes;
    }
    return 0;
}

enum DirEntry {
    File(String, i64),
    Directory(String, Vec<Rc<RefCell<DirEntry>>>),
}

impl DirEntry {
    pub fn size(self: &Self) -> i64 {
        match self {
            DirEntry::File(_, size) => *size,
            DirEntry::Directory(_, entries) => entries
                .into_iter()
                .fold(0, |size, entry| size + entry.borrow().size()),
        }
    }
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let root: Rc<RefCell<DirEntry>> = Rc::new(RefCell::new(DirEntry::Directory(
        "/".to_owned(),
        Vec::new(),
    )));
    let mut pwd: Vec<String> = Vec::new();
    let mut cwd: Rc<RefCell<DirEntry>> = root.clone();

    for line in s.lines() {
        //println!("\n{}\npwd:  {:?}", line, pwd);
        //println!("pwd:  {:?}", pwd);

        if line == "$ cd .." {
            pwd.pop();
            cwd = root.clone();
            for segment in pwd.iter() {
                let local_cwd = cwd.clone();
                let de = local_cwd.borrow_mut();
                if let DirEntry::Directory(_, entries) = &*de {
                    cwd = entries
                        .iter()
                        .find(|entry| match *entry.clone().borrow_mut() {
                            DirEntry::File(_, _) => false,
                            DirEntry::Directory(ref name, _) => name == segment,
                        })
                        .unwrap()
                        .clone()
                }
            }
        } else if line == "$ cd /" {
            pwd.clear();
            cwd = root.clone();
        } else if line.starts_with("$ cd ") {
            pwd.push(line.split_ascii_whitespace().last().unwrap().to_owned());

            let local_cwd = cwd.clone();
            let de = local_cwd.borrow_mut();
            if let DirEntry::Directory(_, entries) = &*de {
                cwd = entries
                    .iter()
                    .find(|entry| match *entry.clone().borrow() {
                        DirEntry::File(_, _) => false,
                        DirEntry::Directory(ref name, _) => name == pwd.last().unwrap(),
                    })
                    .unwrap()
                    .clone();
            }
        } else if line.starts_with("$ ls") {
        } else if line.starts_with("dir ") {
            let name = line.split_ascii_whitespace().last().unwrap().to_owned();

            let local_cwd = cwd.clone();
            let mut de = local_cwd.borrow_mut();
            if let DirEntry::Directory(_, entries) = &mut *de {
                entries.push(Rc::new(RefCell::new(DirEntry::Directory(name, Vec::new()))));
            }
        } else {
            // Assume this is a file entry?
            let mut pieces = line.split_ascii_whitespace();
            let size = pieces.next().unwrap().parse::<i64>().unwrap();
            let name = pieces.next().unwrap();

            let local_cwd = cwd.clone();
            let mut de = local_cwd.borrow_mut();
            if let DirEntry::Directory(_, entries) = &mut *de {
                entries.push(Rc::new(RefCell::new(DirEntry::File(name.to_owned(), size))));
            }
        }
    }
    print_structure(&*root.borrow(), "".to_owned());
    println!("-------------------------------------");
    let mut small_dirs = Vec::new();
    let root_size = print_sizes(&*root.borrow(), "".to_owned(), &mut small_dirs);
    let sum = small_dirs.iter().fold(0, |sum, e| sum + e);
    println!("-------------------------------------");
    println!("sum of small directories: {}", sum);
    println!("-------------------------------------");
    let mut dir_sizes = Vec::new();
    let _ = get_sizes(&*root.borrow(), &mut dir_sizes);
    dir_sizes.sort();
    println!("root size is {}", root_size);
    for dir_size in dir_sizes {
        if 70000000 - (root_size - dir_size) > 30000000 {
            println!("Smallest directory to remove is {} bytes", dir_size);
            return Ok(());
        }
    }

    Ok(())
}
