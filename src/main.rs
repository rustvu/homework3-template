//! GPS Track Analysis Tool
//! 
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseFloatError;

mod geo;
// TODO: make use of the geo module

/// Error type for track operations (wraps different errors)
#[derive(Debug)]
enum TrackError {
    IO(io::Error),
    ParseLine(String), // String is the invalid line
    ParseFloat(ParseFloatError),
    LatitudeBounds(f64),  // f64 is the invalid latitude
    LongitudeBounds(f64), // f64 is the invalid longitude
}

impl Display for TrackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackError::IO(err) => write!(f, "IO error: {}", err),
            TrackError::ParseLine(line) => write!(f, "Invalid line: {}", line),
            TrackError::ParseFloat(err) => write!(f, "Invalid float: {}", err),
            TrackError::LatitudeBounds(latitude) => write!(f, "Invalid latitude: {}", latitude),
            TrackError::LongitudeBounds(longitude) => write!(f, "Invalid longitude: {}", longitude),
        }
    }
}

impl From<io::Error> for TrackError {
    fn from(err: io::Error) -> Self {
        TrackError::IO(err)
    }
}

impl From<ParseFloatError> for TrackError {
    fn from(err: ParseFloatError) -> Self {
        TrackError::ParseFloat(err)
    }
}

/// A track point is a coordinate with a time stamp
#[derive(Debug)]
struct TrackPoint {
    time: f64,
    coordinate: Coordinate,
}

impl TrackPoint {
    /// Create a new TrackPoint with the given time and coordinate
    fn new(time: f64, coordinate: Coordinate) -> Self {
        Self { time, coordinate }
    }

    /// Create new TrackPoint from a string (comma separated values)
    fn from_line(line: &str) -> Result<TrackPoint, TrackError> {
        // TODO: implement this function
    }
}

/// Read a track from a CSV file
fn read_track(filename: &str) -> Result<Vec<TrackPoint>, TrackError> {
    // TODO: implement this function
}

/// Calculate the total distance of a track (in meters)
fn total_distance(track: &[TrackPoint]) -> f64 {
    // TODO: implement this function
}

/// Calculate the average speed of a track (in meters per second)
fn average_speed(track: &[TrackPoint]) -> f64 {
    // TODO: implement this function
}

// Calculate maximum speed of a track (in meters per second)
fn max_speed(track: &[TrackPoint]) -> f64 {
    // TODO: implement this function
}

fn main() {
    let track = read_track("gps_track.csv").unwrap();
    let total = total_distance(&track);
    let avg_speed = average_speed(&track);
    let max_speed = max_speed(&track);

    println!("Total distance: {:.1} km", total / 1000.0);
    println!("Average speed: {:.1} km/h", avg_speed * 3.6);
    println!("Maximum speed: {:.1} km/h", max_speed * 3.6);
}

//////////////////////////////////////////////////////////////////////////////
// DO NOT EDIT BELOW THIS LINE
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const REL_TOL: f64 = 1e-9;

    #[test]
    fn test_from_line() {
        let line = "0.0,0.0,0.0";
        assert!(matches!(TrackPoint::from_line(line), Ok(_)));

        let line = "foo"; // Invalid line
        assert!(matches!(
            TrackPoint::from_line(line),
            Err(TrackError::ParseLine(_))
        ));

        let line = "0.0,0.0"; // Invalid line
        assert!(matches!(
            TrackPoint::from_line(line),
            Err(TrackError::ParseLine(_))
        ));

        let line = "0.0,0.0,0.0,0.0"; // Invalid line
        assert!(matches!(
            TrackPoint::from_line(line),
            Err(TrackError::ParseLine(_))
        ));

        let line = "0.0,foo,0.0"; // Invalid float
        assert!(matches!(
            TrackPoint::from_line(line),
            Err(TrackError::ParseFloat(_))
        ));

        let line = "0.0,91.0,0.0"; // Invalid latitude
        assert!(matches!(
            TrackPoint::from_line(line),
            Err(TrackError::LatitudeBounds(_))
        ));

        let line = "0.0,0.0,181.0"; // Invalid longitude
        assert!(matches!(
            TrackPoint::from_line(line),
            Err(TrackError::LongitudeBounds(_))
        ));
    }

    #[test]
    fn test_read_track() {
        let track = read_track("gps_track.csv").unwrap();
        assert_eq!(track.len(), 984);

        let track = read_track("foo"); // Invalid file
        assert!(matches!(track, Err(TrackError::IO(_))));
    }

    #[test]
    fn test_total_distance() {
        let track = read_track("gps_track.csv").unwrap();
        let total = total_distance(&track);
        assert_relative_eq!(total, 30744.471123153537, max_relative = REL_TOL);
    }

    #[test]
    fn test_average_speed() {
        let track = read_track("gps_track.csv").unwrap();
        let avg_speed = average_speed(&track);
        assert_relative_eq!(avg_speed, 23.308765956465567, max_relative = REL_TOL);
    }

    #[test]
    fn test_max_speed() {
        let track = read_track("gps_track.csv").unwrap();
        let max_speed = max_speed(&track);
        assert_relative_eq!(max_speed, 37.54823020368235, max_relative = REL_TOL);
    }
}
