pub const RETURN: char = '\n';
pub const BACKSPACE: char = '\x08';

// abuse the ASCII "shift out" / "shift in" characters to store our shift key state
pub const LSHIFT: char = '\x0E';
pub const RSHIFT: char = '\x0F';

// Windows key or Command key
pub const LSUPER: char = '\x05';
pub const RSUPER: char = '\x06';

pub const LEFT_ARROW: char = '\x11';
pub const RIGHT_ARROW: char = '\x12';
pub const UP_ARROW: char = '\x13';
pub const DOWN_ARROW: char = '\x14';

pub const HOME: char = '\x15';
pub const CONTROL: char = '\x16';

// commodore stuff
pub const COMMODORE: char = '\x17';
pub const RUN_STOP: char = '\x18';

// Function keys
pub const F1: char = (0x81 as u8) as char;
pub const F2: char = (0x82 as u8) as char;
pub const F3: char = (0x83 as u8) as char;
pub const F4: char = (0x84 as u8) as char;
pub const F5: char = (0x85 as u8) as char;
pub const F6: char = (0x86 as u8) as char;
pub const F7: char = (0x87 as u8) as char;
pub const F8: char = (0x88 as u8) as char;
