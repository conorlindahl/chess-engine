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