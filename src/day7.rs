use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alphanumeric1, digit1, multispace1, newline, u32},
    combinator::opt,
    multi::{separated_list0, separated_list1},
    IResult,
};

const MAX_SIZE: u32 = 100000;

const SAMPLE: &str = "$ cd /
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
7214296 k";

pub fn run_part_one() {
    let input = fs::read_to_string("src/day7.input").unwrap();
    // let input = SAMPLE;
    let (_, commands) = parse_commands(&input).unwrap();
    let x = DirEntry::Dir(DirectoryEntry {
        path: "/".into(),
        size: None,
        entries: vec![],
        parent: None,
    });
    let root = Rc::new(RefCell::new(x));
    let mut iterator = root.clone();
    commands
        .into_iter()
        .for_each(|command| iterator = command.execute(iterator.clone()));
    let mut sizes: Vec<u32> = vec![];
    root.borrow_mut().get_size(&mut sizes);
    let cumulated_sizes = sizes.iter().fold(0, |mut acc, size| {
        acc += size;
        acc
    });
    println!();
    println!("{}", cumulated_sizes);
}

enum DirEntry {
    Dir(DirectoryEntry),
    File(FileEntry),
}

#[derive(Default, Debug)]
struct DirectoryEntry {
    path: PathBuf,
    size: Option<u32>,
    entries: Vec<Rc<RefCell<DirEntry>>>,
    parent: Option<Rc<RefCell<DirEntry>>>,
}

#[derive(Debug)]
struct FileEntry {
    path: PathBuf,
    size: u32,
    parent: Option<Rc<RefCell<DirEntry>>>,
}

impl Default for FileEntry {
    fn default() -> Self {
        FileEntry {
            path: PathBuf::default(),
            size: u32::default(),
            parent: None,
        }
    }
}

trait GetSize {
    fn get_size(&mut self, sizes: &mut Vec<u32>) -> u32;
}

impl GetSize for FileEntry {
    fn get_size(&mut self, sizes: &mut Vec<u32>) -> u32 {
        self.size
    }
}

impl GetSize for DirectoryEntry {
    fn get_size(&mut self, sizes: &mut Vec<u32>) -> u32 {
        let x = match self.size {
            Some(ref size) => return *size,
            None => {
                let size = self
                    .entries
                    .iter_mut()
                    .map(|entry| entry.borrow_mut().get_size(sizes))
                    .fold(0, |mut acc, size| {
                        acc += size;
                        acc
                    });
                if size <= MAX_SIZE {
                    sizes.push(size);
                }
                self.size = Some(size);
                size
            }
        };
        x
    }
}

impl GetSize for DirEntry {
    fn get_size(&mut self, sizes: &mut Vec<u32>) -> u32 {
        match self {
            DirEntry::Dir(directory) => directory.get_size(sizes),
            DirEntry::File(file) => file.get_size(sizes),
        }
    }
}

#[derive(Debug)]
enum FileType<'a> {
    File { size: u32, name: &'a str },
    Dir { name: &'a str },
}

struct FromWrapper(Vec<Rc<RefCell<DirEntry>>>);

impl<'a> From<(Rc<RefCell<DirEntry>>, Vec<FileType<'a>>)> for FromWrapper {
    fn from(value: (Rc<RefCell<DirEntry>>, Vec<FileType<'a>>)) -> Self {
        let res = value
            .1
            .into_iter()
            .map(|file| match file {
                FileType::File { size, name } => Rc::new(RefCell::new(DirEntry::File(FileEntry {
                    path: name.into(),
                    size,
                    parent: Some(value.0.clone()),
                }))),
                FileType::Dir { name } => Rc::new(RefCell::new(DirEntry::Dir(DirectoryEntry {
                    path: name.into(),
                    size: None,
                    entries: vec![],
                    parent: Some(value.0.clone()),
                }))),
            })
            .collect::<Vec<Rc<RefCell<DirEntry>>>>();
        FromWrapper(res)
    }
}

#[derive(Debug)]
enum Command<'a> {
    Cd(CD<'a>),
    Ls(Vec<FileType<'a>>),
}

#[derive(Debug)]
enum CD<'a> {
    Root,
    Back,
    Forward(&'a str),
}

impl<'a> Command<'a> {
    fn execute(self, entry: Rc<RefCell<DirEntry>>) -> Rc<RefCell<DirEntry>> {
        match self {
            Command::Cd(command) => match command {
                CD::Root => entry,
                CD::Back => {
                    let x = match *entry.borrow() {
                        DirEntry::Dir(ref dir) => match &dir.parent {
                            Some(parent) => parent.clone(),
                            None => panic!("no parent!"),
                        },
                        _ => entry.clone(),
                    };
                    x
                }
                CD::Forward(dir_name) => match *entry.borrow() {
                    DirEntry::Dir(ref dir) => dir
                        .entries
                        .iter()
                        .find(|file| match *file.borrow() {
                            DirEntry::Dir(ref dir) => {
                                dir.path.file_name().unwrap().to_string_lossy().to_string()
                                    == dir_name
                            }
                            DirEntry::File(_) => false,
                        })
                        .expect("file not found")
                        .clone(),
                    _ => panic!("Cd forward on file"),
                },
            },
            Command::Ls(entries) => {
                let FromWrapper(mut res) = FromWrapper::from((entry.clone(), entries));
                let mut borrowed_entry = entry.borrow_mut();
                match *borrowed_entry {
                    DirEntry::Dir(ref mut dir) => {
                        dir.entries.append(&mut res);
                        drop(borrowed_entry);
                        return entry;
                    }
                    DirEntry::File(_) => todo!(),
                }
            }
        }
    }
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(newline, parse_command)(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    nom::branch::alt((parse_ls, parse_cd))(input)
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, vec) = separated_list0(newline, parse_file_type)(input)?;
    Ok((input, Command::Ls(vec)))
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    use Command::Cd;
    let (input, _) = tag("$ cd ")(input)?;
    let (input, name) = alt((tag(".."), alphanumeric1, tag("/")))(input)?;
    let cd = match name {
        "/" => Cd(CD::Root),
        ".." => Cd(CD::Back),
        other => Cd(CD::Forward(other)),
    };
    Ok((input, cd))
}

fn parse_file_type(input: &str) -> IResult<&str, FileType> {
    let (input, size) = opt(digit1)(input)?;
    let (input, file_type) = match size {
        Some(size) => {
            let (_, size) = u32(size)?;
            let (input, _) = multispace1(input)?;
            let (input, name) = is_a("qwertyuiopasdfghjklzxcvbnm.")(input)?;
            (input, FileType::File { size, name })
        }
        None => {
            let (input, _) = tag("dir ")(input)?;
            let (input, name) = alphanumeric1(input)?;
            (input, FileType::Dir { name })
        }
    };
    Ok((input, file_type))
}

impl std::fmt::Debug for DirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirEntry::Dir(ref dir) => {
                write!(
                    f,
                    "{{ \n  path: {:?}\n  size: {:?}\n  entries: {:?}\n}}",
                    dir.path, dir.size, dir.entries
                )
            }
            DirEntry::File(ref file) => write!(
                f,
                "{{ \n  path: {:?}\n  size: {:?}\n}}",
                file.path, file.size
            ),
        }
    }
}
