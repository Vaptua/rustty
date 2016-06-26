// Temporary fix before certain constants are used.
#![allow(dead_code)]

use std::io::{Error, ErrorKind};

use term::terminfo::TermInfo;
use term::terminfo::parm;
use term::terminfo::parm::{Param, Variables};

use core::input::Event;

// Array of tuples of events and their corresponding terminal keys.
// Tuples are of the form (event, variable_name, tuple_name).
// Both the variable_name and cap_name are given since terminfo
// uses a combination of variable and cap names.
const KEYS: &'static [(Event, &'static str, &'static str)] = &[
    (Event::Function(1), "key_f1", "kf1"),
    (Event::Function(2), "key_f2", "kf2"),
    (Event::Function(3), "key_f3", "kf3"),
    (Event::Function(4), "key_f4", "kf4"),
    (Event::Function(5), "key_f5", "kf5"),
    (Event::Function(6), "key_f6", "kf6"),
    (Event::Function(7), "key_f7", "kf7"),
    (Event::Function(8), "key_f8", "kf8"),
    (Event::Function(9), "key_f9", "kf9"),
    (Event::Function(10), "key_f10", "kf10"),
    (Event::Function(11), "key_f11", "kf11"),
    (Event::Function(12), "key_f12", "kf12"),
    (Event::Up, "key_up", "kcuu1"),
    (Event::Down, "key_down", "kcud1"),
    (Event::Left, "key_left", "kcub1"),
    (Event::Right, "key_right", "kcuf1"),
    (Event::PageUp, "key_ppage", "kpp"),
    (Event::PageDown, "key_npage", "knp"),
    (Event::Home, "key_home", "khome"),
    (Event::End, "key_end", "kend"),
];

// String constants correspond to terminfo capnames and are used inside the module for convenience.
const ENTER_CA: &'static str = "smcup";
const EXIT_CA: &'static str = "rmcup";
const ENTER_XMIT: &'static str = "smkx";
const EXIT_XMIT: &'static str = "rmkx";
const SHOW_CURSOR: &'static str = "cnorm";
const HIDE_CURSOR: &'static str = "civis";
const SET_CURSOR: &'static str = "cup";
const CLEAR: &'static str = "clear";
const RESET: &'static str = "sgr0";
const UNDERLINE: &'static str = "smul";
const BOLD: &'static str = "bold";
const BLINK: &'static str = "blink";
const REVERSE: &'static str = "rev";
const SETFG: &'static str = "setaf";
const SETBG: &'static str = "setab";

// Driver capabilities are an enum instead of string constants (there are string constants private
// to the module however, those are only used for naming convenience and disambiguation)
// to take advantage of compile-time type-checking instead of hoping invalid strings aren't passed.
// In addition, using an enum means Driver doesn't need hard-coded methods for each capability we
// want to use.
pub enum DevFn {
    EnterCa,
    ExitCa,
    EnterXmit,
    ExitXmit,
    ShowCursor,
    HideCursor,
    SetCursor(usize, usize),
    Clear,
    Reset,
    Underline,
    Bold,
    Blink,
    Reverse,
    SetFg(u8),
    SetBg(u8),
}

impl DevFn {
    fn as_str(&self) -> &'static str {
        match *self {
            DevFn::EnterCa => ENTER_CA,
            DevFn::ExitCa => EXIT_CA,
            DevFn::EnterXmit => ENTER_XMIT,
            DevFn::ExitXmit => EXIT_XMIT,
            DevFn::ShowCursor => SHOW_CURSOR,
            DevFn::HideCursor => HIDE_CURSOR,
            DevFn::SetCursor(..) => SET_CURSOR,
            DevFn::Clear => CLEAR,
            DevFn::Reset => RESET,
            DevFn::Underline => UNDERLINE,
            DevFn::Bold => BOLD,
            DevFn::Blink => BLINK,
            DevFn::Reverse => REVERSE,
            DevFn::SetFg(..) => SETFG,
            DevFn::SetBg(..) => SETBG,
        }
    }
}

pub struct Driver {
    tinfo: TermInfo,
}

impl Driver {
    // Creates a new `Driver`
    pub fn new() -> Result<Driver, Error> {
        let tinfo = try!(TermInfo::from_env());
        Ok(Driver { tinfo: tinfo })
    }

    // Returns the device specific escape sequence for the given `DevFn`, or None if the terminal
    // lacks the capability to perform the specified function.
    pub fn get(&self, dfn: DevFn) -> Option<Vec<u8>> {
        let capname = dfn.as_str();
        self.tinfo.strings.get(capname).map(|cap| {

            match dfn {
                DevFn::SetFg(attr) |
                DevFn::SetBg(attr) => {
                    let params = &[Param::Number(attr as i32)];
                    let mut vars = Variables::new();
                    parm::expand(cap, params, &mut vars).unwrap()
                }
                DevFn::SetCursor(x, y) => {
                    let params = &[Param::Number(y as i32), Param::Number(x as i32)];
                    let mut vars = Variables::new();
                    parm::expand(cap, params, &mut vars).unwrap()
                }
                _ => cap.clone(),
            }
        })
    }
}
