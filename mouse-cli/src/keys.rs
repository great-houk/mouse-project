use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ParseError {
    ToManyKeys(usize),
    InvalidToken(String),
}

pub fn map_keys(mods: u8, keys: [u8; 6]) -> String {
    let (modmap, mut keymap) = get_hashmaps();
    keymap.insert("", 0x00);
    let mut ret_str = "".to_string();
    // Get mods first
    for i in 0..8 {
        if let Some(val) = find_key_for_value(&modmap, mods & (1 << i)) {
            ret_str = ret_str + val + "+";
        }
    }
    // Get keys
    for key in keys {
        if let Some(val) = find_key_for_value(&keymap, key) {
            if val != "" {
                ret_str = ret_str + val + "+";
            }
        } else {
            ret_str += &format!("0x{key:02X}+");
        }
    }
    // Remove last +
    if ret_str.len() > 0 {
        ret_str.truncate(ret_str.len() - 1);
    } else {
        ret_str = "None".to_string();
    }
    ret_str
}

fn find_key_for_value<'a>(map: &'a HashMap<&'static str, u8>, value: u8) -> Option<&'static str> {
    map.iter()
        .find_map(|(key, &val)| if val == value { Some(*key) } else { None })
}

pub fn map_string(input: String) -> Result<(u8, [u8; 6]), ParseError> {
    let tokens = input.split('+');
    let (mod_map, mut key_map) = get_hashmaps();
    key_map.insert("NONE", 0x00);
    let mut keys = Vec::with_capacity(6);
    let mut mods = 0;
    // Parse all tokens
    for token in tokens {
        let token: &str = &token.to_uppercase();
        if let Some(key) = key_map.get(token) {
            keys.push(*key);
        } else if let Some(mod_val) = mod_map.get(token) {
            mods |= mod_val;
        } else if token.as_bytes().len() == 4 && token.as_bytes()[..2] == ['0' as u8, 'X' as u8] {
            let num = String::from_utf8(token.as_bytes()[2..4].to_vec()).unwrap();
            match u8::from_str_radix(&num, 16) {
                Ok(num) => keys.push(num),
                Err(_) => return Err(ParseError::InvalidToken(token.to_string())),
            }
        } else {
            return Err(ParseError::InvalidToken(token.to_string()));
        }
    }
    // Make sure keys isn't too long
    if keys.len() > 6 {
        return Err(ParseError::ToManyKeys(keys.len()));
    }
    // Return parsed keys
    let mut key_codes = [0; 6];
    key_codes[..keys.len()].copy_from_slice(&keys);
    Ok((mods, key_codes))
}

