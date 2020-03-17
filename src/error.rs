use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    // map loading (deserializing)
    InvalidMapSize,
    OverlappingStartingCells,

    // character creation (before game starts)
    TeamFull,
    InvalidCharacterName,
    InvalidCharacterClass,
    InvalidStartingCell,

    // client-error (wrong value is API request)
    InvalidSkill,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::InvalidMapSize => {
                f.write_str("Map's width*height does not equal its number of cells")
            }
            Error::OverlappingStartingCells => {
                f.write_str("Starting cells cannot have duplicates (different teams or not)")
            }

            Error::TeamFull => f.write_str("Team is already full"),
            Error::InvalidStartingCell => {
                f.write_str("Initial position is either not a starting cell, or is already taken")
            }
            Error::InvalidCharacterName => {
                f.write_str("Character's name cannot be made only of whitespaces")
            }
            Error::InvalidCharacterClass => {
                f.write_str("Class id does not correspond to an existing class")
            }

            Error::InvalidSkill => f.write_str("Team is already full"),
        }
    }
}

impl std::error::Error for Error {}
