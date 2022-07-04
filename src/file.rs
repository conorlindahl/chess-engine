#[derive(Debug, Clone, Copy)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn next(&self) -> Option<File> {
        match self {
            File::A => Some(File::B),
            File::B => Some(File::C),
            File::C => Some(File::D),
            File::D => Some(File::E),
            File::E => Some(File::F),
            File::F => Some(File::G),
            File::G => Some(File::H),
            File::H => None,
        }
    }

    pub fn previous(&self) -> Option<File> {
        match self {
            File::A => None,
            File::B => Some(File::A),
            File::C => Some(File::B),
            File::D => Some(File::C),
            File::E => Some(File::D),
            File::F => Some(File::E),
            File::G => Some(File::F),
            File::H => Some(File::G),
        }
    }
}