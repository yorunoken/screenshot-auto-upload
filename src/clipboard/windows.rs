use std::{ffi::CString, ptr};

use winapi::shared::minwindef::{HGLOBAL, UINT};
use winapi::um::{
    winbase::GlobalFree,
    winuser::{
        CloseClipboard, EmptyClipboard, GlobalAlloc, GlobalLock, GlobalUnlock, OpenClipboard,
        SetClipboardData, CF_TEXT, GMEM_MOVEABLE,
    },
};

pub fn set_clipboard_content<T: AsRef<str>>(text: T) -> Result<(), String> {
    let text = text.as_ref();

    unsafe {
        if OpenClipboard(ptr::null_mut()) == 0 {
            return Err("Failed to open clipboard".into());
        }

        if EmptyClipboard() == 0 {
            CloseClipboard();
            return Err("Failed to empty clipboard".into());
        }

        let c_text = CString::new(text).map_err(|_| "Failed to create CString")?;
        let h_global: HGLOBAL = GlobalAlloc(GMEM_MOVEABLE, c_text.as_bytes_with_nul().len());
        if h_global.is_null() {
            CloseClipboard();
            return Err("Failed to allocate global memory".to_string());
        }

        let ptr = GlobalLock(h_global);
        if ptr.is_null() {
            GlobalFree(h_global);
            CloseClipboard();
            return Err("Failed to set clipboard data".to_string());
        }

        ptr::copy_nonoverlapping(
            c_text.as_ptr(),
            ptr as *mut _,
            c_text.as_bytes_with_nul().len(),
        );
        GlobalUnlock(h_global);

        if SetClipboardData(CF_TEXT as UINT, h_global as *mut _).is_null() {
            GlobalFree(h_global);
            CloseClipboard();
            return Err("Failed to set clipboard data".to_string());
        }

        CloseClipboard();
    }
    Ok(())
}
