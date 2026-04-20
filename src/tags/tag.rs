use std::fmt;

#[repr(u32)]
#[derive(Clone, Copy)] // Permite cópia para uso nas constantes
pub enum Type {
    TERMINAL = 0x8000,
    VARIABLE = 0x4000,
    ACTION = 0x2000
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Tag {
    value: u32,
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tag")
            .field("value", &format!("0x{:04x}", self.value))
            .finish()
    }
}

impl Tag {
    pub const fn new(tp: Type, val: u32) -> Self {
        Tag {
            value: (tp as u32) | val
        }
    }

    pub fn is_terminal(self) -> bool {
        self.value & (Type::TERMINAL as u32) != 0
    }

    pub fn is_variable(self) -> bool {
        self.value & (Type::VARIABLE as u32) != 0
    }

    pub fn is_action(self) -> bool {
        self.value & (Type::ACTION as u32) != 0
    }

    pub fn get_value(self) -> u32 {
        self.value & 0x0fff
    }

}

