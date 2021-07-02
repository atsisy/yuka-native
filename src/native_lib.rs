pub mod save_data;

use gdnative::prelude::*;

#[macro_export]
macro_rules! get_node_assume_safe {
    ($owner: expr, $path: expr) => {
        unsafe { $owner.get_node($path).unwrap().assume_safe() }
    };
}

#[macro_export]
macro_rules! node_cast_assume_unique {
    ($node: expr, $t: ty) => {
        unsafe { $node.cast::<$t>().unwrap().assume_unique() }
    };
}

#[macro_export]
macro_rules! get_node_auto {
    ($owner: expr, $path: expr, $t: ty) => {
        unsafe {
            $owner
                .get_node($path)
                .unwrap()
                .assume_safe()
                .cast::<$t>()
                .unwrap()
                .assume_unique()
        }
    };
}

pub fn load_scene(path: &str) -> Option<Ref<PackedScene, ThreadLocal>> {
    let scene = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;
    let scene = unsafe { scene.assume_thread_local() };

    scene.cast::<PackedScene>()
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstanceErrors {
    InstancingFailed,
    InvalidType(String),
}

pub fn instance_scene<Root>(scene: &PackedScene) -> Result<Ref<Root, Unique>, InstanceErrors>
where
    Root: gdnative::GodotObject<RefKind = ManuallyManaged> + SubClass<Node>,
{
    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .ok_or(InstanceErrors::InstancingFailed)?;
    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .map_err(|instance| InstanceErrors::InvalidType(instance.name().to_string()))
}
