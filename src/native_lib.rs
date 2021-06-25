#[macro_export]
macro_rules! get_node_assume_safe {
    ($owner: expr, $path: expr) => {
        unsafe {
            $owner.get_node($path).unwrap().assume_safe()
        }
    };
}

#[macro_export]
macro_rules! node_cast_assume_unique {
    ($node: expr, $t: ty) => {
        unsafe {
            $node.cast::<$t>().unwrap().assume_unique()
        }
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
