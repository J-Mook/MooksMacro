extern crate winres;

fn main() {
    // Windows 플랫폼이 아니면 아무 것도 하지 않습니다.
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("asset/keyboard_icon.ico");
        res.compile().unwrap();
    }
}