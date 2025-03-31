//! Ground motion prediction model configuration storage and retrieval.

use crate::gmm::GmpePointKind;
use crate::mf2013::MF2013;
use std::collections::HashMap;
use std::sync::OnceLock;

static CONFIGS: OnceLock<HashMap<&'static str, MF2013>> = OnceLock::new();

/// Lazily initializes and returns a reference to the global MF2013 configuration map.
///
/// This function ensures that the `CONFIGS` static is populated exactly once in a thread-safe
/// manner using [`OnceLock`]. On the first call, it builds the predefined models and stores them
/// in a `HashMap`. Subsequent calls return a shared reference to this map.
///
/// The map contains model configurations keyed by descriptive string identifiers such as
/// `"config_mf2013_crustal_pga"` or `"config_mf2013_crustal_pga_2"`.
///
/// # Returns
///
/// A reference to the `HashMap` containing predefined MF2013 model configurations.
///
/// # Example
///
/// ```rust
/// use ground_motion_lib::configs::get_mf2013_lib_configs;
///
/// let configs = get_mf2013_lib_configs();
/// let pga_model = configs.get("config_mf2013_crustal_pga").unwrap();
/// println!("Mw0 value: {}", pga_model.mw0);
/// ```
///
/// # Thread Safety
///
/// Internally uses `OnceLock` to ensure that the map is initialized only once and is safe to
/// access from multiple threads.
///
pub fn get_mf2013_lib_configs() -> &'static HashMap<&'static str, MF2013> {
    CONFIGS.get_or_init(|| {
        let mut map = HashMap::new();

        // Crustal PGA
        map.insert(
            "config_mf2013_crustal_pga",
            MF2013 {
                mw0: 8.1,
                a: 0.5507,
                b: -0.004531,
                c: 0.4631,
                d: 0.006875,
                e: 0.5,
                sigma: 0.377556,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.00,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // Crustal-2 PGA
        map.insert(
            "config_mf2013_crustal_pga_2",
            MF2013 {
                mw0: 8.1,
                a: 0.87,
                b: -0.0038,
                c: -1.726,
                d: 0.006,
                e: 0.5,
                sigma: 0.34,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // Interplate PGA
        map.insert(
            "config_mf2013_interplate_pga",
            MF2013 {
                mw0: 8.1,
                a: 0.5507,
                b: -0.004716,
                c: 0.5418,
                d: 0.006875,
                e: 0.5,
                sigma: 0.377556,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // Intraplate PGA
        map.insert(
            "config_mf2013_intraplate_pga",
            MF2013 {
                mw0: 8.1,
                a: 0.5507,
                b: -0.005273,
                c: 0.9338,
                d: 0.006875,
                e: 0.5,
                sigma: 0.377556,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // Intraplate PGA ASID=true
        map.insert(
            "config_mf2013_intraplate_pga_asid",
            MF2013 {
                mw0: 8.1,
                a: 0.5507,
                b: -0.005273,
                c: 0.9338,
                d: 0.006875,
                e: 0.5,
                sigma: 0.377556,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: true,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // Crustal PGV
        map.insert(
            "config_mf2013_crustal_pgv",
            MF2013 {
                mw0: 8.1,
                a: 0.6014,
                b: -0.002602,
                c: -1.1779,
                d: 0.002109,
                e: 0.5,
                sigma: 0.341184,
                pd: 0.2317,
                dl_min: 60.,
                d0: 250.,
                ps: -0.5546,
                vs_max: 1100.,
                v0: 350.,
                gamma: 0.00004693,
                asid: false,
                motion_kind: GmpePointKind::Pgv,
            },
        );

        // Interplate PGV
        map.insert(
            "config_mf2013_interplate_pgv",
            MF2013 {
                mw0: 8.1,
                a: 0.6014,
                b: -0.002375,
                c: -1.2682,
                d: 0.002109,
                e: 0.5,
                sigma: 0.341184,
                pd: 0.2317,
                dl_min: 60.,
                d0: 250.,
                ps: -0.5546,
                vs_max: 1100.,
                v0: 350.,
                gamma: 0.00004693,
                asid: false,
                motion_kind: GmpePointKind::Pgv,
            },
        );

        // Intraplate PGV
        map.insert(
            "config_mf2013_intraplate_pgv",
            MF2013 {
                mw0: 8.1,
                a: 0.6014,
                b: -0.003435,
                c: -0.8601,
                d: 0.002109,
                e: 0.5,
                sigma: 0.341184,
                pd: 0.2317,
                dl_min: 60.,
                d0: 250.,
                ps: -0.5546,
                vs_max: 1100.,
                v0: 350.,
                gamma: 0.00004693,
                asid: false,
                motion_kind: GmpePointKind::Pgv,
            },
        );

        // Crustal PSA 0.3s
        map.insert(
            "config_mf2013_crustal_psa_03",
            MF2013 {
                mw0: 8.1,
                a: 0.563,
                b: -0.004033,
                c: 0.639,
                d: 0.005205,
                e: 0.5,
                sigma: 0.407229,
                pd: 0.1006,
                dl_min: 21.,
                d0: 250.,
                ps: -0.6217,
                vs_max: 2000.,
                v0: 350.,
                gamma: 0.00007711,
                asid: false,
                motion_kind: GmpePointKind::Psa,
            },
        );

        // Interplate PSA 0.3s
        map.insert(
            "config_mf2013_interplate_psa_03",
            MF2013 {
                mw0: 8.1,
                a: 0.563,
                b: -0.00388,
                c: 0.6544,
                d: 0.005205,
                e: 0.5,
                sigma: 0.407229,
                pd: 0.1006,
                dl_min: 21.,
                d0: 250.,
                ps: -0.6217,
                vs_max: 2000.,
                v0: 350.,
                gamma: 0.00007711,
                asid: false,
                motion_kind: GmpePointKind::Psa,
            },
        );

        // Intraplate PSA 0.3s
        map.insert(
            "config_mf2013_intraplate_psa_03",
            MF2013 {
                mw0: 8.1,
                a: 0.563,
                b: -0.004427,
                c: 1.0482,
                d: 0.005205,
                e: 0.5,
                sigma: 0.407229,
                pd: 0.1006,
                dl_min: 21.,
                d0: 250.,
                ps: -0.6217,
                vs_max: 2000.,
                v0: 350.,
                gamma: 0.00007711,
                asid: false,
                motion_kind: GmpePointKind::Psa,
            },
        );

        // Crustal PSA 1.0s
        map.insert(
            "config_mf2013_crustal_psa_10",
            MF2013 {
                mw0: 8.1,
                a: 0.6011,
                b: -0.001955,
                c: -0.2766,
                d: 0.00055,
                e: 0.5,
                sigma: 0.410513,
                pd: 0.2744,
                dl_min: 39.32,
                d0: 250.,
                ps: -0.6755,
                vs_max: 1423.23,
                v0: 350.,
                gamma: 0.00005324,
                asid: false,
                motion_kind: GmpePointKind::Psa,
            },
        );

        // Interplate PSA 1.0s
        map.insert(
            "config_mf2013_interplate_psa_10",
            MF2013 {
                mw0: 8.1,
                a: 0.6011,
                b: -0.001256,
                c: -0.4191,
                d: 0.00055,
                e: 0.5,
                sigma: 0.410513,
                pd: 0.2744,
                dl_min: 39.32,
                d0: 250.,
                ps: -0.6755,
                vs_max: 1423.23,
                v0: 350.,
                gamma: 0.00005324,
                asid: false,
                motion_kind: GmpePointKind::Psa,
            },
        );

        // Intraplate PSA 1.0s
        map.insert(
            "config_mf2013_intraplate_psa_10",
            MF2013 {
                mw0: 8.1,
                a: 0.6011,
                b: -0.00229,
                c: -0.024,
                d: 0.00055,
                e: 0.5,
                sigma: 0.410513,
                pd: 0.2744,
                dl_min: 39.32,
                d0: 250.,
                ps: -0.6755,
                vs_max: 1423.23,
                v0: 350.,
                gamma: 0.00005324,
                asid: false,
                motion_kind: GmpePointKind::Psa,
            },
        );

        // Crustal PSA 3.0s
        map.insert(
            "config_mf2013_crustal_psa_30",
            MF2013 {
                mw0: 8.1,
                a: 0.7089,
                b: -0.001276,
                c: -1.6579,
                d: 0.001021,
                e: 0.5,
                sigma: 0.379064,
                pd: 0.3996,
                dl_min: 69.69,
                d0: 250.,
                ps: -0.4398,
                vs_max: 864.01,
                v0: 350.,
                gamma: 0.00002548,
                asid: false,
                motion_kind: GmpePointKind::Psa,
            },
        );

        // Interplate PSA 3.0s
        map.insert(
            "config_mf2013_interplate_psa_30",
            MF2013 {
                mw0: 8.1,
                a: 0.7089,
                b: -0.00047,
                c: -1.9088,
                d: 0.001021,
                e: 0.5,
                sigma: 0.379064,
                pd: 0.3996,
                dl_min: 69.69,
                d0: 250.,
                ps: -0.4398,
                vs_max: 864.01,
                v0: 350.,
                gamma: 0.00002548,
                asid: false,
                motion_kind: GmpePointKind::Psa,
            },
        );

        // Intraplate PSA 3.0s
        map.insert(
            "config_mf2013_intraplate_psa_30",
            MF2013 {
                mw0: 8.1,
                a: 0.7089,
                b: -0.001086,
                c: -1.5998,
                d: 0.001021,
                e: 0.5,
                sigma: 0.379064,
                pd: 0.3996,
                dl_min: 69.69,
                d0: 250.,
                ps: -0.4398,
                vs_max: 864.01,
                v0: 350.,
                gamma: 0.00002548,
                asid: false,
                motion_kind: GmpePointKind::Psa,
            },
        );

        // AB1995 PGA
        map.insert(
            "config_mf2013_ab1995",
            MF2013 {
                mw0: 8.1,
                a: 0.344,
                b: -0.0014,
                c: 1.141,
                d: 0.0005,
                e: 0.5,
                sigma: 0.308,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // AS1997 PGA
        map.insert(
            "config_mf2013_as1997",
            MF2013 {
                mw0: 8.1,
                a: 0.505,
                b: -0.0029,
                c: 0.41,
                d: 0.0026,
                e: 0.5,
                sigma: 0.272,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // ASB2013 PGA
        map.insert(
            "config_mf2013_asb2013",
            MF2013 {
                mw0: 8.1,
                a: 0.495,
                b: -0.0041,
                c: 0.468,
                d: 0.004,
                e: 0.5,
                sigma: 0.321,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // ASB2013 PGA 2
        map.insert(
            "config_mf2013_asb2013_2",
            MF2013 {
                mw0: 8.1,
                a: 0.495,
                b: 0.0,
                c: 0.468,
                d: 0.004,
                e: 0.5,
                sigma: 0.321,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // JSGGA2022 PGA
        map.insert(
            "config_mf2013_jsgga2022",
            MF2013 {
                mw0: 8.1,
                a: 0.447,
                b: -0.0021,
                c: 0.81,
                d: 0.0006,
                e: 0.5,
                sigma: 0.355,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // config MF2013 1 PGA
        map.insert(
            "config_mf2013_mf2013_1",
            MF2013 {
                mw0: 8.1,
                a: 0.5507,
                b: -0.004531,
                c: 0.4631,
                d: 0.006875,
                e: 0.5,
                sigma: 0.378,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // config MF2013 2 PGA
        map.insert(
            "config_mf2013_mf2013_2",
            MF2013 {
                mw0: 8.1,
                a: 0.5507,
                b: -0.004716,
                c: 0.5418,
                d: 0.006875,
                e: 0.5,
                sigma: 0.378,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // config MF2013 3 PGA
        map.insert(
            "config_mf2013_mf2013_3",
            MF2013 {
                mw0: 8.1,
                a: 0.5507,
                b: -0.005273,
                c: 0.9338,
                d: 0.006875,
                e: 0.5,
                sigma: 0.378,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // config AB1995 AS1997 PGA
        map.insert(
            "config_mf2013_ab1995_ab1997",
            MF2013 {
                mw0: 8.1,
                a: 0.43,
                b: -0.0025,
                c: 0.778,
                d: 0.0016,
                e: 0.5,
                sigma: 0.307,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // config AB1995 ASB2013 PGA
        map.insert(
            "config_mf2013_ab1995_asb2013",
            MF2013 {
                mw0: 8.1,
                a: 0.432,
                b: -0.0028,
                c: 0.735,
                d: 0.0021,
                e: 0.5,
                sigma: 0.327,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        // config Sakh2018 AS1997 PGA
        map.insert(
            "config_mf2013_sakh2018_as1997",
            MF2013 {
                mw0: 8.1,
                a: 0.552,
                b: -0.0027,
                c: 0.115,
                d: 0.0027,
                e: 0.5,
                sigma: 0.301,
                pd: 0.0663,
                dl_min: 100.,
                d0: 250.,
                ps: -0.3709,
                vs_max: 1950.,
                v0: 350.,
                gamma: 0.00007602,
                asid: false,
                motion_kind: GmpePointKind::Pga,
            },
        );

        map
    })
}
