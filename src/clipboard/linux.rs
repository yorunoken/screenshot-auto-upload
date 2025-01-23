use std::{env, error::Error, io::Write, process::Command};

pub fn set_clipboard_content<T: AsRef<str>>(text: T) -> Result<(), Box<dyn Error>> {
    let text = text.as_ref();
    let is_wayland = env::var("WAYLAND_DISPLAY").is_ok();

    if is_wayland {
        let mut child = Command::new("wl-copy")
            .stdin(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }

        child.wait()?;
    } else {
        let mut child = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }

        child.wait()?;
    }

    Ok(())
}
