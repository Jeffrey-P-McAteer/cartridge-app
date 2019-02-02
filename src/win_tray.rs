//-----Import Libraries (called crates)-----
use winapi;
//-----Import Built-in Libraries (not called crates)-----
use std::process::Command; //use cmd.exe
use std::mem::{size_of, zeroed}; //get size of stuff and init with zeros
use std::ptr::null_mut; //use a null pointer (I think)
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub fn run()
{
// to navigate calling with the winapi "crate" use the search function at link
// https://docs.rs/winapi/*/x86_64-pc-windows-msvc/winapi/um/wincon/fn.GetConsoleWindow.html
let hWnd = unsafe { winapi::um::wincon::GetConsoleWindow }; //gets the current console window handle

//System Tray Icon support - here it is
let WM_MYMESSAGE = winapi::um::winuser::WM_APP + 100; //prep WM_MYMESSAGE
let mut trayToolTip = "Tool tip words here".to_string(); //record tooltip words for the icon
let mut trayToolTipInt: [u16; 128] = [0; 128]; //fill with 0's
let trayToolTipStrStep: &str = &*trayToolTip; //these two types of strings
let mut trayToolTipStepOS = OsStr::new(trayToolTipStrStep); //convert to OS string format or something
let mut trayToolTipStepUTF16 = trayToolTipStepOS.encode_wide().collect::<Vec<u16>>(); //now actually convert to UTF16 format for the OS
trayToolTipInt[..trayToolTipStepUTF16.len()].copy_from_slice(&trayToolTipStepUTF16); //record it in that nice integer holder

let mut nid: winapi::um::shellapi::NOTIFYICONDATAW = unsafe{ zeroed() }; //thing that has info on window and system tray stuff in it 
unsafe
{
    nid.cbSize = size_of::<winapi::um::shellapi::NOTIFYICONDATAW>() as u32; //prep
    nid.hWnd = hWnd(); //links the console window
    nid.uID = 1001; //it's a number
    nid.uCallbackMessage = WM_MYMESSAGE; //whoknows should be related to click capture but doesn't so
    nid.hIcon = winapi::um::winuser::LoadIconW(null_mut(), winapi::um::winuser::IDI_APPLICATION); //icon idk
    nid.szTip = trayToolTipInt; //tooltip for the icon
    nid.uFlags = winapi::um::shellapi::NIF_MESSAGE | winapi::um::shellapi::NIF_ICON | winapi::um::shellapi::NIF_TIP; //who knows
};

//let mut nidszTipLength = trayToolTip.chars().count() as u64; //gets the size of nid.szTip (tooltip length) indirectly (not the right size!)
let mut nidszTipLength = trayToolTipStepUTF16.len() as u64; //gets the size of nid.szTip (tooltip length) for the UTF-16 format, which is what Windows cares about

unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_ADD, &mut nid) }; //shows the icon
let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();

trayToolTip = "An updated tooltip is now here!".to_string(); //update the tooltip string
trayToolTipInt = [0; 128]; //fill with 0's (clear it out I hope)
let trayToolTipStrStep: &str = &*trayToolTip; //these two types of strings are hella annoying
trayToolTipStepOS = OsStr::new(trayToolTipStrStep); //convert to OS string format or something
trayToolTipStepUTF16 = trayToolTipStepOS.encode_wide().collect::<Vec<u16>>(); //now actually convert to UTF16 format for the OS
trayToolTipInt[..trayToolTipStepUTF16.len()].copy_from_slice(&trayToolTipStepUTF16); //record it in that nice integer holder
nid.szTip = trayToolTipInt; //tooltip for the icon
//nidszTipLength = trayToolTip.chars().count() as u64; //gets the size of nid.szTip (tooltip length) indirectly (not the right size!)
nidszTipLength = trayToolTipStepUTF16.len() as u64; //gets the size of nid.szTip (tooltip length) for the UTF-16 format, which is what Windows cares about
unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_MODIFY, &mut nid) }; //updates system tray icon

let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();

unsafe{ winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid) }; //deletes system tray icon when done

let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();

}