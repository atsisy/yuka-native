pub mod textbox;

use gdnative::{api::TextureRect, prelude::*};

use crate::get_node_auto;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct TextureDigit;

#[methods]
impl TextureDigit {
    fn new(_owner: &Node2D) -> Self {
        TextureDigit
    }

    #[export]
    fn _ready(&self, owner: &Node2D) {
        godot_print!("TextureDigit ready");
        self.hide_all_texture(owner);
        self.set_number(owner, 1);
    }

    #[export]
    fn set_number(&self, owner: &Node2D, number: i32) {
        self.hide_all_texture(owner);

        match number {
            1 => get_node_auto!(owner, "One", TextureRect).show(),
            2 => get_node_auto!(owner, "Two", TextureRect).show(),
            3 => get_node_auto!(owner, "Three", TextureRect).show(),
            4 => get_node_auto!(owner, "Four", TextureRect).show(),
            5 => get_node_auto!(owner, "Five", TextureRect).show(),
            6 => get_node_auto!(owner, "Six", TextureRect).show(),
            7 => get_node_auto!(owner, "Seven", TextureRect).show(),
            8 => get_node_auto!(owner, "Eight", TextureRect).show(),
            9 => get_node_auto!(owner, "Nine", TextureRect).show(),
            _ => get_node_auto!(owner, "Zero", TextureRect).show(),
        }
    }

    fn hide_all_texture(&self, owner: &Node2D) {
        get_node_auto!(owner, "One", TextureRect).hide();
        get_node_auto!(owner, "Two", TextureRect).hide();
        get_node_auto!(owner, "Three", TextureRect).hide();
        get_node_auto!(owner, "Four", TextureRect).hide();
        get_node_auto!(owner, "Five", TextureRect).hide();
        get_node_auto!(owner, "Six", TextureRect).hide();
        get_node_auto!(owner, "Seven", TextureRect).hide();
        get_node_auto!(owner, "Eight", TextureRect).hide();
        get_node_auto!(owner, "Nine", TextureRect).hide();
        get_node_auto!(owner, "Zero", TextureRect).hide();
    }
}
