use gdnative::prelude::*;
use gdnative::core_types::ToVariant;

pub mod scene;
pub mod utils;
pub mod native_lib;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct SaveDataManager;

#[methods]
impl SaveDataManager {
    fn new(_owner: &Node) -> Self {
        SaveDataManager
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        godot_print!("SaveDataManager singleton loaded");
    }

    #[export]
    fn save(&mut self, _owner: &Node, file_name: GodotString) {
        godot_print!("save! -> {}", file_name.to_string());
    }
}

pub fn goto_scene(owner: &Node, path: &str) {
    let global = owner.get_node("/root/Global").unwrap();
    let global = unsafe {
	    global.assume_safe()
    };
    
    unsafe {
        global.call("goto_scene", &[path.to_string().to_variant()]);
    }
}

pub fn quit_game(global: Ref<Node, Shared>) {
    let global = unsafe {
        global.assume_safe()
    };
    
    unsafe {
        global.call("quit_game", &[]);
    }
}


fn init(handle: InitHandle) {
    handle.add_class::<SaveDataManager>();
    handle.add_class::<crate::scene::title::TitleScene>();
    handle.add_class::<crate::scene::title::TitleEntries>();
    handle.add_class::<crate::utils::textbox::TextBox>();
    handle.add_class::<crate::utils::TextureDigit>();
    handle.add_class::<crate::scene::home::Calendar>();
    handle.add_class::<crate::scene::home::MagicBoardHome>();
    handle.add_class::<crate::scene::home::MagicBoard>();
    handle.add_class::<crate::scene::home::MBSaveEntrance>();
    handle.add_class::<crate::scene::home::MBSaveApp>();
    handle.add_class::<crate::scene::home::SaveEntry>();
    handle.add_class::<crate::scene::home::SaveDataSet>();
}

godot_init!(init);
