use std::fmt;

use cpu::register;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Segment {
    Default,
    CS,
    DS,
    ES,
    FS,
    GS,
    SS,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Segment {
    pub fn as_str(&self) -> &str {
        match *self {
            Segment::Default | Segment::DS => "ds",
            Segment::CS => "cs",
            Segment::ES => "es",
            Segment::FS => "fs",
            Segment::GS => "gs",
            Segment::SS => "ss",
        }
    }

    pub fn as_register(&self) -> register::SR {
        match *self {
            Segment::Default | Segment::DS => register::SR::DS,
            Segment::CS => register::SR::CS,
            Segment::ES => register::SR::ES,
            Segment::FS => register::SR::FS,
            Segment::GS => register::SR::GS,
            Segment::SS => register::SR::SS,
        }
    }
}
