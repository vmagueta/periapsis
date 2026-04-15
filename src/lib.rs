//! Orbital mechanics engine in Rust.

/// A parsed Two-Line Element set (TLE).
///
/// A TLE describes a satellite's orbit using a fixed-format text record
/// originally designed for 80-column punch cards. It has three lines:
///
/// - Line 0: the satellite's name (plain text).
/// - Line 1: orbital metadata (catalog number, epoch, drag, etc).
/// - Line 2: the six orbital elements (inclination, eccentricity, etc).
///
/// This struct currently only splits the input into the three lines.
/// Field-level parsing will arrive in future versions.

#[derive(Debug)]
pub struct Tle {
    /// Satellite's name (line 0 of the TLE).
    pub name: String,
    /// Raw content of line 1 (orbital metadata, not yet parsed).
    pub line1: String,
    /// Raw content of line 2 (orbital elements, not yet parsed).
    pub line2: String,
}

/// Errors returned when parsing a TLE.
#[derive(Debug)]
pub enum TleParseError {
    /// The input did not contain three non-empty lines.
    InvalidFormat,
}

impl Tle {
    /// Parses a TLE from a string with three lines.
    ///
    /// Leading and trailing whitespace on each line is trimmed, and empty
    /// lines are ignored.
    ///
    /// # Errors
    ///
    /// Returns [`TleParseError::InvalidFormat`] if the input does not
    /// contain exactly three non-empty lines.
    ///
    /// # Examples
    ///
    /// ```
    /// use periapsis::Tle;
    ///
    /// let input = "ISS (ZARYA)
    /// 1 25544U 98067A   24001.48046053  .00007287  00000+0  13569-3 0  9997
    /// 2 25544  51.6413 285.0716 0003999 246.9528 255.0349 15.49855516432089";
    ///
    /// let tle = Tle::parse(input).unwrap();
    /// assert_eq!(tle.name, "ISS (ZARYA)");
    /// ```
    pub fn parse(input: &str) -> Result<Tle, TleParseError> {
        // Split the input into lines, trim each, and drop empty ones.
        let lines: Vec<&str> = input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        // A valid TLE must have exactly three lines after cleanup.
        if lines.len() != 3 {
            return Err(TleParseError::InvalidFormat);
        }

        Ok(Tle {
            name: lines[0].to_string(),
            line1: lines[1].to_string(),
            line2: lines[2].to_string(),
        })
    }
}
