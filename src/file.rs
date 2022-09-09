#[derive(Debug, Clone, Copy)]
pub struct File {
    val: u8,
}

pub const MAX_NUMBER_OF_FILES: u8 = 8;

impl File {
    pub fn build(file: u8) -> Result<File, &'static str> {
        if file >= MAX_NUMBER_OF_FILES {
            return Err("File outside allowable bounds");
        }
        Ok(File{val: file})
    }

    fn new(file: u8) -> File {
        File{val: file}
    }

    pub fn iter_files(range: impl Iterator<Item = u8>) -> impl Iterator<Item = File> {
        range.filter(|file| file < &MAX_NUMBER_OF_FILES).map(|file| File::new(file))
    }

    pub fn value(&self) -> u8 {
        self.val
    }

    pub fn next(&self) -> Option<File> {
        if self.val + 1 >= MAX_NUMBER_OF_FILES {
            None
        } else {
            Some(File{val: self.val + 1})
        }
    }

    pub fn previous(&self) -> Option<File> {
        if self.val == 0 {
            None
        } else {
            Some(File{val: self.val - 1})
        }
    }
}

use std::cmp::PartialEq;
use std::cmp::Eq;
impl PartialEq for File {
    fn eq(&self, other: &File) -> bool {
        self.val == other.val
    }
}
impl Eq for File {}

use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::cmp::Ordering;
impl Ord for File {
    fn cmp(&self, other: &File) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &File) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}