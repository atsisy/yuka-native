use gdnative::prelude::*;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    io::{Read, Write},
    str::FromStr,
};

use super::GensoDate;

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

impl Display for SoilItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Fuyodo => write!(f, "腐葉土"),
            Self::Kurotsuchi => write!(f, "黒土"),
            Self::Baiyodo => write!(f, "培養土"),
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

impl Display for FertilizerItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Aburakasu => write!(f, "油粕"),
            Self::Gyohi => write!(f, "魚肥"),
            Self::ShimoGoe => write!(f, "下肥"),
            Self::Chemical => write!(f, "化学肥料"),
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

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Soil(soil) => soil.fmt(f),
            Self::Fertilizer(fert) => fert.fmt(f),
        }
    }
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "腐葉土" => Ok(Item::Soil(SoilItem::Fuyodo)),
            "黒土" => Ok(Item::Soil(SoilItem::Kurotsuchi)),
            "培養土" => Ok(Item::Soil(SoilItem::Baiyodo)),
            "油粕" => Ok(Item::Fertilizer(FertilizerItem::Aburakasu)),
            "魚肥" => Ok(Item::Fertilizer(FertilizerItem::Gyohi)),
            "下肥" => Ok(Item::Fertilizer(FertilizerItem::ShimoGoe)),
            "化学肥料" => Ok(Item::Fertilizer(FertilizerItem::Chemical)),
            _ => Err("BUG".to_string()),
        }
    }
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct ItemManager {
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
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

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NativeSaveData {
    items: ItemManager,
    date: GensoDate,
}

impl NativeSaveData {
    pub fn new() -> Self {
        NativeSaveData {
            items: ItemManager::new(),
            date: GensoDate::new(112, 5, 1),
        }
    }

    pub fn get_items(&self) -> &ItemManager {
        &self.items
    }

    pub fn add_items(&mut self, item: Item, count: usize) {
        self.items.add_items(item, count);
    }

    pub fn get_date(&self) -> &GensoDate {
        &self.date
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
    fn save(&mut self, _owner: &Node, file_name: Variant) {
        control_save_data(|save_data| {
            let mut file = std::fs::File::create(file_name.to_string()).unwrap();

            file.write_all(
                crate::native_lib::crypt::crypt_str(&toml::to_string(save_data).unwrap())
                    .unwrap()
                    .as_slice(),
            )
            .unwrap();
            file.flush().unwrap();
        });
    }

    #[export]
    fn load(&mut self, _owner: &Node, file_name: GodotString) {
        godot_print!("load! -> {}", file_name.to_string());

        let loaded_save_data: NativeSaveData =
            match std::fs::File::open(file_name.to_string().as_str()) {
                Ok(mut file) => {
                    let mut buf = Vec::new();
                    match file.read_to_end(&mut buf) {
                        Ok(_) => (),
                        Err(_) => return,
                    }

                    let content = crate::native_lib::crypt::decrypt_str(&buf);

                    let loaded_save_data = toml::from_str(&content.unwrap());

                    match loaded_save_data {
                        Ok(loaded_save_data) => loaded_save_data,
                        Err(_) => return,
                    }
                }
                Err(_) => return,
            };

        CURRENT_SAVEDATA.with(|current_save_data| {
            current_save_data.replace_with(|_| Some(loaded_save_data));
        });
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
where
    F: FnOnce(&NativeSaveData) -> R,
{
    CURRENT_SAVEDATA.with(|current_save_data| f(current_save_data.borrow().as_ref().unwrap()))
}

pub fn control_save_data_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut NativeSaveData) -> R,
{
    CURRENT_SAVEDATA.with(|current_save_data| f(current_save_data.borrow_mut().as_mut().unwrap()))
}
