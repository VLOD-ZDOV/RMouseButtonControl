#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("../gui/resources/mouse.ico");
    res.set("FileDescription", "RMouseButtonControl Background Daemon");
    res.set("ProductName", "RMouseButtonControl");
    res.set("OriginalFilename", "rmousebuttoncontrol.exe");
    if let Err(e) = res.compile() {
        eprintln!("Failed to compile Windows resource: {e}");
    }
}

#[cfg(not(windows))]
fn main() {}
