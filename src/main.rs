use termicolour::*;

// use termicolour::{colour_print, colour_print_bright, print_styled, Styles};
// use termicolour::Colour8;
// use termicolour::BrightColour;

fn main() {
    test_print();
}

fn test_print() {
    println!("=== Text Formatting ===");
    print_styled("Bold Text",Styles::Bold);
    print_styled("Italic Text",Styles::Italic);
    print_styled("Underlined Text",Styles::Underlined);

    println!("\n=== 8-Bit Text Colours ===");
    colour_print!( "Black Text",Colour8::Colours::Black);
    colour_print!( "Red Text",Colour8::Colours::Red);
    colour_print!( "Gren Text",Colour8::Colours::Green);
    colour_print!( "Yellow Text",Colour8::Colours::Yellow);
    colour_print!( "Blue Text",Colour8::Colours::Blue);
    colour_print!( "Magenta Text",Colour8::Colours::Magenta);
    colour_print!( "Cyan Text",Colour8::Colours::Cyan);
    colour_print!( "White Text",Colour8::Colours::White);
    colour_print!( "Default Text",Colour8::Colours::Default);

    println!("\n=== 8-Bit Background Colours ===");
    colour_print!( "Black Background", Colour8::Colours::Black, true);
    colour_print!( "Red Background", Colour8::Colours::Red, true);
    colour_print!( "Gren Background", Colour8::Colours::Green, true);
    colour_print!( "Yellow Background", Colour8::Colours::Yellow, true);
    colour_print!( "Blue Background", Colour8::Colours::Blue, true);
    colour_print!( "Magenta Background", Colour8::Colours::Magenta, true);
    colour_print!( "Cyan Background", Colour8::Colours::Cyan, true);
    colour_print!( "White Background", Colour8::Colours::White, true);
    colour_print!( "Default Background", Colour8::Colours::Default, true);

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
