use std::cell::RefCell;
use std::rc::Rc;

use super::input_handler::InputHandler;
use crate::Context;

pub struct KeyboardContext {
    pub input_handler: Rc<RefCell<InputHandler>>,
}

impl KeyboardContext {
    pub(crate) fn new(input_handler: Rc<RefCell<InputHandler>>) -> Self {
        KeyboardContext { input_handler }
    }

    pub(crate) fn is_key_pressed(&self, key: &str) -> bool {
        self.input_handler.borrow().is_key_pressed(key)
    }

    pub(crate) fn is_key_down(&self, key: &str) -> bool {
        self.input_handler.borrow().is_key_down(key)
    }
}

/// Checks if a key is currently pressed down.
pub fn is_key_pressed(ctx: &Context, key: &str) -> bool {
    ctx.keyboard_context.is_key_pressed(key)
}

/// Checks if a key was pressed down on exectly this frame.
pub fn is_key_down(ctx: &Context, key: &str) -> bool {
    ctx.keyboard_context.is_key_down(key)
}
