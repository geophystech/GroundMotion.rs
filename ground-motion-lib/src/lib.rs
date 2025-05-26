//! # `ground_motion_lib`
//!
//! **A performant, modular Rust library for computing and analyzing ground motion predictions
//! using GMPE (Ground Motion Prediction Equation) models.**
//!
//! This crate provides data structures, model implementations, vectorized parallel computation routines,
//! and file I/O utilities for earthquake ground motion prediction workflows.
//!
//! ## Features
//!
//! - Modular ground motion model interface via the [`GroundMotionModeling`](crate::gmm::GroundMotionModeling) trait.
//! - Full implementation of the **Morikawa & Fujiwara (2013)** GMPE models via the [`mf2013`](crate::mf2013) module.
//! - Parallelized ground motion calculations with Rayon for efficient batch processing ([`vectorized`](crate::vectorized)).
//! - CSV-based readers and writers for site-specific input points and GMPE output values.
//! - Config management for model presets ([`configs`](crate::configs)).
//!
//! ## Module Overview
//!
//! - [`auxilary`](crate::auxilary) — Supporting utility functions (internal use).
//! - [`configs`](crate::configs) — Predefined model configuration loader.
//! - [`gmm`](crate::gmm) — Core data types and GMPE trait definitions.
//! - [`mf2013`](crate::mf2013) — Implementation of the Morikawa & Fujiwara (2013) GMPE models.
//! - [`readers`](crate::readers) — CSV-based input data loaders for site points.
//! - [`vectorized`](crate::vectorized) — Parallel ground motion calculation and statistics routines.
//! - [`writers`](crate::writers) — CSV-based output writers for GMPE prediction results.
//!
//! ## Example
//!
//! ```rust
//! use ground_motion_lib::configs::get_mf2013_lib_configs;
//! use ground_motion_lib::gmm::{Earthquake, Magnitude, Vs30Point};
//! use ground_motion_lib::vectorized::calc_gmpe_vec;
//!
//! let points = vec![
//!     Vs30Point::new(142.5, 50.0, 400., Some(200.), Some(0)),
//!     Vs30Point::new(142.6, 50.1, 350., Some(150.), Some(1)),
//! ];
//!
//! let eq = Earthquake {
//!     lon: 142.4,
//!     lat: 50.0,
//!     depth: 10.0,
//!     magnitude: 6.5,
//!     magnitude_kind: Magnitude::Mw,
//! };
//!
//! let gmpe_ref = get_mf2013_lib_configs().get("config_mf2013_crustal_pga").unwrap();
//!
//! let results = calc_gmpe_vec(&points, gmpe_ref, &eq);
//! println!("{results:?}");
//! ```
//!
//! ## Parallelism
//!
//! This crate uses [`Rayon`](https://docs.rs/rayon/latest/rayon/) for data-parallel ground motion
//! calculations and statistical summaries, with sensible defaults for thread pool management.
//!
//! ## Future Work
//!
//! Planned extensions include:
//!
//! - Additional GMPE model families
//! - Spatial interpolation utilities
//! - Uncertainty propagation routines
//! - Integrated hazard curve calculators
//!
//! ## License
//!
//! Licensed under the Apache License, Version 2.0 ([Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0))
//!
//! ---
//!
//! ```text
//! Copyright 2025 Andrey Stepnov, GEOPHYSTECH LLC
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.
//! ```
//! ---

pub mod auxilary;
pub mod configs;
pub mod gmm;
pub mod mf2013;
pub mod readers;
pub mod vectorized;
pub mod writers;
