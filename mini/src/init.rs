use std::os::uefi as uefi_std;
use uefi::{Handle, boot};

pub fn init() {
    let st = uefi_std::env::system_table();
    let ih = uefi_std::env::image_handle();
    unsafe {
        uefi::table::set_system_table(st.as_ptr().cast());
        let ih = Handle::from_ptr(ih.as_ptr().cast()).unwrap();
        boot::set_image_handle(ih);
    }
}
