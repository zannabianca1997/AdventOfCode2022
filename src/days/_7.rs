use std::{collections::HashMap, error::Error};

#[derive(Debug)]
enum Command<'a> {
    Ls(Vec<&'a str>, Vec<(&'a str, usize)>),
    Cd(CdDest<'a>),
}
#[derive(Debug)]
enum CdDest<'a> {
    Root,
    Parent,
    Child(&'a str),
}

fn parse_input<'a>(input: &'a str) -> Result<Vec<Command<'a>>, Box<dyn Error>> {
    // removing the starting "$"
    let input = input
        .strip_prefix("$")
        .ok_or("Input must start with a command")?;
    // splitting commands
    let mut commands = Vec::new();
    for command in input.split("\n$") {
        // splitting the first line
        let (command, output) = command.split_once("\n").unwrap_or((command, ""));
        // splitting the first word of the command
        let (command, arg) = command.trim().split_once(" ").unwrap_or((command, ""));
        // matching the command
        commands.push(match command.trim() {
            "ls" => {
                if arg.trim() != "" {
                    return Err("Unexpected arg to ls".into());
                }
                let mut files = vec![];
                let mut dirs = vec![];
                for line in output.lines() {
                    if let Some((p1, name)) = line.split_once(" ") {
                        if p1.trim() == "dir" {
                            dirs.push(name.trim())
                        } else {
                            match p1.parse::<usize>() {
                                Ok(size) => files.push((name.trim(), size)),
                                Err(err) => return Err(err.into()),
                            }
                        }
                    } else {
                        return Err("No space in ls output line".into());
                    };
                }
                Command::Ls(dirs, files)
            }
            "cd" => {
                if output.trim() != "" {
                    return Err("Unexpected output to cd".into());
                }
                Command::Cd(match arg.trim() {
                    "/" => CdDest::Root,
                    ".." => CdDest::Parent,
                    dest => CdDest::Child(dest),
                })
            }
            cmd => return Err(format!("Unknow command {}", cmd).into()),
        })
    }
    Ok(commands)
}

#[derive(Debug)]
struct Directory {
    subdirs: HashMap<String, Directory>,
    files: HashMap<String, usize>,
    size: Option<usize>,
}
impl Directory {
    fn empty() -> Self {
        Self {
            subdirs: HashMap::new(),
            files: HashMap::new(),
            size: None,
        }
    }

    fn cached_size(&mut self) -> usize {
        if let Some(size) = self.size {
            size
        } else {
            let size = self
                .subdirs
                .iter_mut()
                .map(|(_, subdir)| subdir.cached_size())
                .sum::<usize>()
                + self.files.iter().map(|(_, &size)| size).sum::<usize>();
            // caching the result
            self.size = Some(size);
            size
        }
    }

    fn walk_subdirs<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut Directory) -> (),
    {
        f(self);
        for (_, subdir) in self.subdirs.iter_mut() {
            subdir.walk_subdirs(f)
        }
    }
}

fn build_directory_tree(mut history: Vec<Command<'_>>) -> Result<Directory, Box<dyn Error>> {
    // list of opened directories
    let mut current_path = vec![(String::new(), Directory::empty())];
    // adding a "cd /" as the last command, so the tree is completly closed at the end
    history.push(Command::Cd(CdDest::Root));
    // running each command
    for command in history {
        match command {
            Command::Ls(dirs, files) => {
                let (_, current_dir) = current_path.last_mut().unwrap();
                // adding all subdirs
                for subdir in dirs {
                    if !current_dir.subdirs.contains_key(subdir) {
                        current_dir
                            .subdirs
                            .insert(subdir.to_owned(), Directory::empty());
                    }
                }
                // adding all files
                for (file, size) in files {
                    if !current_dir.files.contains_key(file) {
                        current_dir.files.insert(file.to_owned(), size);
                    }
                }
            }
            Command::Cd(CdDest::Child(child_name)) => {
                let (_, current_dir) = current_path.last_mut().unwrap();
                // removing child directory
                let (child_name, child_dir) = current_dir
                    .subdirs
                    .remove_entry(child_name)
                    .ok_or(format!("{} sub directory not found", child_name))?;
                // putting back the directory on the stack
                current_path.push((child_name, child_dir))
            }
            Command::Cd(CdDest::Parent) => {
                let (old_dir_name, old_dir) = current_path.pop().unwrap();
                let (_, new_dir) = current_path
                    .last_mut()
                    .ok_or("Tried to \"cd ..\" from root")?;
                new_dir.subdirs.insert(old_dir_name, old_dir);
            }
            Command::Cd(CdDest::Root) => {
                while current_path.len() > 1 {
                    let (old_dir_name, old_dir) = current_path.pop().unwrap();
                    let (_, new_dir) = current_path.last_mut().unwrap();
                    new_dir.subdirs.insert(old_dir_name, old_dir);
                }
            }
        }
    }
    // returning root directory
    Ok(current_path.pop().unwrap().1)
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let history = parse_input(input)?;
    let mut root_dir = build_directory_tree(history)?;

    let mut total = 0;
    root_dir.walk_subdirs(&mut |dir| {
        let size = dir.cached_size();
        if size <= 100000 {
            total += size
        }
    });
    Ok(total.to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let history = parse_input(input)?;
    let mut root_dir = build_directory_tree(history)?;

    const MAX_ROOT_SIZE: usize = 70000000 - 30000000;
    if root_dir.cached_size() < MAX_ROOT_SIZE {
        return Err("Root is already small enough".into());
    }
    let delete_threshold = root_dir.cached_size() - MAX_ROOT_SIZE;

    let mut smallest = root_dir.cached_size();
    root_dir.walk_subdirs(&mut |dir| {
        let size = dir.cached_size();
        if size <= smallest && size >= delete_threshold {
            smallest = size
        }
    });

    Ok(smallest.to_string())
}
