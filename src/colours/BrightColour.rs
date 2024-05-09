// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797#8-16-colors
const BLACK:   (&str,&str) = ("[90m", "[100m");
const RED:     (&str,&str) = ("[91m", "[101m");
const GREEN:   (&str,&str) = ("[92m", "[102m");
const YELLOW:  (&str,&str) = ("[93m", "[103m");
const BLUE:    (&str,&str) = ("[94m", "[104m");
const MAGENTA: (&str,&str) = ("[95m", "[105m");
const CYAN:    (&str,&str) = ("[96m", "[106m");
const WHITE:   (&str,&str) = ("[97m", "[107m");

pub enum Colours{
    Black, 
    Red,    
    Green,  
    Yellow, 
    Blue,   
    Magenta,
    Cyan,
    White
}

impl Colours {
    pub fn getVals(self) -> (String,String) {
        match self {
            Colours::Black      => return (BLACK.0.to_string(), BLACK.1.to_string()),
            Colours::Red        => return (RED.0.to_string(), RED.1.to_string()),
            Colours::Green      => return (GREEN.0.to_string(), GREEN.1.to_string()) ,
            Colours::Yellow     => return (YELLOW.0.to_string(), YELLOW.1.to_string()),
            Colours::Blue       => return (BLUE.0.to_string(), BLUE.1.to_string()),
            Colours::Magenta    => return (MAGENTA.0.to_string(), MAGENTA.1.to_string()),
            Colours::Cyan       => return (CYAN.0.to_string(), CYAN.1.to_string())  ,
            Colours::White      => return (WHITE.0.to_string(), WHITE.1.to_string())
        }
    }
}

