//! Implementation of Morikawa & Fujiwara (2013) Ground Motion Prediction Equations (GMPE).
//!
//! This module defines the parameters and calculation logic for predicting
//! ground motion values (PGA, PGV, PSA) based on earthquake and site characteristics.

use crate::auxilary::{DL, G_GLOBAL};
use crate::gmm::{Earthquake, GmpePoint, GmpePointKind, GroundMotionModeling, Vs30Point};
use geo::{Distance, Haversine, Point};

/// Morikawa & Fujiwara (2013) Ground Motion Prediction Equation parameters.
#[derive(Debug)]
pub struct MF2013 {
    /// Magnitude upper limit (Mw0)
    pub mw0: f64,
    /// Coefficient for magnitude scaling
    pub a: f64,
    /// Coefficient for distance scaling
    pub b: f64,
    /// Constant term
    pub c: f64,
    /// Distance damping parameter
    pub d: f64,
    /// Exponent scaling factor for distance damping
    pub e: f64,
    /// Standard deviation of the log ground motion (not currently used)
    pub sigma: f64,
    /// Coefficient for deep sedimentary layer correction
    pub pd: f64,
    /// Minimum depth for deep sedimentary layer correction
    pub dl_min: f64,
    /// Reference depth for deep layer correction
    pub d0: f64,
    /// Coefficient for Vs30 amplification term
    pub ps: f64,
    /// Maximum Vs30 considered for amplification (Vs_max)
    pub vs_max: f64,
    /// Reference Vs30 value (V0)
    pub v0: f64,
    /// Coefficient for anomalous seismic intensity distribution (ASID)
    pub gamma: f64,
    /// Whether ASID correction is enabled
    pub asid: bool,
    /// Type of motion (PGA, PGV, PSA etc.)
    pub motion_kind: GmpePointKind,
}

impl MF2013 {
    /// Calculate predicted ground motion value (in physical units) for a site and earthquake.
    ///
    /// Note: Currently assumes a point source (no finite fault modeling).
    ///
    /// # Arguments
    ///
    /// * `epicentral_distance` - Horizontal distance from the site to the earthquake epicenter (km).
    /// * `eq_mag` - Earthquake moment magnitude (Mw).
    /// * `eq_depth` - Hypocentral depth (km).
    /// * `vs_30` - Average shear-wave velocity in the top 30 meters at the site (m/s).
    /// * `dl` - Depth to the 1400 m/s shear-wave velocity layer (m).
    /// * `xvf` - Binary flag for volcanic front effect (1.0 if oceanward of front, 0.0 otherwise).
    ///
    /// # Returns
    ///
    /// Predicted ground motion value in cm/s² (PGA, PSA) or cm/s (PGV).
    fn get_gmpe_by_distnace(
        &self,
        epicentral_distance: f64,
        eq_mag: f64,
        eq_depth: f64,
        vs_30: f64,
        dl: f64,
        xvf: f64,
    ) -> f64 {
        // Rupture distance assuming point source
        let r_rup = (epicentral_distance.powi(2) + eq_depth.powi(2)).sqrt();

        let magnitude = eq_mag.min(self.mw0);
        let a_m_w = self.a * magnitude;

        // Deep sedimentary layer correction
        let g_d = self.pd * (dl.max(self.dl_min) / self.d0).log10();

        // Main GMPE equation (log10 of predicted motion)
        // logA where A in cm/s^2 (pga,psa) or cm/s (pgv)
        let log_a = (a_m_w + self.b * r_rup + self.c)
            - (r_rup + self.d * 10.0_f64.powf(self.e * magnitude)).log10();

        // Amplification by Deep Sedimentary Layers
        // Apply deep layer correction
        let log_agd = log_a + g_d;

        // Vs30 site amplification
        let gs = self.ps * (vs_30.min(self.vs_max) / self.v0).log10();
        let log_ags = log_agd + gs;

        // Optional anomalous seismic intensity distribution correction
        if self.asid {
            let ai = self.gamma + xvf * (eq_depth - 30.);
            10.0_f64.powf(log_ags + ai)
        } else {
            10.0_f64.powf(log_ags)
        }
    }
}

impl GroundMotionModeling for MF2013 {
    /// Compute ground motion prediction at a given site point for a specified earthquake event.
    ///
    /// # Arguments
    ///
    /// * `point` - The site location and properties (longitude, latitude, Vs30, depth to 1400 m/s
    ///   layer, etc.).
    /// * `eq` - The earthquake event (magnitude, depth, hypocenter location).
    ///
    /// # Returns
    ///
    /// A `GmpePoint` containing the predicted ground motion value and associated metadata.
    fn calc_from_point(&self, point: &Vs30Point, eq: &Earthquake) -> GmpePoint {
        let epicentral_distance = Haversine
            .distance(Point::new(eq.lon, eq.lat), Point::new(point.lon, point.lat))
            / 1000.;
        let vs_30 = point.vs30;
        let dl = match point.dl {
            None => DL as f64,
            Some(dl) => dl,
        };
        let xvf = match point.xvf {
            None => 0.,
            Some(_) => 1.,
        };
        let mut ground_motion =
            self.get_gmpe_by_distnace(epicentral_distance, eq.magnitude, eq.depth, vs_30, dl, xvf);
        // convert cm/c^2 to %g
        if matches!(self.motion_kind, GmpePointKind::Pga | GmpePointKind::Psa) {
            ground_motion = ((ground_motion / 100.) / G_GLOBAL) * 100.;
        };
        GmpePoint {
            lon: point.lon,
            lat: point.lat,
            value: ground_motion,
            kind: self.motion_kind,
        }
    }
}
