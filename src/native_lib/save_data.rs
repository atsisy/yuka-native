use gdnative::prelude::*;

use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SoilItem {
    Fuyodo,
    Kurotsuchi,
    Baiyodo,
}

impl SoilItem {
    pub fn get_display_name(&self) -> &str {
        match self {
            Self::Fuyodo => "腐葉土",
            Self::Kurotsuchi => "黒土",
            Self::Baiyodo => "培養土",
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FertilizerItem {
    Aburakasu,
    Gyohi,
    ShimoGoe,
    Chemical,
}

impl FertilizerItem {
    pub fn get_display_name(&self) -> &str {
        match self {
            Self::Aburakasu => "油粕",
            Self::Gyohi => "魚肥",
            Self::ShimoGoe => "下肥",
            Self::Chemical => "化学肥料",
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Item {
    Soil(SoilItem),
    Fertilizer(FertilizerItem),
}

impl Item {
    pub fn get_display_name(&self) -> &str {
        match self {
            Self::Soil(soil_item) => soil_item.get_display_name(),
            Self::Fertilizer(fertilizer) => fertilizer.get_display_name(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemManager {
    items: HashMap<Item, usize>,
}

impl ItemManager {
    pub fn new() -> Self {
        ItemManager {
            items: HashMap::new(),
        }
    }

    pub fn add_items(&mut self, item: Item, count: usize) {
        if let Some(bag_count) = self.items.get_mut(&item) {
            *bag_count += count;
        } else {
            self.items.insert(item, count);
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<Item, usize> {
        self.items.iter()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NativeSaveData {
    items: ItemManager,
}

impl NativeSaveData {
    pub fn new() -> Self {
        NativeSaveData {
            items: ItemManager::new(),
        }
    }

    pub fn get_items(&self) -> &ItemManager {
        &self.items
    }

    pub fn add_items(&mut self, item: Item, count: usize) {
        self.items.add_items(item, count);
    }
}

thread_local!(static CURRENT_SAVEDATA: RefCell<Option<NativeSaveData>> = {
    RefCell::new(None)
});

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

    #[export]
    fn load(&mut self, _owner: &Node, file_name: GodotString) {
        godot_print!("load! -> {}", file_name.to_string());
    }

    #[export]
    fn create_new_entry_and_set_as_current(&mut self, _owner: &Node) {
        godot_print!("create new save entry");
        CURRENT_SAVEDATA.with(|current_save_data| {
            current_save_data.replace_with(|_| Some(NativeSaveData::new()))
        });
    }
}

pub fn control_save_data<F, R>(f: F) -> R
where F: FnOnce(&NativeSaveData) -> R
{
    CURRENT_SAVEDATA.with(|current_save_data| {
        f(current_save_data.borrow().as_ref().unwrap())
    })
}

pub fn control_save_data_mut<F, R>(f: F) -> R
where F: FnOnce(&mut NativeSaveData) -> R
{
    CURRENT_SAVEDATA.with(|current_save_data| {
        f(current_save_data.borrow_mut().as_mut().unwrap())
    })
}