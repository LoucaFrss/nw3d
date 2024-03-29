use crate::TextBuf;
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
#[derive(Debug)]
pub struct Color(pub u16);
impl Color {
    pub const BLACK: Color = rgb(0, 0, 0);
    pub const WHITE: Color = rgb(255, 255, 255);
    pub const RED: Color = rgb(255, 0, 0);
    pub const GREEN: Color = rgb(0, 255, 0);
    pub const BLUE: Color = rgb(0, 0, 255);
    pub const CYAN: Color = rgb(255, 255, 0);
    pub const YELLOW: Color = rgb(0, 255, 255);
    pub const MAGENTA: Color = rgb(255, 0, 255);
}
#[inline]

pub const fn rgb(r: u16, g: u16, b: u16) -> Color {
    Color(((r & 0b11111000) << 8) + ((g & 0b11111100) << 3) + (b >> 3))
}

#[macro_export]
macro_rules! rgb {
    ($r: expr, $g: expr, $b: expr) => {
        ((($r as u16 & 0b11111000) << 8) + (($g as u16 & 0b11111100) << 3) + ($b as u16 >> 3))
    };
}

#[repr(C)]
#[derive(Default, Clone, PartialEq, PartialOrd)]

pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

pub const SCREEN: Rect = Rect {
    x: 0,
    y: 0,
    w: 320,
    h: 240,
};
#[repr(C)]
#[derive(Default, Clone, PartialEq, PartialOrd, Copy)]

pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub mod backlight {
    pub fn set_brightness(brightness: u8) {
        unsafe {
            eadk_backlight_set_brightness(brightness);
        }
    }
    pub fn brightness() -> u8 {
        unsafe { eadk_backlight_brightness() }
    }

    extern "C" {
        fn eadk_backlight_set_brightness(brightness: u8);
        fn eadk_backlight_brightness() -> u8;
    }
}

pub mod display {

    use super::Color;
    use super::Point;
    use super::Rect;

    pub fn push_rect(rect: Rect, pixels: &[Color]) {
        unsafe {
            eadk_display_push_rect(rect, pixels.as_ptr());
        }
    }

    pub fn push_rect_uniform(rect: Rect, color: Color) {
        unsafe {
            eadk_display_push_rect_uniform(rect, color);
        }
    }

    pub fn wait_for_vblank() {
        unsafe {
            eadk_display_wait_for_vblank();
        }
    }
    pub fn draw_string(
        text: &[u8],
        point: Point,
        large_format: bool,
        text_color: Color,
        background_color: Color,
    ) {
        unsafe {
            eadk_display_draw_string(
                text.as_ptr(),
                point,
                large_format,
                text_color,
                background_color,
            )
        }
    }

    extern "C" {
        fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
        fn eadk_display_push_rect(rect: Rect, color: *const Color);
        fn eadk_display_wait_for_vblank();
        fn eadk_display_draw_string(
            text: *const u8,
            point: Point,
            large_format: bool,
            text_color: Color,
            background_color: Color,
        );
    }
}

pub mod timing {
    pub fn usleep(us: u32) {
        unsafe {
            eadk_timing_usleep(us);
        }
    }

    pub fn msleep(ms: u32) {
        unsafe {
            eadk_timing_msleep(ms);
        }
    }

    pub fn millis() -> u64 {
        unsafe { eadk_timing_millis() }
    }

    extern "C" {
        fn eadk_timing_usleep(us: u32);
        fn eadk_timing_msleep(us: u32);
        fn eadk_timing_millis() -> u64;
    }
}

pub fn random() -> u32 {
    unsafe { eadk_random() }
}
extern "C" {
    pub static eadk_external_data: *const u8;
    pub static eadk_external_data_size: c_size_t;
}

impl<'a> core::fmt::Write for TextBuf<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();

        // ckip over already-copied data
        let remainder = &mut self.buf[self.offset..];
        // check if there is space remaining (return error instead of panicking)
        if remainder.len() < bytes.len() {
            return Err(core::fmt::Error);
        }
        // cake the two slices the same length
        let remainder = &mut remainder[..bytes.len()];
        // copy
        remainder.copy_from_slice(bytes);

        // update offset to avoid overwriting
        self.offset += bytes.len();

        Ok(())
    }
}
#[macro_export]
macro_rules! println {
    ($($arg:expr),*) => {
{
    use ::core::fmt::Write;
    let mut buf = $crate::TextBuf::new(unsafe {&mut $crate::TEXT_BUFFER});
    write!(buf, $($arg),*).unwrap();
    $crate::eadk::display::draw_string(unsafe {&$crate::TEXT_BUFFER[..buf.offset]}, $crate::eadk::Point{x: 0, y:0}, false, Color(0), Color(u16::MAX));
}
    };
}
extern "C" {
    fn eadk_random() -> u32;
}
pub mod input {
    type EadkKeyboardState = u64;

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Key {
        Left = 0,
        Up = 1,
        Down = 2,
        Right = 3,
        Ok = 4,
        Back = 5,
        Home = 6,
        OnOff = 8,
        Shift = 12,
        Alpha = 13,
        Xnt = 14,
        Var = 15,
        Toolbox = 16,
        Backspace = 17,
        Exp = 18,
        Ln = 19,
        Log = 20,
        Imaginary = 21,
        Comma = 22,
        Power = 23,
        Sine = 24,
        Cosine = 25,
        Tangent = 26,
        Pi = 27,
        Sqrt = 28,
        Square = 29,
        Seven = 30,
        Eight = 31,
        Nine = 32,
        LeftParenthesis = 33,
        RightParenthesis = 34,
        Four = 36,
        Five = 37,
        Six = 38,
        Multiplication = 39,
        Division = 40,
        One = 42,
        Two = 43,
        Three = 44,
        Plus = 45,
        Minus = 46,
        Zero = 48,
        Dot = 49,
        Ee = 50,
        Ans = 51,
        Exe = 52,
    }

