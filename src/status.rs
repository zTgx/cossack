#[warn(dead_code)]

pub enum Code {
    KOK = 0,
    KNotFound,
    KCorruption,
    KNotSupported,
    KInvalidArgument,
    KIOError,
}

pub struct Status {
    pub code: Code,
}

impl Status {
    pub fn new(code: Code) -> Self {
        Status {
            code
        }
    }
}

pub fn code(code: Code) -> isize {
    match code {
        Code::KOK => {
            return 0;
        },
        _ => {
            return 1;
        }
    }
}

pub fn strings(code: isize) -> String {
    if code == 0 {
        return "KOK".to_owned();
    }

    return "KIOError".to_owned()
}