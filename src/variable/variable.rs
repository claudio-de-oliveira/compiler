// use crate::tags::{Tag, Type};

// pub enum Variable {
//     Start(Tag),
//     Exp(Tag),
//     ExpLine(Tag, i32),
//     Term(Tag),
//     TermLine(Tag, i32),
//     Factor(Tag),
// }

// const START: u32 = 0;
// const EXP: u32 = 1;
// const EXPLINE: u32 = 2;
// const TERM: u32 = 3;
// const TERMLINE: u32 = 4;
// const FACTOR: u32 = 5;

// pub const S: Tag = Tag::new(Type::VARIABLE, START);
// pub const E: Tag = Tag::new(Type::VARIABLE, EXP);
// pub const E_: Tag = Tag::new(Type::VARIABLE, EXPLINE);
// pub const T: Tag = Tag::new(Type::VARIABLE, TERM);
// pub const T_: Tag = Tag::new(Type::VARIABLE, TERMLINE);
// pub const F: Tag = Tag::new(Type::VARIABLE, FACTOR);

// impl Variable {
//     pub fn from_tag(tag: Tag) -> Variable {
//         if !tag.is_variable() {
//             panic!("Tag must be of variable type");
//         }

//         match tag.get_value() {
//             START => Variable::Start(tag),
//             EXP => Variable::Exp(tag),
//             EXPLINE => Variable::ExpLine(tag, 0), // Inicializa com 0
//             TERM => Variable::Term(tag),
//             TERMLINE => Variable::TermLine(tag, 0), // Inicializa com
//             FACTOR => Variable::Factor(tag),
//             _ => panic!("Unknown variable tag value: {}", tag.get_value()),
//         }
//     }

//     pub fn get_value(&self) -> u32 {
//         match self {
//             Variable::Start(tag) => tag.get_value(),
//             Variable::Exp(tag) => tag.get_value(),
//             Variable::ExpLine(tag, _) => tag.get_value(),
//             Variable::Term(tag) => tag.get_value(),
//             Variable::TermLine(tag, _) => tag.get_value(),
//             Variable::Factor(tag) => tag.get_value(),
//         }
//     }
// }