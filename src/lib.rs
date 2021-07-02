use gdnative::core_types::ToVariant;
use gdnative::prelude::*;

pub mod native_lib;
pub mod scene;
pub mod utils;

pub fn goto_scene(owner: &Node, path: &str) {
    let global = owner.get_node("/root/Global").unwrap();
    let global = unsafe { global.assume_safe() };

    unsafe {
        global.call("goto_scene", &[path.to_string().to_variant()]);
    }
}

pub fn quit_game(global: Ref<Node, Shared>) {
    let global = unsafe { global.assume_safe() };

    unsafe {
        global.call("quit_game", &[]);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<native_lib::save_data::SaveDataManager>();
    handle.add_class::<crate::scene::title::TitleScene>();
    handle.add_class::<crate::scene::title::TitleEntries>();
    handle.add_class::<crate::utils::textbox::TextBox>();
    handle.add_class::<crate::utils::TextureDigit>();
    handle.add_class::<crate::scene::home::Calendar>();
    handle.add_class::<crate::scene::home::MagicBoardHome>();
    handle.add_class::<crate::scene::home::MagicBoard>();
    handle.add_class::<crate::scene::home::MBSaveEntrance>();
    handle.add_class::<crate::scene::home::MBSaveApp>();
    handle.add_class::<crate::scene::home::MBLoadApp>();
    handle.add_class::<crate::scene::home::SaveEntry>();
    handle.add_class::<crate::scene::home::SaveDataSet>();
    handle.add_class::<crate::scene::home::MBItemList>();
    handle.add_class::<crate::scene::home::MBItemEntry>();
}

godot_init!(init);
