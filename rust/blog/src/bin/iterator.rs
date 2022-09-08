use std::path::Path;

fn main() {}

fn xxx(mut x: impl Iterator<Item = i32>) {
    let p = x.by_ref().take(3).collect::<Vec<_>>();
    let m = x.count();
}

fn path_can_be_iterated(path: &Path) {
    for part in path {}
}

