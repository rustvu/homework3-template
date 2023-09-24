//! Geospatial module.

/// Point represents a point on the surface of a sphere.
/// Latitude and longitude coordinates are represented in degrees as floats.
/// The latitude is negative if it is south of the equator
/// (a positive number implies north), and the longitude is negative
/// if it is west of the prime meridian (a positive number implies east).

// TODO: make sure this struct is visible outside this module
// TODO: derive the neccesary traits
struct Coordinate {
    latitude: f64,
    longitude: f64,
}

impl Coordinate {
    const EARTH_RADIUS: f64 = 6_371_000.0; // meters

    /// Create a new Point with the given latitude and longitude.
    ///
    /// # Panics
    ///
    /// Panics if the latitude is not between -90 and 90 degrees,
    /// or if the longitude is not between -180 and 180 degrees.
    pub fn new(latitude: f64, longitude: f64) -> Self {
        // TODO: implement this function
    }

    /// Calculate the distance in meters on the surface of Earth between this
    /// point and another.
    /// Based on: <https://en.wikipedia.org/wiki/Haversine_formula>
    pub fn distance(&self, other: &Self) -> f64 {
        // TODO: implement this function
    }
}

//////////////////////////////////////////////////////////////////////////////
// DO NOT EDIT BELOW THIS LINE
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const REL_TOL: f64 = 1e-9;

    #[test]
    fn test_new() {
        let p = Coordinate::new(0.0, 0.0);
        assert_relative_eq!(p.latitude, 0.0, max_relative = REL_TOL);
        assert_relative_eq!(p.longitude, 0.0, max_relative = REL_TOL);

        let p = Coordinate::new(90.0, 180.0);
        assert_relative_eq!(p.latitude, 90.0, max_relative = REL_TOL);
        assert_relative_eq!(p.longitude, 180.0, max_relative = REL_TOL);

        let p = Coordinate::new(-90.0, -180.0);
        assert_relative_eq!(p.latitude, -90.0, max_relative = REL_TOL);
        assert_relative_eq!(p.longitude, -180.0, max_relative = REL_TOL);
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_latitude() {
        Coordinate::new(91.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_longitude() {
        Coordinate::new(0.0, 181.0);
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_latitude_negative() {
        Coordinate::new(-91.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_longitude_negative() {
        Coordinate::new(0.0, -181.0);
    }

    #[test]
    fn test_derived_traits() {
        let p1 = Coordinate::new(13.0, -7.0);

        let p2 = p1.clone(); // Clone
        let p3 = p1; // Copy
        let _s = format!("{:?}, {:?}, {:?}", p1, p2, p3); // Debug
    }

    #[test]
    fn test_distance_point() {
        let nashville = Coordinate::new(36.174465, -86.767960);
        let london = Coordinate::new(51.507351, -0.127758);
        let rio = Coordinate::new(-22.906847, -43.172896);
        let sydney = Coordinate::new(-33.865143, 151.209900);

        assert_relative_eq!(nashville.distance(&nashville), 0.0, max_relative = REL_TOL);
        assert_relative_eq!(nashville.distance(&london), 6734609.838384662, max_relative = REL_TOL);
        assert_relative_eq!(nashville.distance(&rio), 8007694.96445111, max_relative = REL_TOL);
        assert_relative_eq!(nashville.distance(&sydney), 14809198.514313262, max_relative = REL_TOL);
    }
}
