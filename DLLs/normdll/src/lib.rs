use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA};
use windows::Win32::Foundation::HWND;
use windows::s;

#[no_mangle]
pub unsafe extern "C" fn startDLL()
{
    MessageBoxA(HWND(0), s!("normDLLmessage"), s!("normDLLtitle"), Default::default());
    // let hwnd : HWND = windows::Win32::Foundation::HWND(0);
    // MessageBoxA( HWND::from(hwnd), 
    //     // std::ptr::null_mut().into(),
    // PCSTR::from_raw("normDLLmessage\0".as_ptr() as *const u8).into(),
    // PCSTR::from_raw("normDLLtitle\0".as_ptr() as *const u8).into(),
    //  windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE(0)).into();
}



// use std::ffi::CString;
// use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE};

// #[no_mangle]
// pub extern "C" fn startDLL() {
//     let title = CString::new("normDLLtitle").unwrap();
//     let message = CString::new("normDLLmessage").unwrap();
//     windows::Win32::Foundation::HWND();
//     unsafe {
//         MessageBoxA((std::ptr::null_mut() as *mut i32).into(),
//          (message.as_ptr() as *const i32).into(),
//           (title.as_ptr() as *const i32).into(), 
//           MESSAGEBOX_STYLE(0));
//     }
// }