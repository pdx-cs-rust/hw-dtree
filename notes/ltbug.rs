fn main() {
    let mut dt = DTree::new();
    let s = String::new("x");
    dt.with_subdir_mut(&[&s], |d| println!("{:?}", d));
    drop(s);
    println!("{:#?}", dt.paths());
}
