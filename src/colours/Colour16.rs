// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797#8-16-colors
const BLACK:   (&str,&str) = ("[30m", "[40m");
const RED:     (&str,&str) = ("[31m", "[41m");
const GREEN:   (&str,&str) = ("[32m", "[42m");
const YELLOW:  (&str,&str) = ("[33m", "[43m");
const BLUE:    (&str,&str) = ("[34m", "[44m");
const MAGENTA: (&str,&str) = ("[35m", "[45m");
const CYAN:    (&str,&str) = ("[36m", "[46m");
const WHITE:   (&str,&str) = ("[37m", "[47m");
const DEFAULT: (&str,&str) = ("[39m", "[49m");

pub enum Colours{
    Black, 
    Red,    
    Green,  
    Yellow, 
    Blue,   
    Magenta,
    Cyan,
    White, 
    Default
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
            Colours::White      => return (WHITE.0.to_string(), WHITE.1.to_string()) ,
            Colours::Default    => return (DEFAULT.0.to_string(), DEFAULT.1.to_string())
        }
    }
}

