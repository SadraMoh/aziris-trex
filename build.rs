#[cfg(windows)]
extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("favicon.ico");
    res.compile().unwrap();
}