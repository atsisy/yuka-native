pub mod save_data;
pub mod crypt;

use gdnative::prelude::*;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GensoDate {
    pub season: u32,
    pub month: u8,
    pub day: u8,
}

impl GensoDate {
    pub fn new(season: u32, month: u8, day: u8) -> Self {
        GensoDate {
            season: season,
            month: month,
            day: day,
        }
    }

    pub fn new_empty() -> Self {
        GensoDate {
            season: 0,
            month: 0,
            day: 0,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}季 {}月 {}日",
            self.season,
            self.month,
            self.day
        )
    }

    pub fn to_short_string(&self) -> String {
        format!(
            "{}月{}日",
            self.month,
            self.day
        )
    }

    pub fn to_month_string_eng_short(&self) -> String {
        match self.month {
            1 => "Jan.",
            2 => "Feb.",
            3 => "Mar.",
            4 => "Apr.",
            5 => "May",
            6 => "Jun.",
            7 => "Jul.",
            8 => "Aug.",
            9 => "Sep.",
            10 => "Oct.",
            11 => "Nov.",
            12 => "Dec.",
            _ => {
                eprintln!("Invalid month");
                "Dec."
            }
        }
        .to_string()
    }

    pub fn add_day_chain(mut self, day: i32) -> Self {
        self.add_day(day);
        self
    }

    pub fn add_day(&mut self, mut day: i32) {
        static MONTH: [i32; 13] = [0, 31, 28, 31, 30, 30, 30, 31, 31, 30, 31, 30, 31];

        while self.day as i32 + day > MONTH[self.month as usize] {
            day -= MONTH[self.month as usize] - self.day as i32;
            self.day = 0;
            self.month += 1;

            if self.month > 12 {
                self.season += 1;
                self.month %= 12;
            }
        }

        self.day += day as u8;
    }

    ///
    /// self -> 7/1
    /// date2 -> 7/8
    /// return 7
    ///
    pub fn diff_day(&self, date2: &Self) -> i32 {
        static MONTH: [i32; 13] = [0, 31, 28, 31, 30, 30, 30, 31, 31, 30, 31, 30, 31];

        let greater_self = self.month.partial_cmp(&date2.month).unwrap();

        match greater_self {
            std::cmp::Ordering::Less => {
                let mut diff = MONTH[self.month as usize] - self.day as i32;
                for month_index in (self.month + 1)..date2.month {
                    diff += MONTH[month_index as usize];
                }
                diff + date2.day as i32
            }
            std::cmp::Ordering::Equal => {
                let diff = if self.day > date2.day {
                    self.day - date2.day
                } else {
                    date2.day - self.day
                };

                diff as i32
            }
            std::cmp::Ordering::Greater => {
                let mut diff = MONTH[date2.month as usize] - date2.day as i32;
                for month_index in (date2.month + 1)..self.month {
                    diff += MONTH[month_index as usize];
                }
                -(diff + self.day as i32)
            }
        }
    }

    pub fn is_past(&self, date: &GensoDate) -> bool {
        match self.season.cmp(&date.season) {
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Equal => match self.month.cmp(&date.month) {
                std::cmp::Ordering::Less => false,
                std::cmp::Ordering::Greater => true,
                std::cmp::Ordering::Equal => match self.day.cmp(&date.day) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => false,
                    std::cmp::Ordering::Greater => true,
                },
            },
        }
    }

    pub fn is_week_first(&self) -> bool {
        let diff = self.diff_day(&GensoDate::new(112, 7, 23));
	    println!("diff -> {}", diff);
        diff % 7 == 0
    }

    pub fn first_day(&self) -> bool {
        self == &GensoDate::new(112, 7, 23)
    }
}