fn get_hashmaps() -> (HashMap<&'static str, u8>, HashMap<&'static str, u8>) {
    let mods = HashMap::from([
        ("CTRL", 0x01),
        ("SHIFT", 0x02),
        ("ALT", 0x04),
        ("META", 0x08),
        ("LCTRL", 0x01),
        ("LSHIFT", 0x02),
        ("LALT", 0x04),
        ("LMETA", 0x08),
        ("RCTRL", 0x10),
        ("RSHIFT", 0x20),
        ("RALT", 0x40),
        ("RMETA", 0x80),
    ]);

    /*
     * Scan codes - last N slots in the HID report (usually 6).
     * 0x00 if no key pressed.
     *
     * If more than N keys are pressed, the HID reports
     * KEY_ERR_OVF in all slots to indicate this condition.
     */

    let chars = HashMap::from([
        ("A", 0x04),          // Keyboard a and A
        ("B", 0x05),          // Keyboard b and B
        ("C", 0x06),          // Keyboard c and C
        ("D", 0x07),          // Keyboard d and D
        ("E", 0x08),          // Keyboard e and E
        ("F", 0x09),          // Keyboard f and F
        ("G", 0x0a),          // Keyboard g and G
        ("H", 0x0b),          // Keyboard h and H
        ("I", 0x0c),          // Keyboard i and I
        ("J", 0x0d),          // Keyboard j and J
        ("K", 0x0e),          // Keyboard k and K
        ("L", 0x0f),          // Keyboard l and L
        ("M", 0x10),          // Keyboard m and M
        ("N", 0x11),          // Keyboard n and N
        ("O", 0x12),          // Keyboard o and O
        ("P", 0x13),          // Keyboard p and P
        ("Q", 0x14),          // Keyboard q and Q
        ("R", 0x15),          // Keyboard r and R
        ("S", 0x16),          // Keyboard s and S
        ("T", 0x17),          // Keyboard t and T
        ("U", 0x18),          // Keyboard u and U
        ("V", 0x19),          // Keyboard v and V
        ("W", 0x1a),          // Keyboard w and W
        ("X", 0x1b),          // Keyboard x and X
        ("Y", 0x1c),          // Keyboard y and Y
        ("Z", 0x1d),          // Keyboard z and Z
        ("1", 0x1e),          // Keyboard 1 and !
        ("2", 0x1f),          // Keyboard 2 and @
        ("3", 0x20),          // Keyboard 3 and #
        ("4", 0x21),          // Keyboard 4 and $
        ("5", 0x22),          // Keyboard 5 and %
        ("6", 0x23),          // Keyboard 6 and ^
        ("7", 0x24),          // Keyboard 7 and &
        ("8", 0x25),          // Keyboard 8 and *
        ("9", 0x26),          // Keyboard 9 and (
        ("0", 0x27),          // Keyboard 0 and )
        ("ENTER", 0x28),      // Keyboard Return (ENTER)
        ("ESC", 0x29),        // Keyboard ESCAPE
        ("BACKSPACE", 0x2a),  // Keyboard DELETE (Backspace)
        ("TAB", 0x2b),        // Keyboard Tab
        ("SPACE", 0x2c),      // Keyboard Spacebar
        ("MINUS", 0x2d),      // Keyboard - and _
        ("EQUAL", 0x2e),      // Keyboard = and +
        ("LEFTBRACE", 0x2f),  // Keyboard [ and {
        ("RIGHTBRACE", 0x30), // Keyboard ] and }
        ("BACKSLASH", 0x31),  // Keyboard \ and |
        ("HASHTILDE", 0x32),  // Keyboard Non-US # and ~
        ("SEMICOLON", 0x33),  // Keyboard ; and :
        ("APOSTROPHE", 0x34), // Keyboard ' and "
        ("GRAVE", 0x35),      // Keyboard ` and ~
        ("COMMA", 0x36),      // Keyboard , and <
        ("DOT", 0x37),        // Keyboard . and >
        ("SLASH", 0x38),      // Keyboard / and ?
        ("CAPSLOCK", 0x39),   // Keyboard Caps Lock
        ("F1", 0x3a),         // Keyboard F1
        ("F2", 0x3b),         // Keyboard F2
        ("F3", 0x3c),         // Keyboard F3
        ("F4", 0x3d),         // Keyboard F4
        ("F5", 0x3e),         // Keyboard F5
        ("F6", 0x3f),         // Keyboard F6
        ("F7", 0x40),         // Keyboard F7
        ("F8", 0x41),         // Keyboard F8
        ("F9", 0x42),         // Keyboard F9
        ("F10", 0x43),        // Keyboard F10
        ("F11", 0x44),        // Keyboard F11
        ("F12", 0x45),        // Keyboard F12
        ("SYSRQ", 0x46),      // Keyboard Print Screen
        ("SCROLLLOCK", 0x47), // Keyboard Scroll Lock
        ("PAUSE", 0x48),      // Keyboard Pause
        ("INSERT", 0x49),     // Keyboard Insert
        ("HOME", 0x4a),       // Keyboard Home
        ("PAGEUP", 0x4b),     // Keyboard Page Up
        ("DELETE", 0x4c),     // Keyboard Delete Forward
        ("END", 0x4d),        // Keyboard End
        ("PAGEDOWN", 0x4e),   // Keyboard Page Down
        ("RIGHT", 0x4f),      // Keyboard Right Arrow
        ("LEFT", 0x50),       // Keyboard Left Arrow
        ("DOWN", 0x51),       // Keyboard Down Arrow
        ("UP", 0x52),         // Keyboard Up Arrow
        ("NUMLOCK", 0x53),    // Keyboard Num Lock and Clear
        ("KPSLASH", 0x54),    // Keypad /
        ("KPASTERISK", 0x55), // Keypad *
        ("KPMINUS", 0x56),    // Keypad -
        ("KPPLUS", 0x57),     // Keypad +
        ("KPENTER", 0x58),    // Keypad ENTER
        ("KP1", 0x59),        // Keypad 1 and End
        ("KP2", 0x5a),        // Keypad 2 and Down Arrow
        ("KP3", 0x5b),        // Keypad 3 and PageDn
        ("KP4", 0x5c),        // Keypad 4 and Left Arrow
        ("KP5", 0x5d),        // Keypad 5
        ("KP6", 0x5e),        // Keypad 6 and Right Arrow
        ("KP7", 0x5f),        // Keypad 7 and Home
        ("KP8", 0x60),        // Keypad 8 and Up Arrow
        ("KP9", 0x61),        // Keypad 9 and Page Up
        ("KP0", 0x62),        // Keypad 0 and Insert
        ("KPDOT", 0x63),      // Keypad . and Delete
        ("102ND", 0x64),      // Keyboard Non-US \ and |
        ("COMPOSE", 0x65),    // Keyboard Application
        ("POWER", 0x66),      // Keyboard Power
        ("KPEQUAL", 0x67),    // Keypad =
        ("F13", 0x68),        // Keyboard F13
        ("F14", 0x69),        // Keyboard F14
        ("F15", 0x6a),        // Keyboard F15
        ("F16", 0x6b),        // Keyboard F16
        ("F17", 0x6c),        // Keyboard F17
        ("F18", 0x6d),        // Keyboard F18
        ("F19", 0x6e),        // Keyboard F19
        ("F20", 0x6f),        // Keyboard F20
        ("F21", 0x70),        // Keyboard F21
        ("F22", 0x71),        // Keyboard F22
        ("F23", 0x72),        // Keyboard F23
        ("F24", 0x73),        // Keyboard F24
        ("OPEN", 0x74),       // Keyboard Execute
        ("HELP", 0x75),       // Keyboard Help
        ("PROPS", 0x76),      // Keyboard Menu
        ("FRONT", 0x77),      // Keyboard Select
        ("STOP", 0x78),       // Keyboard Stop
        ("AGAIN", 0x79),      // Keyboard Again
        ("UNDO", 0x7a),       // Keyboard Undo
        ("CUT", 0x7b),        // Keyboard Cut
        ("COPY", 0x7c),       // Keyboard Copy
        ("PASTE", 0x7d),      // Keyboard Paste
        ("FIND", 0x7e),       // Keyboard Find
        ("MUTE", 0x7f),       // Keyboard Mute
        ("VOLUMEUP", 0x80),   // Keyboard Volume Up
        ("VOLUMEDOWN", 0x81), // Keyboard Volume Down
        // 0x82  Keyboard Locking Caps Lock
        // 0x83  Keyboard Locking Num Lock
        // 0x84  Keyboard Locking Scroll Lock
        ("KPCOMMA", 0x85), // Keypad Comma
        // 0x86  Keypad Equal Sign
        ("RO", 0x87),               // Keyboard International1
        ("KATAKANAHIRAGANA", 0x88), // Keyboard International2
        ("YEN", 0x89),              // Keyboard International3
        ("HENKAN", 0x8a),           // Keyboard International4
        ("MUHENKAN", 0x8b),         // Keyboard International5
        ("KPJPCOMMA", 0x8c),        // Keyboard International6
        // 0x8d  Keyboard International7
        // 0x8e  Keyboard International8
        // 0x8f  Keyboard International9
        ("HANGEUL", 0x90),        // Keyboard LANG1
        ("HANJA", 0x91),          // Keyboard LANG2
        ("KATAKANA", 0x92),       // Keyboard LANG3
        ("HIRAGANA", 0x93),       // Keyboard LANG4
        ("ZENKAKUHANKAKU", 0x94), // Keyboard LANG5
        // 0x95  Keyboard LANG6
        // 0x96  Keyboard LANG7
        // 0x97  Keyboard LANG8
        // 0x98  Keyboard LANG9
        // 0x99  Keyboard Alternate Erase
        // 0x9a  Keyboard SysReq/Attention
        // 0x9b  Keyboard Cancel
        // 0x9c  Keyboard Clear
        // 0x9d  Keyboard Prior
        // 0x9e  Keyboard Return
        // 0x9f  Keyboard Separator
        // 0xa0  Keyboard Out
        // 0xa1  Keyboard Oper
        // 0xa2  Keyboard Clear/Again
        // 0xa3  Keyboard CrSel/Props
        // 0xa4  Keyboard ExSel

        // 0xb0  Keypad 00
        // 0xb1  Keypad 000
        // 0xb2  Thousands Separator
        // 0xb3  Decimal Separator
        // 0xb4  Currency Unit
        // 0xb5  Currency Sub-unit
        ("KPLEFTPAREN", 0xb6),  // Keypad (
        ("KPRIGHTPAREN", 0xb7), // Keypad )
    ]);
    // Unused
    // 0xb8  Keypad {
    // 0xb9  Keypad }
    // 0xba  Keypad Tab
    // 0xbb  Keypad Backspace
    // 0xbc  Keypad A
    // 0xbd  Keypad B
    // 0xbe  Keypad C
    // 0xbf  Keypad D
    // 0xc0  Keypad E
    // 0xc1  Keypad F
    // 0xc2  Keypad XOR
    // 0xc3  Keypad ^
    // 0xc4  Keypad %
    // 0xc5  Keypad <
    // 0xc6  Keypad >
    // 0xc7  Keypad &
    // 0xc8  Keypad &&
    // 0xc9  Keypad |
    // 0xca  Keypad ||
    // 0xcb  Keypad :
    // 0xcc  Keypad #
    // 0xcd  Keypad Space
    // 0xce  Keypad @
    // 0xcf  Keypad !
    // 0xd0  Keypad Memory Store
    // 0xd1  Keypad Memory Recall
    // 0xd2  Keypad Memory Clear
    // 0xd3  Keypad Memory Add
    // 0xd4  Keypad Memory Subtract
    // 0xd5  Keypad Memory Multiply
    // 0xd6  Keypad Memory Divide
    // 0xd7  Keypad +/-
    // 0xd8  Keypad Clear
    // 0xd9  Keypad Clear Entry
    // 0xda  Keypad Binary
    // 0xdb  Keypad Octal
    // 0xdc  Keypad Decimal
    // 0xdd  Keypad Hexadecimal
    // ("MEDIA_PLAYPAUSE", 0xe8),
    // ("MEDIA_STOPCD", 0xe9),
    // ("MEDIA_PREVIOUSSONG", 0xea),
    // ("MEDIA_NEXTSONG", 0xeb),
    // ("MEDIA_EJECTCD", 0xec),
    // ("MEDIA_VOLUMEUP", 0xed),
    // ("MEDIA_VOLUMEDOWN", 0xee),
    // ("MEDIA_MUTE", 0xef),
    // ("MEDIA_WWW", 0xf0),
    // ("MEDIA_BACK", 0xf1),
    // ("MEDIA_FORWARD", 0xf2),
    // ("MEDIA_STOP", 0xf3),
    // ("MEDIA_FIND", 0xf4),
    // ("MEDIA_SCROLLUP", 0xf5),
    // ("MEDIA_SCROLLDOWN", 0xf6),
    // ("MEDIA_EDIT", 0xf7),
    // ("MEDIA_SLEEP", 0xf8),
    // ("MEDIA_COFFEE", 0xf9),
    // ("MEDIA_REFRESH", 0xfa),
    // ("MEDIA_CALC", 0xfb),
    (mods, chars)
}
