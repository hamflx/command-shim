use std::{ffi::CStr, process::Command};

fn main() {
    let real_command =
        b"<MAGIC_STRING_START______________________________________________MAGIC_STRING_END>\0";
    let real_command = unsafe { CStr::from_ptr(real_command.as_ptr() as _) };
    let real_command = real_command.to_str().unwrap();
    if let Some(code) = Command::new(real_command)
        .args(std::env::args().skip(1))
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .code()
    {
        std::process::exit(code);
    }
}
