use crate::tags::{Tag, Type};

pub enum Action {
    Add(Tag, i32, i32),
    Sub(Tag, i32, i32),
    Mul(Tag, i32, i32),
    Div(Tag, i32, i32),
    Num(Tag),
    Echo(Tag, i32),
    Done(Tag, i32),
}

const ADD: u32 = 0;
const SUB: u32 = 1;
const MUL: u32 = 2;
const DIV: u32 = 3;
const NUM: u32 = 4;
const ECHO: u32 = 5;
const DONE: u32 = 6;

pub const ADD: Tag = Tag::new(Type::ACTION, ADD);
pub const SUB: Tag = Tag::new(Type::ACTION, SUB);
pub const MUL: Tag = Tag::new(Type::ACTION, MUL);
pub const DIV: Tag = Tag::new(Type::ACTION, DIV);
pub const NUM: Tag = Tag::new(Type::ACTION, NUM);
pub const ECHO: Tag = Tag::new(Type::ACTION, ECHO);
pub const DONE: Tag = Tag::new(Type::ACTION, DONE);

impl Action {
    pub fn from_tag(tag: Tag) -> Action {
        if !tag.is_action() {
            panic!("Tag must be of action type");
        }

        match tag.get_value() {
            ADD => Action::Add(tag, 0, 0),
            SUB => Action::Sub(tag, 0, 0),
            MUL => Action::Mul(tag, 0, 0),
            DIV => Action::Div(tag, 0, 0),
            NUM => Action::Num(tag),
            ECHO => Action::Echo(tag, 0),
            DONE => Action::Done(tag, 0),
            _ => panic!("Unknown action tag value: {}", tag.get_value()),
        }
    }

    pub fn get_value(&self) -> u32 {
        match self {
            Action::Add(tag, _, _) => tag.get_value(),
            Action::Sub(tag, _, _) => tag.get_value(),
            Action::Mul(tag, _, _) => tag.get_value(),
            Action::Div(tag, _, _) => tag.get_value(),
            Action::Num(tag) => tag.get_value(),
            Action::Echo(tag, _) => tag.get_value(),
            Action::Done(tag, _) => tag.get_value(),
        }
    }
}