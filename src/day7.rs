use std::{
    cell::RefCell,
    path::PathBuf,
    rc::{Rc, Weak},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, multispace1, newline, u32},
    combinator::opt,
    IResult, multi::{separated_list1, separated_list0},
};

pub fn run_part_one() {
    let mut root: DirEntry = DirEntry::Dir(DirectoryEntry {
        path: "/".into(),
        size: None,
        entries: vec![
            DirEntry::Dir(DirectoryEntry {
                path: PathBuf::from("idm"),
                size: None,
                entries: vec![
                    DirEntry::File(FileEntry {
                        path: "idm".into(),
                        size: 5,
                        parent: None,
                    }),
                    DirEntry::File(FileEntry {
                        path: "idm".into(),
                        size: 5,
                        parent: None,
                    }),
                ],
                parent: None,
            }),
            DirEntry::File(FileEntry {
                size: 2,
                ..FileEntry::default()
            }),
            DirEntry::Dir(DirectoryEntry::default()),
        ],
        parent: None,
    });
    dbg!("{:?}", root.get_size());
    dbg!(&root);

    println!("{:?}", parse_command("$ cd tarek"));
    println!("{:?}", parse_command("$ ls\n34241 tom\ndir uncle\n"));
    println!("{:?}", parse_file_type("13455 LOL"));
    println!("{:?}", parse_file_type("dir LOL"));
}

#[derive(Debug)]
enum DirEntry {
    Dir(DirectoryEntry),
    File(FileEntry),
}

#[derive(Default, Debug)]
struct DirectoryEntry {
    path: PathBuf,
    size: Option<u32>,
    entries: Vec<DirEntry>,
    parent: Option<Rc<RefCell<DirEntry>>>,
}

#[derive(Debug)]
struct FileEntry {
    path: PathBuf,
    size: u32,
    parent: Option<Weak<RefCell<DirEntry>>>,
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
    fn get_size(&mut self) -> u32;
}

impl GetSize for FileEntry {
    fn get_size(&mut self) -> u32 {
        self.size
    }
}

impl GetSize for DirectoryEntry {
    fn get_size(&mut self) -> u32 {
        let x = match self.size {
            Some(ref size) => return *size,
            None => {
                let size = self.entries.iter_mut().map(|entry| entry.get_size()).fold(
                    0,
                    |mut acc, size| {
                        acc += size;
                        acc
                    },
                );
                self.size = Some(size);
                size
            }
        };
        x
    }
}

impl GetSize for DirEntry {
    fn get_size(&mut self) -> u32 {
        match self {
            DirEntry::Dir(directory) => directory.get_size(),
            DirEntry::File(file) => file.get_size(),
        }
    }
}

#[derive(Debug)]
enum FileType<'a> {
    File { size: u32, name: &'a str },
    Dir { name: &'a str },
}

#[derive(Debug)]
enum Command<'a> {
    Cd { to: &'a str },
    Ls(Vec<FileType<'a>>),
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    nom::branch::alt((parse_ls, parse_cd))(input)
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, vec) = separated_list0(newline, parse_file_type)(input)?;
    println!("UL{input}");
    Ok((input, Command::Ls(vec)))
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, name) = alt((tag(".."), alphanumeric1))(input)?;
    Ok((input, Command::Cd { to: name }))
}

fn parse_file_type(input: &str) -> IResult<&str, FileType> {
    let (input, size) = opt(digit1)(input)?;
    let (input, file_type) = match size {
        Some(size) => {
            let (_, size) = u32(size)?;
            let (input, _) = multispace1(input)?;
            let (input, name) = alphanumeric1(input)?;
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
