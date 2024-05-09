use termicolour::{colour_print, colour_print_bright, print_styled, Styles};

use termicolour::colours::Colour16;
use termicolour::colours::BrightColour;

fn main() {
    test_print();
}

fn test_print() {
    println!("=== Text Formatting ===");
    print_styled("Bold Text",Styles::Bold);
    print_styled("Italic Text",Styles::Italic);
    print_styled("Underlined Text",Styles::Underlined);

    println!("\n=== 8-Bit Text Colours ===");
    colour_print!( "Black Text",Colour16::Colours::Black);
    colour_print!( "Red Text",Colour16::Colours::Red);
    colour_print!( "Gren Text",Colour16::Colours::Green);
    colour_print!( "Yellow Text",Colour16::Colours::Yellow);
    colour_print!( "Blue Text",Colour16::Colours::Blue);
    colour_print!( "Magenta Text",Colour16::Colours::Magenta);
    colour_print!( "Cyan Text",Colour16::Colours::Cyan);
    colour_print!( "White Text",Colour16::Colours::White);
    colour_print!( "Default Text",Colour16::Colours::Default);

    println!("\n=== 8-Bit Background Colours ===");
    colour_print!( "Black Background", Colour16::Colours::Black, true);
    colour_print!( "Red Background", Colour16::Colours::Red, true);
    colour_print!( "Gren Background", Colour16::Colours::Green, true);
    colour_print!( "Yellow Background", Colour16::Colours::Yellow, true);
    colour_print!( "Blue Background", Colour16::Colours::Blue, true);
    colour_print!( "Magenta Background", Colour16::Colours::Magenta, true);
    colour_print!( "Cyan Background", Colour16::Colours::Cyan, true);
    colour_print!( "White Background", Colour16::Colours::White, true);
    colour_print!( "Default Background", Colour16::Colours::Default, true);

    println!("\n=== Bright Text Colours ===");
    colour_print_bright!( "Black Text",BrightColour::Colours::Black);
    colour_print_bright!( "Red Text",BrightColour::Colours::Red);
    colour_print_bright!( "Gren Text",BrightColour::Colours::Green);
    colour_print_bright!( "Yellow Text",BrightColour::Colours::Yellow);
    colour_print_bright!( "Blue Text",BrightColour::Colours::Blue);
    colour_print_bright!( "Magenta Text",BrightColour::Colours::Magenta);
    colour_print_bright!( "Cyan Text",BrightColour::Colours::Cyan);
    colour_print_bright!( "White Text",BrightColour::Colours::White);

    println!("\n=== Bright Background Colours ===");
    colour_print_bright!( "Black Background",BrightColour::Colours::Black, true);
    colour_print_bright!( "Red Background",BrightColour::Colours::Red, true);
    colour_print_bright!( "Gren Background",BrightColour::Colours::Green, true);
    colour_print_bright!( "Yellow Background",BrightColour::Colours::Yellow, true);
    colour_print_bright!( "Blue Background",BrightColour::Colours::Blue, true);
    colour_print_bright!( "Magenta Background",BrightColour::Colours::Magenta, true);
    colour_print_bright!( "Cyan Background",BrightColour::Colours::Cyan, true);
    colour_print_bright!( "White Background",BrightColour::Colours::White, true);
}
