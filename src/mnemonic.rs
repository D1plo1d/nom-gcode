use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mnemonic {
    /// Preparatory commands, often telling the controller what kind of motion
    /// or offset is desired.
    General,
    /// Auxilliary commands.
    Miscellaneous,
    /// Used to give the current program a unique "name".
    ProgramNumber,
    /// Tool selection.
    ToolChange,
    /// O-Code: http://linuxcnc.org/docs/html/gcode/o-code.html
    Subroutine,
}

pub static G: Mnemonic = Mnemonic::General;
pub static M: Mnemonic = Mnemonic::Miscellaneous;
pub static P: Mnemonic = Mnemonic::ProgramNumber;
pub static T: Mnemonic = Mnemonic::ToolChange;
pub static O: Mnemonic = Mnemonic::Subroutine;

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Mnemonic::*;

        let mnemonic_char = match self {
            General => 'G',
            Miscellaneous => 'M',
            ProgramNumber =>  'P',
            ToolChange => 'T',
            Subroutine =>  'O',
        };

        write!(f, "{}", mnemonic_char)
    }
}
