use gdnative::{
    api::{File, RichTextLabel},
    prelude::*,
};

use crate::get_node_auto;

pub struct Serif {
    speaker: String,
    lines: Vec<String>,
    current_line: usize,
}

impl Serif {
    pub fn new(speaker: String, lines: Vec<String>) -> Self {
        Serif {
            speaker: speaker,
            lines: lines,
            current_line: 0,
        }
    }

    pub fn current_line_len(&self) -> usize {
        let index = self.current_line % self.lines.len();
        self.lines[index].len()
    }

    pub fn current_line(&self) -> &str {
        let index = self.current_line % self.lines.len();
        self.lines.get(index).unwrap()
    }

    pub fn next_line(&mut self) {
        self.current_line += 1;
    }

    pub fn finish(&self) -> bool {
        self.lines.len() <= self.current_line
    }

    pub fn get_speaker_name(&self) -> &str {
        self.speaker.as_str()
    }
}

pub struct Dialogue {
    text: Vec<Serif>,
    current_line: usize,
}

impl Dialogue {
    pub fn new(dialogue_file_path: String) -> Self {
        let mut text = Vec::new();

        let f = File::new();
        f.open(dialogue_file_path, File::READ)
            .expect("failed open file");
        let s = f.get_as_text().to_string();

        for line in s.lines() {
            text.push(Serif::new("スピーカー".to_string(), vec![line.to_string()]));
        }

        Dialogue {
            text: text,
            current_line: 0,
        }
    }

    pub fn next_line(&mut self) {
        self.current_line += 1;
    }

    pub fn get_current_serif(&self) -> &Serif {
        let index = self.current_line % self.text.len();
        self.text.get(index).unwrap()
    }

    pub fn get_current_serif_mut(&mut self) -> &mut Serif {
        let index = self.current_line % self.text.len();
        self.text.get_mut(index).unwrap()
    }

    pub fn finish(&self) -> bool {
        self.text.len() <= self.current_line + 1
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct TextBox {
    dialogue: Dialogue,
    seeker: usize,
    current_buffer: String,
}

#[methods]
impl TextBox {
    fn new(_owner: &Node2D) -> Self {
        godot_print!("TextBox::new");

        let dialogue = Dialogue::new("res://resources/scenario/sample.txt".to_string());
        let current_buffer = dialogue.get_current_serif().lines[0].to_string();

        godot_print!("{}", current_buffer);

        TextBox {
            dialogue: dialogue,
            seeker: 0,
            current_buffer: current_buffer,
        }
    }

    #[export]
    fn _ready(&self, owner: TRef<Node2D>) {
        godot_print!("TextBox::_ready");

        //let main_text_timer = get_node_assume_safe!(owner, "Background/MainText/Timer");
        //let main_text_timer = node_cast_assume_unique!(main_text_timer, Timer);
        let main_text_timer = get_node_auto!(owner, "MainText/Timer", Timer);

        main_text_timer
            .connect(
                "timeout",
                owner,
                "maintext_timeout",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
        main_text_timer.start(0.1);
        main_text_timer.set_one_shot(false);

        let name_text = get_node_auto!(owner, "Name", RichTextLabel);
        name_text.set_bbcode(
            self.dialogue
                .get_current_serif()
                .get_speaker_name()
                .to_string(),
        );

        godot_print!("TextBox::_ready done");
    }

    #[export]
    fn _process(&mut self, owner: &Node2D, _delta: f64) {
        if Input::godot_singleton().is_action_just_released("ui_accept") {
            let current_buffer_len = self.current_buffer.chars().count();

            // Serif内の一行の表示が完了している？
            if self.seeker < current_buffer_len {
                // していないから、最後まで表示する
                self.seeker = current_buffer_len;
            } else {
                // 最後まで行ってるから次の一行にしたい
                // とりあえずseekerを0にする
                self.seeker = 0;
                self.dialogue.get_current_serif_mut().next_line();

                // 現在のSerifの全ての行が終わった？
                if self.dialogue.get_current_serif().finish() {
                    // 終わっているので次のSerifを取り出したいが、
                    // dialogueも最後まで行ってる可能性があるので分岐
                    if self.dialogue.finish() {
                        // 最後の行まで行っていたので何もしない。
                        godot_print!("end");

                        // 0にリセットされていたseekerを末尾に戻す
                        self.seeker = current_buffer_len;

                        // 更新を中断
                        self.stop_text_update(owner);
                    } else {
                        // まだ次の行がある。
                        godot_print!("next serif");
                        self.dialogue.next_line();
                        self.current_buffer =
                            self.dialogue.get_current_serif().current_line().to_string();
                        self.set_speaker_text(
                            owner,
                            self.dialogue
                                .get_current_serif()
                                .get_speaker_name()
                                .to_string(),
                        );
                    }
                } else {
                    // まだSerifが終わっていないので
                    // 次の行をロードする
                    self.current_buffer =
                        self.dialogue.get_current_serif().current_line().to_string();
                }
            }
        }
    }

    #[export]
    fn maintext_timeout(&mut self, owner: &Node2D) {
        if self.current_buffer.chars().count() >= self.seeker {
            let s = self
                .current_buffer
                .chars()
                .take(self.seeker)
                .collect::<String>();
            self.set_main_text(owner, s);
        }
        self.seeker += 1;
    }

    fn set_main_text(&mut self, owner: &Node2D, text: String) {
        let main_text = get_node_auto!(owner, "MainText", RichTextLabel);

        main_text.set_bbcode(text);
    }

    fn set_speaker_text(&mut self, owner: &Node2D, name: String) {
        let name_text = get_node_auto!(owner, "Name", RichTextLabel);

        name_text.set_bbcode(name);
    }

    fn stop_text_update(&mut self, owner: &Node2D) {
        let main_text_timer = get_node_auto!(owner, "MainText/Timer", Timer);
        main_text_timer.set_one_shot(true);
    }
}
