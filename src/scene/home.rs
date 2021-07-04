use gdnative::{
    api::{Container, TextureButton},
    prelude::*,
};

use crate::{get_node_auto, native_lib::{save_data::{Item, SoilItem, control_save_data, control_save_data_mut}}};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Calendar;

#[methods]
impl Calendar {
    fn new(_owner: &Node2D) -> Self {
        Calendar
    }

    #[export]
    fn _ready(&self, owner: &Node2D) {
        godot_print!("Calendar ready");
        self.set_month(owner, 1);
        self.set_day(owner, 1);
    }

    #[export]
    fn set_month(&self, owner: &Node2D, m: i32) {
        let m1 = crate::get_node_assume_safe!(owner, "M1");
        unsafe {
            m1.call("set_number", &[(m % 10).to_variant()]);
        }

        let m10 = crate::get_node_assume_safe!(owner, "M10");
        unsafe {
            m10.call("set_number", &[(m / 10).to_variant()]);
        }
    }

    #[export]
    fn set_day(&self, owner: &Node2D, day: i32) {
        let d1 = crate::get_node_assume_safe!(owner, "D1");
        unsafe {
            d1.call("set_number", &[(day % 10).to_variant()]);
        }

        let d10 = crate::get_node_assume_safe!(owner, "D10");
        unsafe {
            d10.call("set_number", &[(day / 10).to_variant()]);
        }
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct MagicBoardHome;

#[methods]
impl MagicBoardHome {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "move_mb_contents",
            args: &[
                SignalArgument {
                    name: "before",
                    default: Variant::from_str("None"),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "after",
                    default: Variant::from_str("None"),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::DEFAULT,
                },
            ],
        });
    }

    pub fn new(_owner: &Node2D) -> MagicBoardHome {
        MagicBoardHome
    }

    #[export]
    fn _ready(&self, owner: TRef<Node2D>) {
        let profile_button = get_node_auto!(owner, "Scroll/VBox/Line1/Profile", TextureButton);
        profile_button
            .connect(
                "pressed",
                owner,
                "profile_pressed",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        let save_button = get_node_auto!(owner, "Scroll/VBox/Line2/Save", TextureButton);
        save_button
            .connect(
                "pressed",
                owner,
                "save_pressed",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        let date = get_node_auto!(owner, "Date", Label);
        control_save_data(|save_data| {
            date.set_text(GodotString::from_str(
                save_data.get_date().to_short_string(),
            ));
        });
    }

    #[export]
    fn profile_pressed(&self, owner: &Node2D) {
        owner.emit_signal(
            "move_mb_contents",
            &[Variant::from_str("Home"), Variant::from_str("ItemList")],
        );
    }

    #[export]
    fn save_pressed(&self, owner: &Node2D) {
        owner.emit_signal(
            "move_mb_contents",
            &[Variant::from_str("Home"), Variant::from_str("SaveEntrance")],
        );
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct MagicBoard;

#[methods]
impl MagicBoard {
    fn new(_owner: &Node2D) -> Self {
        MagicBoard
    }

    #[export]
    fn _ready(&self, owner: TRef<Node2D>) {
        godot_print!("MagicBoard ready");

        let mb_home = get_node_auto!(owner, "Background/MBHome", Node2D);
        mb_home
            .connect(
                "move_mb_contents",
                owner,
                "move_mb_contents_handler",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        let save_entrance = get_node_auto!(owner, "Background/SaveEntrance", Node2D);
        save_entrance
            .connect(
                "move_mb_contents",
                owner,
                "move_mb_contents_handler",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        let save_app = get_node_auto!(owner, "Background/MBSaveApp", Node2D);
        save_app
            .connect(
                "move_mb_contents",
                owner,
                "move_mb_contents_handler",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        let load_app = get_node_auto!(owner, "Background/MBLoadApp", Node2D);
        load_app
            .connect(
                "move_mb_contents",
                owner,
                "move_mb_contents_handler",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        let item_list = get_node_auto!(owner, "Background/ItemList", Node2D);
        item_list
            .connect(
                "move_mb_contents",
                owner,
                "move_mb_contents_handler",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
    }

    fn set_child_node_visibility(&self, owner: &Node2D, name: &str, visible: bool) {
        let target_node = match name {
            "Home" => get_node_auto!(owner, "Background/MBHome", Node2D),
            "SaveEntrance" => get_node_auto!(owner, "Background/SaveEntrance", Node2D),
            "SaveApp" => get_node_auto!(owner, "Background/MBSaveApp", Node2D),
            "LoadApp" => get_node_auto!(owner, "Background/MBLoadApp", Node2D),
            "ItemList" => get_node_auto!(owner, "Background/ItemList", Node2D),
            _ => return,
        };

        if visible {
            target_node.show();
        } else {
            target_node.hide();
        }
    }

    #[export]
    fn move_mb_contents_handler(&self, owner: &Node2D, prev: Variant, next: Variant) {
        let prev = prev.try_to_string().unwrap();
        let next = next.try_to_string().unwrap();

        godot_print!("MagicBoard: trans {} -> {}", prev, next);

        if prev == next {
            // シーン遷移元と先が同じなので、遷移しない
            return;
        }

        self.set_child_node_visibility(owner, prev.as_str(), false);
        self.set_child_node_visibility(owner, next.as_str(), true);

        let target = match next.as_str() {
            "Home" => get_node_auto!(owner, "Background/MBHome", Node2D),
            "SaveEntrance" => get_node_auto!(owner, "Background/SaveEntrance", Node2D),
            "SaveApp" => get_node_auto!(owner, "Background/MBSaveApp", Node2D),
            "LoadApp" => get_node_auto!(owner, "Background/MBLoadApp", Node2D),
            "ItemList" => get_node_auto!(owner, "Background/ItemList", Node2D),
            _ => return,
        };
        unsafe {
            if target.has_method("root_app_update_handler") {
                target.call("root_app_update_handler", &[]);
            }
        }
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct MBSaveEntrance;

#[methods]
impl MBSaveEntrance {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "move_mb_contents",
            args: &[
                SignalArgument {
                    name: "before",
                    default: Variant::from_str("None"),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "after",
                    default: Variant::from_str("None"),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::DEFAULT,
                },
            ],
        });
    }

    fn new(_owner: &Node2D) -> Self {
        MBSaveEntrance
    }

    #[export]
    fn _ready(&self, owner: TRef<Node2D>) {
        godot_print!("MBSaveEntrance ready");

        let save = get_node_auto!(owner, "Save", Button);
        save.connect(
            "pressed",
            owner,
            "save_button_pressed",
            VariantArray::new_shared(),
            0,
        )
        .unwrap();

        let load = get_node_auto!(owner, "Load", Button);
        load.connect(
            "pressed",
            owner,
            "load_button_pressed",
            VariantArray::new_shared(),
            0,
        )
        .unwrap();

        let back = get_node_auto!(owner, "Back", TextureButton);
        back.connect(
            "pressed",
            owner,
            "back_button_pressed",
            VariantArray::new_shared(),
            0,
        )
        .unwrap();
    }

    #[export]
    fn save_button_pressed(&self, owner: &Node2D) {
        owner.emit_signal(
            "move_mb_contents",
            &[
                Variant::from_str("SaveEntrance"),
                Variant::from_str("SaveApp"),
            ],
        );
    }

    #[export]
    fn load_button_pressed(&self, owner: &Node2D) {
        owner.emit_signal(
            "move_mb_contents",
            &[
                Variant::from_str("SaveEntrance"),
                Variant::from_str("LoadApp"),
            ],
        );
    }

    #[export]
    fn back_button_pressed(&self, owner: &Node2D) {
        owner.emit_signal(
            "move_mb_contents",
            &[Variant::from_str("SaveEntrance"), Variant::from_str("Home")],
        );
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct MBSaveApp;

#[methods]
impl MBSaveApp {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "move_mb_contents",
            args: &[
                SignalArgument {
                    name: "before",
                    default: Variant::from_str("None"),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "after",
                    default: Variant::from_str("None"),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::DEFAULT,
                },
            ],
        });
    }

    fn new(_owner: &Node2D) -> Self {
        MBSaveApp
    }

    #[export]
    fn _ready(&self, owner: TRef<Node2D>) {
        godot_print!("MBSaveApp ready");

        let back = get_node_auto!(owner, "Back", TextureButton);
        back.connect(
            "pressed",
            owner,
            "back_button_pressed",
            VariantArray::new_shared(),
            0,
        )
        .unwrap();

        let save_data_set = get_node_auto!(owner, "Scroll/VBox/SaveDataSet", Node2D);
        unsafe {
            save_data_set.call("set_mode", &[Variant::from_bool(true)]);
        }
    }

    #[export]
    fn back_button_pressed(&self, owner: &Node2D) {
        owner.emit_signal(
            "move_mb_contents",
            &[
                Variant::from_str("SaveApp"),
                Variant::from_str("SaveEntrance"),
            ],
        );
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct SaveEntry {
    save_mode: bool,
}

#[methods]
impl SaveEntry {
    fn new(_owner: &Node2D) -> Self {
        SaveEntry { save_mode: true }
    }

    #[export]
    fn set_mode(&mut self, _owner: TRef<Node2D>, save_mode: Variant) {
        self.save_mode = save_mode.to_bool();
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node2D>) {
        godot_print!("SaveEntry ready");

        let action = get_node_auto!(owner, "Button", Button);
        action
            .connect(
                "pressed",
                owner,
                "action_button_pressed",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
    }

    #[export]
    fn action_button_pressed(&self, owner: &Node2D) {
        let save_data_manager = get_node_auto!(owner, "/root/SaveDataManager", Node);

        if self.save_mode {
            unsafe {
                save_data_manager.call("save", &[Variant::from_godot_string(&owner.name())]);
            }
        } else {
            unsafe {
                save_data_manager.call("load", &[Variant::from_godot_string(&owner.name())]);
            }
        }
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct SaveDataSet;

#[methods]
impl SaveDataSet {
    fn new(_owner: &Node2D) -> Self {
        SaveDataSet
    }

    #[export]
    fn _ready(&self, _owner: TRef<Node2D>) {
        godot_print!("SaveDataSet ready");
    }

    #[export]
    fn set_mode(&self, owner: &Node2D, save_mode: Variant) {
        for entry in [
            "VBox/Entry1",
            "VBox/Entry2",
            "VBox/Entry3",
            "VBox/Entry4",
            "VBox/Entry5",
            "VBox/Entry6",
        ] {
            let node = get_node_auto!(owner, entry, Node2D);
            unsafe {
                node.call("set_mode", &[save_mode.clone()]);
            }
        }
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct MBLoadApp;

#[methods]
impl MBLoadApp {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "move_mb_contents",
            args: &[
                SignalArgument {
                    name: "before",
                    default: Variant::from_str("None"),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "after",
                    default: Variant::from_str("None"),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::DEFAULT,
                },
            ],
        });
    }

    fn new(_owner: &Node2D) -> Self {
        MBLoadApp
    }

    #[export]
    fn _ready(&self, owner: TRef<Node2D>) {
        godot_print!("MBSaveApp ready");

        let back = get_node_auto!(owner, "Back", TextureButton);
        back.connect(
            "pressed",
            owner,
            "back_button_pressed",
            VariantArray::new_shared(),
            0,
        )
        .unwrap();

        let save_data_set = get_node_auto!(owner, "Scroll/VBox/SaveDataSet", Node2D);
        unsafe {
            save_data_set.call("set_mode", &[Variant::from_bool(false)]);
        }
    }

    #[export]
    fn back_button_pressed(&self, owner: &Node2D) {
        owner.emit_signal(
            "move_mb_contents",
            &[
                Variant::from_str("LoadApp"),
                Variant::from_str("SaveEntrance"),
            ],
        );
    }
}

#[derive(NativeClass)]
#[inherit(Container)]
pub struct MBItemEntry;

#[methods]
impl MBItemEntry {
    fn new(_owner: &Container) -> Self {
        MBItemEntry
    }

    #[export]
    fn _ready(&mut self, _owner: TRef<Container>) {
        godot_print!("MBItemEntry ready");
    }

    #[export]
    fn set_name(&self, owner: TRef<Container>, name: GodotString) {
        let name_label = get_node_auto!(owner, "Name", Label);
        name_label.set_text(name);
    }

    #[export]
    fn set_count(&self, owner: TRef<Container>, count: Variant) {
        let count_label = get_node_auto!(owner, "Count", Label);
        count_label.set_text(GodotString::from_str(&format!("x{}", count.to_u64())));
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct MBItemList {
    page: usize,
}

#[methods]
impl MBItemList {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "move_mb_contents",
            args: &[
                SignalArgument {
                    name: "before",
                    default: Variant::from_str("None"),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "after",
                    default: Variant::from_str("None"),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::DEFAULT,
                },
            ],
        });
    }

    fn new(_owner: &Node2D) -> Self {
        MBItemList { page: 0 }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node2D>) {
        godot_print!("MBItemList ready");

        let back_button = get_node_auto!(owner, "Back", TextureButton);
        back_button
            .connect(
                "pressed",
                owner,
                "back_button_pressed",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        get_node_auto!(owner, "WholeVBox/PageButtonContainer/Next", Button)
            .connect(
                "pressed",
                owner,
                "next_button_pressed",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        get_node_auto!(owner, "WholeVBox/PageButtonContainer/Prev", Button)
            .connect(
                "pressed",
                owner,
                "prev_button_pressed",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        get_node_auto!(owner, "Back", TextureButton)
            .connect(
                "pressed",
                owner,
                "next_button_pressed",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        self.update_item_list(owner);
    }

    fn hide_all_item_entries(&self, owner: TRef<Node2D>) {
        for i in 1..=6 {
            get_node_auto!(owner, format!("WholeVBox/ItemListVBox/Line{}", i).as_str(), Container).hide();
        }
    }

    fn update_item_list(&self, owner: TRef<Node2D>) {
        self.hide_all_item_entries(owner);
        control_save_data_mut(|save_data| {
            save_data.add_items(Item::Soil(SoilItem::Fuyodo), 2);
            for (line, data) in save_data.get_items().iter().enumerate().take(6) {
                let key_str = format!("WholeVBox/ItemListVBox/Line{}", line + 1);
                let item_entry = get_node_auto!(owner, key_str.as_str(), Container);

                item_entry.show();

                unsafe {
                    item_entry.call("set_name", &[Variant::from_str(data.0.get_display_name())]);
                    item_entry.call("set_count", &[Variant::from_u64(*data.1 as u64)]);
                }
            }
        });
    }

    #[export]
    fn back_button_pressed(&self, owner: &Node2D) {
        owner.emit_signal(
            "move_mb_contents",
            &[Variant::from_str("ItemList"), Variant::from_str("Home")],
        );
    }

    #[export]
    fn next_button_pressed(&self, owner: &Node2D) {
        owner.emit_signal(
            "move_mb_contents",
            &[Variant::from_str("ItemList"), Variant::from_str("Home")],
        );
    }

    #[export]
    fn prev_button_pressed(&self, owner: &Node2D) {
        owner.emit_signal(
            "move_mb_contents",
            &[Variant::from_str("ItemList"), Variant::from_str("Home")],
        );
    }

    #[export]
    fn root_app_update_handler(&self, owner: TRef<Node2D>) {
        self.update_item_list(owner);
    }
}