    extern "C" {
        fn eadk_keyboard_scan() -> EadkKeyboardState;
    }

    #[derive(Clone, Copy)]
    pub struct KeyboardState(EadkKeyboardState);

    impl KeyboardState {
        pub fn scan() -> Self {
            Self::from_raw(unsafe { eadk_keyboard_scan() })
        }

        pub const fn from_raw(state: EadkKeyboardState) -> Self {
            Self(state)
        }

        pub fn key_down(&self, key: Key) -> bool {
            (self.0 >> (key as u8)) & 1 != 0
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(u16)]
    pub enum Event {
        Left = 0,
        Up = 1,
        Down = 2,
        Right = 3,
        Ok = 4,
        Back = 5,
        Shift = 12,
        Alpha = 13,
        Xnt = 14,
        Var = 15,
        Toolbox = 16,
        Backspace = 17,
        Exp = 18,
        Ln = 19,
        Log = 20,
        Imaginary = 21,
        Comma = 22,
        Power = 23,
        Sine = 24,
        Cosine = 25,
        Tangent = 26,
        Pi = 27,
        Sqrt = 28,
        Square = 29,
        Seven = 30,
        Eight = 31,
        Nine = 32,
        LeftParenthesis = 33,
        RightParenthesis = 34,
        Four = 36,
        Five = 37,
        Six = 38,
        Multiplication = 39,
        Division = 40,
        One = 42,
        Two = 43,
        Three = 44,
        Plus = 45,
        Minus = 46,
        Zero = 48,
        Dot = 49,
        Ee = 50,
        Ans = 51,
        Exe = 52,
        ShiftLeft = 54,
        ShiftUp = 55,
        ShiftDown = 56,
        ShiftRight = 57,
        AlphaLock = 67,
        Cut = 68,
        Copy = 69,
        Paste = 70,
        Clear = 71,
        LeftBracket = 72,
        RightBracket = 73,
        LeftBrace = 74,
        RightBrace = 75,
        Underscore = 76,
        Sto = 77,
        Arcsine = 78,
        Arccosine = 79,
        Arctangent = 80,
        Equal = 81,
        Lower = 82,
        Greater = 83,
        Colon = 122,
        Semicolon = 123,
        DoubleQuotes = 124,
        Percent = 125,
        LowerA = 126,
        LowerB = 127,
        LowerC = 128,
        LowerD = 129,
        LowerE = 130,
        LowerF = 131,
        LowerG = 132,
        LowerH = 133,
        LowerI = 134,
        LowerJ = 135,
        LowerK = 136,
        LowerL = 137,
        LowerM = 138,
        LowerN = 139,
        LowerO = 140,
        LowerP = 141,
        LowerQ = 142,
        LowerR = 144,
        LowerS = 145,
        LowerT = 146,
        LowerU = 147,
        LowerV = 148,
        LowerW = 150,
        LowerX = 151,
        LowerY = 152,
        LowerZ = 153,
        Space = 154,
        Question = 156,
        Exclamation = 157,
        UpperA = 180,
        UpperB = 181,
        UpperC = 182,
        UpperD = 183,
        UpperE = 184,
        UpperF = 185,
        UpperG = 186,
        UpperH = 187,
        UpperI = 188,
        UpperJ = 189,
        UpperK = 190,
        UpperL = 191,
        UpperM = 192,
        UpperN = 193,
        UpperO = 194,
        UpperP = 195,
        UpperQ = 196,
        UpperR = 198,
        UpperS = 199,
        UpperT = 200,
        UpperU = 201,
        UpperV = 202,
        UpperW = 204,
        UpperX = 205,
        UpperY = 206,
        UpperZ = 207,
    }

    impl Event {
        pub fn is_digit(&self) -> bool {
            matches!(
                self,
                Event::Zero
                    | Event::One
                    | Event::Two
                    | Event::Three
                    | Event::Four
                    | Event::Five
                    | Event::Six
                    | Event::Seven
                    | Event::Eight
                    | Event::Nine
            )
        }

        pub fn to_digit(&self) -> Option<u8> {
            match self {
                Event::Zero => Some(0),
                Event::One => Some(1),
                Event::Two => Some(2),
                Event::Three => Some(3),
                Event::Four => Some(4),
                Event::Five => Some(5),
                Event::Six => Some(6),
                Event::Seven => Some(7),
                Event::Eight => Some(8),
                Event::Nine => Some(9),
                _ => None,
            }
        }
    }

    extern "C" {
        fn eadk_event_get(timeout: &i32) -> Event;
    }

    pub fn event_get(timeout: i32) -> Event {
        unsafe { eadk_event_get(&timeout) }
    }
}

use core::{ffi::c_size_t, panic::PanicInfo};

#[cfg(debug_assertions)]
#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    println!("{}", panic);

    loop {} // FIXME: Do something better. Exit the app maybe?
}
#[cfg(not(debug_assertions))]
#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    println!("{}", panic);

    loop {} // FIXME: Do something better. Exit the app maybe?
}
