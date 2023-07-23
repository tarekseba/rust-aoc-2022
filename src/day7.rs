#[derive(Debug)]
enum DirEntry {
    Dir(Box<DirectoryEntry>),
    File(Box<FileEntry>),
}

#[derive(Default, Debug)]
struct DirectoryEntry {
    name: String,
    path: String,
    size: Option<u32>,
    entries: Vec<DirEntry>,
    parent: Option<DirEntry>,
}

#[derive(Debug)]
struct FileEntry {
    name: String,
    path: String,
    size: u32,
    parent: Option<DirEntry>,
}

impl Default for FileEntry {
    fn default() -> Self {
        FileEntry {
            name: String::default(),
            path: String::default(),
            size: u32::default(),
            parent: Some(DirEntry::Dir(Box::new(DirectoryEntry::default()))),
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
