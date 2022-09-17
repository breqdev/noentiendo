pub const RETURN: char = '\n';
pub const BACKSPACE: char = '\x08';

// abuse the ASCII "shift out" / "shift in" characters to store our shift key state
pub const LSHIFT: char = '\x0E';
pub const RSHIFT: char = '\x0F';

// Windows key or Command key
pub const LSUPER: char = '\x11';
pub const RSUPER: char = '\x12';
