pub const TERMINAL: u16 = 0x8000u16;
pub const VARIABLE: u16  = 0x4000u16;
pub const ACTION: u16 = 0x2000u16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tag {}

impl Tag {
    #[inline]
    pub fn is_terminal(&self) -> bool {
        ((*self as isize) & (TERMINAL as isize)) != 0
    }

    #[inline]
    pub fn is_variable(&self) -> bool {
        ((*self as isize) & (VARIABLE as isize)) != 0
    }

    #[inline]
    pub fn is_action(&self) -> bool {
        ((*self as isize) & (ACTION as isize)) != 0
    }

    #[inline]
    pub fn get_value(&self) -> isize {
        (*self as isize) & ((TERMINAL + VARIABLE + ACTION) as isize)
    }
}
