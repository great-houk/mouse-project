# Mouse CLI

Mouse CLI is a command line tool to interface with the mouse and it's custom command protocol. It has various commands, like bat to check battery voltage, or lorem-ipsum to check that both devices are working properly. 

# Basic Help Page

USAGE:
    mouse-cli.exe [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help                       Print help information
    -p, --pid <PID>                  [default: 10205]
    -u, --usage-page <USAGE_PAGE>    [default: 65280]
    -v, --vid <VID>                  [default: 5824]
    -V, --version                    Print version information

SUBCOMMANDS:
    bat            Reads the battery voltage of the mouse, doesn't work when the mouse is being
                       charged. Proper voltage should be between 3.0v and 4.2v
    cpi            Sets the keys to press when the CPI button is pressed
    dpi            Reports/Sets the DPI of the Mouse
    fun            Unsaveable options for fun :)
    help           Print this message or the help of the given subcommand(s)
    lift           Sets the keys to press when the mouse is quickly lifted and set back down
    lorem-ipsum    Get's the mouse to say back a part of the famous Lorem Ipsum. Used to check
                       that every device/library is working as intended
    poll-rate      Sets the number of ms between polls. So 1 means 1000hz polling rate, 2 means
                       500, etc
    save           Saves the mouse's current settings (DPI, Keybinds) in to flash memory. Should
                       be used sparingly, since there is a limited number of times flash can be
                       written to
    sayhi          Says Hi 😎
    
   
