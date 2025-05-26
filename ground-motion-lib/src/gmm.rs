//! Ground Motion Prediction Equation (GMPE) definitions and traits.
//!
//! This module provides fundamental data structures and traits for representing
//! seismic input points, earthquake parameters, and ground motion model outputs.
//! It also defines the core trait for implementing specific GMPE models.

use serde::{Deserialize, Serialize};

/// Input point definition for which GMPE will be calculated.
#[derive(Debug, Serialize, Deserialize)]
pub struct Vs30Point {
    /// Longitude in decimal degrees. Example: `142.23567`
    pub lon: f64,
    /// Latitude in decimal degrees. Example: `50.35927`
    pub lat: f64,
    /// Average shear-wave velocity (Vs, in m/s) in the top 30 meters of soil.
    pub vs30: u64,
    /// Depth (in meters) to the subsurface layer where Vs reaches 1400 m/s at the site.
    #[serde(default)]
    pub dl: Option<u64>,
    /// Binary variable (0 or 1) indicating the site's position relative to the volcanic front
    /// (specific to Japan).
    #[serde(default)]
    pub xvf: Option<u8>,
}

/// Magnitude type used in GMPE calculations.
#[derive(Debug)]
pub enum Magnitude {
    /// Moment magnitude (Mw)
    Mw,
    /// Local magnitude (Ml)
    Ml,
}

/// Represents an earthquake event with its source parameters.
#[derive(Debug)]
pub struct Earthquake {
    /// Longitude in decimal degrees.
    pub lon: f64,
    /// Latitude in decimal degrees.
    pub lat: f64,
    /// Earthquake focal depth in kilometers.
    pub depth: f64,
    /// Magnitude value.
    pub magnitude: f64,
    /// Type of magnitude scale (Mw, Ml, etc.)
    pub magnitude_kind: Magnitude,
}

/// Available GMPE output types.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GmpePointKind {
    /// Peak Ground Acceleration, expressed as a percentage of gravity (%g)
    Pga,
    /// Peak Spectral Acceleration, expressed as a percentage of gravity (%g)
    Psa,
    /// Peak Ground Velocity, expressed in cm/s
    Pgv,
}

/// Struct representing a point with a computed GMPE value.
#[derive(Debug, Serialize, Deserialize)]
pub struct GmpePoint {
    /// Longitude in decimal degrees.
    pub lon: f64,
    /// Latitude in decimal degrees.
    pub lat: f64,
    /// Computed ground motion value.
    pub value: f64,
    /// Type of GMPE output value.
    pub kind: GmpePointKind,
}

/// Trait representing a Ground Motion Prediction Equation (GMPE).
///
/// Implementors of this trait can compute ground motion values at a site
/// for a given earthquake scenario.
pub trait GroundMotionModeling {
    /// Compute ground motion value for a given input point and earthquake.
    ///
    /// # Arguments
    ///
    /// * `point` - Input site point parameters.
    /// * `eq` - Earthquake source parameters.
    ///
    /// # Returns
    ///
    /// A `GmpePoint` containing the computed value and its location.
    fn calc_from_point(&self, point: &Vs30Point, eq: &Earthquake) -> GmpePoint;
}

impl Vs30Point {
    /// Create a new Vs30Point instance.
    ///
    /// # Arguments
    ///
    /// * `lon` - Longitude in decimal degrees.
    /// * `lat` - Latitude in decimal degrees.
    /// * `vs30` - Average Vs in the top 30 meters.
    /// * `dl` - Depth to Vs=1400 m/s layer (optional).
    /// * `xvf` - Binary volcanic front position indicator (optional).
    ///
    /// # Example
    ///
    /// ```
    /// use ground_motion_lib::gmm::Vs30Point;
    /// let vs30_point = Vs30Point::new(142.523, 52.913, 300, Some(250), Some(1));
    /// println!("Point is {vs30_point:?}");
    /// ```
    pub fn new(lon: f64, lat: f64, vs30: u64, dl: Option<u64>, xvf: Option<u8>) -> Self {
        Self {
            lon,
            lat,
            vs30,
            dl,
            xvf,
        }
    }

    /// Calculate ground motion value for this point and given earthquake, using a GMPE.
    ///
    /// # Arguments
    ///
    /// * `gmpe` - Reference to a type implementing `GroundMotionModeling`.
    /// * `eq` - Earthquake source parameters.
    ///
    /// # Returns
    ///
    /// A `GmpePoint` with the computed value.
    pub fn get_gm<T: GroundMotionModeling>(&self, gmpe: &T, eq: &Earthquake) -> GmpePoint {
        gmpe.calc_from_point(self, eq)
    }
}

impl Earthquake {
    /// Create a new Earthquake instance.
    ///
    /// # Arguments
    ///
    /// * `lon` - Longitude in decimal degrees.
    /// * `lat` - Latitude in decimal degrees.
    /// * `depth` - Focal depth in kilometers.
    /// * `magnitude` - Magnitude value.
    /// * `magnitude_kind` - Type of magnitude scale.
    pub fn new(lon: f64, lat: f64, depth: f64, magnitude: f64, magnitude_kind: Magnitude) -> Self {
        Self {
            lon,
            lat,
            depth,
            magnitude,
            magnitude_kind,
        }
    }

    /// Convenience constructor for Local magnitude (Ml).
    pub fn new_ml(lon: f64, lat: f64, depth: f64, magnitude: f64) -> Self {
        Self::new(lon, lat, depth, magnitude, Magnitude::Ml)
    }

    /// Convenience constructor for Moment magnitude (Mw).
    pub fn new_mw(lon: f64, lat: f64, depth: f64, magnitude: f64) -> Self {
        Self::new(lon, lat, depth, magnitude, Magnitude::Mw)
    }
}

impl GmpePoint {
    /// Create a new GmpePoint instance.
    pub fn new(lon: f64, lat: f64, value: f64, kind: GmpePointKind) -> Self {
        Self {
            lon,
            lat,
            value,
            kind,
        }
    }

    /// Create a new Peak Ground Acceleration (PGA) point.
    pub fn new_pga(lon: f64, lat: f64, value: f64) -> Self {
        Self::new(lon, lat, value, GmpePointKind::Pga)
    }

    /// Create a new Peak Ground Velocity (PGV) point.
    pub fn new_pgv(lon: f64, lat: f64, value: f64) -> Self {
        Self::new(lon, lat, value, GmpePointKind::Pgv)
    }

    /// Create a new Peak Spectral Acceleration (PSA) point.
    pub fn new_psa(lon: f64, lat: f64, value: f64) -> Self {
        Self::new(lon, lat, value, GmpePointKind::Psa)
    }
}
