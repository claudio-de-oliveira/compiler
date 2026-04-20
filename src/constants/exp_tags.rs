use crate::tags::{Tag, Type};

pub const E: Tag = Tag::new(Type::VARIABLE, 0);
pub const E_: Tag = Tag::new(Type::VARIABLE, 1);
pub const T: Tag = Tag::new(Type::VARIABLE, 2);
pub const T_: Tag = Tag::new(Type::VARIABLE, 3);
pub const F: Tag = Tag::new(Type::VARIABLE, 4);

// pub const ADD_OP: Tag = Tag::new(Type::TERMINAL, 0);
// pub const MUL_OP: Tag = Tag::new(Type::TERMINAL, 1);
// pub const LPAR: Tag = Tag::new(Type::TERMINAL, 2);
// pub const RPAR: Tag = Tag::new(Type::TERMINAL, 3);
// pub const NUMBER: Tag = Tag::new(Type::TERMINAL, 4);
// pub const END_MARK: Tag = Tag::new(Type::TERMINAL, 5);

pub const ADD_OP: u32 = 0;
pub const MUL_OP: u32 = 1;
pub const LPAR: u32 = 2;
pub const RPAR: u32 = 3;
pub const NUMBER: u32= 4;
pub const END_MARK: u32 = 5;
pub const ERROR: u32 = u32::MAX;
