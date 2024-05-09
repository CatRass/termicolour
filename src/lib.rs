use std::io::{self, Write};

pub mod colours;

const ESC: u8       = b'\x1B';
const END: &str     = "[0m";
const BOLD: &str    = "[1m";
const ITAL: &str    = "[3m";
const UNDR: &str    = "[4m";
const BLNK: &str    = "[5m";

pub enum Styles {
    Bold,
    Italic,
    Underlined,
    Blinking
}

fn print_text(text: &[u8]) {
    io::stdout().write(text).unwrap();
    io::stdout().flush().unwrap();
}

pub fn print_styled(text: &str, style: Styles) -> bool {
    let mut text_bytes = vec!(ESC);

    match style {
        Styles::Bold        => text_bytes.append(&mut BOLD.as_bytes().to_owned()),
        Styles::Italic      => text_bytes.append(&mut ITAL.as_bytes().to_owned()),
        Styles::Underlined  => text_bytes.append(&mut UNDR.as_bytes().to_owned()),
        Styles::Blinking    => text_bytes.append(&mut BLNK.as_bytes().to_owned()),  // TODO: Not finished, refer to https://stackoverflow.com/q/69710553/12884111
    }

    text_bytes.append(&mut ((&text.to_owned()).as_bytes().to_owned()));
    text_bytes.push(ESC);
    text_bytes.append(&mut END.as_bytes().to_owned());
    text_bytes.push(b'\n');

    print_text(&text_bytes);

    return true;
}

pub fn print_coloured_8bit(text: &str, colour: colours::Colour16::Colours, background:bool) -> bool {
    let colour_vals = colour.getVals();
    let mut text_bytes = vec!(ESC);
    
    if background {
        text_bytes.append(&mut ((colour_vals.1.to_owned()).as_bytes().to_owned()));
    } else {
        text_bytes.append(&mut ((colour_vals.0.to_owned()).as_bytes().to_owned()));
    }    
    
    text_bytes.append(&mut ((&text.to_owned()).as_bytes().to_owned()));
    text_bytes.push(ESC);
    text_bytes.append(&mut END.as_bytes().to_owned());
    text_bytes.push(b'\n');

    print_text(&text_bytes);
    return true;
}    

pub fn print_coloured_bright(text: &str, colour: colours::BrightColour::Colours, background:bool) -> bool {
    let colour_vals = colour.getVals();
    let mut text_bytes = vec!(ESC);
    
    if background {
        text_bytes.append(&mut ((colour_vals.1.to_owned()).as_bytes().to_owned()));
    } else {
        text_bytes.append(&mut ((colour_vals.0.to_owned()).as_bytes().to_owned()));
    }    
    
    text_bytes.append(&mut ((&text.to_owned()).as_bytes().to_owned()));
    text_bytes.push(ESC);
    text_bytes.append(&mut END.as_bytes().to_owned());
    text_bytes.push(b'\n');

    print_text(&text_bytes);
    return true;
}

#[macro_export]
macro_rules! colour_print {
    ($text: expr, $colour:expr, $background: expr) => {
        $crate::print_coloured_8bit($text, $colour, $background)
    };
    ($text: expr, $colour:expr) => {
        $crate::print_coloured_8bit($text, $colour, false)
    };
}

#[macro_export]
macro_rules! colour_print_bright {
    ($text: expr, $colour:expr, $background: expr) => {
        $crate::print_coloured_bright($text, $colour, $background)
    };
    ($text: expr, $colour:expr) => {
        $crate::print_coloured_bright($text, $colour, false)
    };
}
