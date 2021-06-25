use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct TitleScene;

#[methods]
impl TitleScene {
    fn new(_owner: &Node2D) -> Self {
        TitleScene
    }

    #[export]
    fn _ready(&self, owner: TRef<Node2D>) {
        godot_print!("Some Scene");
	let emitter = owner.get_node("TitleEntries/Main/Start").unwrap();
	let emitter = unsafe { emitter.assume_safe() };
	
	emitter
        .connect("pressed", owner, "button_pressed", VariantArray::new_shared(), 0)
        .unwrap();
    }

    #[export]
    fn _process(&mut self, owner: &Node2D, _delta: f64) {
	    if Input::godot_singleton().is_action_just_released("quit") {
	        crate::quit_game(owner.get_node("/root/Global").unwrap());
	    }
    }

    #[export]
    fn button_pressed(&mut self, owner: &Node2D) {
	    crate::goto_scene(owner, "res://scene/home/Home.tscn");
    }
}


#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct TitleEntries;

#[methods]
impl TitleEntries {
    fn new(_owner: &Node2D) -> Self {
        TitleEntries
    }

    #[export]
    fn _ready(&self, owner: TRef<Node2D>) {
        godot_print!("TitleEntries _ready");
	let emitter = owner.get_node("Main/Start").unwrap();
	let emitter = unsafe { emitter.assume_safe() };

	emitter
        .connect("pressed", owner, "main_start_pressed", VariantArray::new_shared(), 0)
        .unwrap();

	let emitter = owner.get_node("Main/Exit").unwrap();
	let emitter = unsafe { emitter.assume_safe() };

	emitter
        .connect("pressed", owner, "main_exit_pressed", VariantArray::new_shared(), 0)
        .unwrap();
    }

    #[export]
    fn _process(&mut self, _owner: &Node2D, _delta: f64) {
    }

    #[export]
    fn main_start_pressed(&mut self, owner: &Node2D) {
	    crate::goto_scene(owner, "res://scene/home/Home.tscn");
    }

    #[export]
    fn main_exit_pressed(&mut self, owner: &Node2D) {
	    crate::quit_game(owner.get_node("/root/Global").unwrap());
    }
}
