use std::{collections::HashMap};

#[derive(Debug)]
enum Command {
    GoRoot,
    GoUp,
    GoDown(String),
    Files(Vec<(String, Option<usize>)>)
}

pub struct Commands {
    commands : Vec<Command>
}

pub struct FileSystem {
    files :HashMap<String, usize>,
    directories : HashMap<String, FileSystem>
}

fn parse_file(line : &str) -> Option<(String, Option<usize>)> {
    let (start, end) = line.split_once(" ")?;
    if start=="dir" {
        Some((end.to_string(), None))
    } else {
        if let Some(size) = start.parse::<usize>().ok() {
            Some((end.to_string(), Some(size)))
        } else {
            None
        }
    }
}

impl Command {
    fn from_string(string : &str) -> Option<Command> {
        if let Some(rest) = string.strip_prefix("cd ") {
            if rest == "/\n" {
                Some(Command::GoRoot)
            } else {
                if rest == "..\n" {
                    Some(Command::GoUp)
                } else {
                    Some(Command::GoDown(rest.trim_end().to_string()))
                }
            }
        } else {
            if let Some(rest) = string.strip_prefix("ls") {
                let files = rest.lines().filter_map(
                    |line| parse_file(line)
                ).collect();
                Some(Command::Files(files))
            } else {
                None
            }
        }
    }

    fn run(&self, path : &mut Vec<String>, fs : &mut FileSystem) {
        match self {
            Self::GoRoot => {
                path.clear();
            },
            Self::GoUp => {
                path.pop();
            },
            Self::GoDown(dir) => {
                path.push(dir.clone())
            }
            Self::Files(files) => {
                fs.add_files(path, files);
            }
        }
    }
}

impl Commands {
    pub fn from_string(string : &str) -> Commands {
        let cmds = string.split("$ ").filter_map(
            |str| Command::from_string(str)
        ).collect();
        Commands { commands: cmds }
    }

    pub fn filesystem(&self) -> FileSystem {
        let mut fs = FileSystem::new();
        let mut path : Vec<String> = Vec::new();
        for command in self.commands.iter() {
            command.run(&mut path, &mut fs);
        }
        fs
    }
}

impl FileSystem {

    fn new() -> FileSystem {
        FileSystem { 
            files : HashMap::new(),
            directories : HashMap::new()
        }
    }

    pub fn total(&self, max : usize) -> usize {
        self.total_internal(max).1
    }

    pub fn size(&self) -> usize {
        self.total_internal(0).0
    }

    pub fn smallest_dir(&self, min : usize) -> Option<usize> {
        self.smallest_dir_internal(min).1
    }

    fn smallest_dir_internal(&self, min : usize) -> (usize, Option<usize>) {
        let direct = self.direct_files_sizes();
        let (indirect_size,sub_dir) = self.indirect_smallest(min);
        let size = direct + indirect_size;
        if sub_dir.is_some() {
            (size, sub_dir)
        } else {
            if size > min {
                (size, Some(size))
            } else {
                (size, None)
            }
        }
    }

    fn indirect_smallest(&self, min : usize) -> (usize, Option<usize>) {
        let mut size : usize = 0;
        let mut smallest_op : Option<usize> = None;
        for dir in self.directories.values() {
            let (sub_size, sub_smallest_op) = dir.smallest_dir_internal(min);
            size += sub_size;
            if let Some(sub_smallest) = sub_smallest_op {
                if let Some(smallest) = smallest_op {
                    if sub_smallest < smallest {
                        smallest_op = sub_smallest_op;
                    }
                } else {
                    smallest_op = sub_smallest_op;
                }
            }
        }
        (size, smallest_op)
    }

    fn total_internal(&self, max : usize) -> (usize, usize) {
        let direct = self.direct_files_sizes();
        let (indirect_size,sub_total) = self.indirect_totals(max);
        let size = indirect_size + direct;
        let total = if size < max { sub_total + size } else { sub_total };
        (size, total)
    }

    fn direct_files_sizes(&self) -> usize {
        self.files.values().sum()
    }

    fn indirect_totals(&self, max : usize) -> (usize, usize) {
        let mut size : usize = 0;
        let mut total : usize = 0;
        for dir in self.directories.values() {
            let (sub_size, sub_total) = dir.total_internal(max);
            size += sub_size;
            total += sub_total;
        }
        (size, total)
    }

    fn add_files_root(&mut self, files : &Vec<(String, Option<usize>)>) {
        for (name, form) in files {
            if let Some(size) = form {
                self.files.insert(name.clone(), *size);
            } else {
                self.directories.insert(
                    name.clone(), 
                    FileSystem::new()
                );
            }
        }
    }

    fn add_files(&mut self, path : &[String], files : &Vec<(String, Option<usize>)>) {
        if path.len() == 0 {
            self.add_files_root(files);
        } else {
            self.directories.get_mut(&path[0]).unwrap().add_files(
                &path[1..], 
                files
            );
        }
    }

}