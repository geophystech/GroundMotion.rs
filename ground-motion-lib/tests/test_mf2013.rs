use std::error::Error;

use ground_motion_lib::auxilary::{approx_equal, round_to_places};
use ground_motion_lib::configs::get_mf2013_lib_configs;
use ground_motion_lib::gmm::{Earthquake, GmpePoint, GmpePointKind, Magnitude, Vs30Point};
use ground_motion_lib::readers::read_vs30_points;
use ground_motion_lib::vectorized::calc_gmpe_vec;

const EPSILON: f64 = 1e-6;
const CSV_DELIMETER: u8 = b'\t';

const EQ6: Earthquake = Earthquake {
    lon: 143.04,
    lat: 51.92,
    depth: 13.,
    magnitude: 6.,
    magnitude_kind: Magnitude::Mw,
};
const EQ7: Earthquake = Earthquake {
    lon: 143.04,
    lat: 51.92,
    depth: 13.,
    magnitude: 7.,
    magnitude_kind: Magnitude::Mw,
};
const EQ85: Earthquake = Earthquake {
    lon: 143.04,
    lat: 51.92,
    depth: 13.,
    magnitude: 8.5,
    magnitude_kind: Magnitude::Mw,
};

const GRID_EPICENTER: Vs30Point = Vs30Point {
    lon: 143.04,
    lat: 51.92,
    vs30: 350,
    dl: None,
    xvf: None,
};

const GRID_SIZE: usize = 17;
const ROUND_PLACES: u32 = 2;
const VS_30_FILE: &str = "tests/data/testvs30.txt";
const VS_30_WITH_DL_FILE: &str = "tests/data/testvs30dl.txt";

fn sum_and_round_values(points: &[GmpePoint]) -> f64 {
    let mut sum = 0.;
    for point in points {
        sum += point.value;
    }
    round_to_places(sum, ROUND_PLACES)
}

#[test]
fn test_mf2013_const_dl() -> Result<(), Box<dyn Error>> {
    let vs_30_grid = read_vs30_points(VS_30_FILE, CSV_DELIMETER)?;
    let configs = get_mf2013_lib_configs();

    let config_ref = configs.get("config_mf2013_crustal_pga").unwrap();
    let epicenter_pga = GRID_EPICENTER.get_gm(config_ref, &EQ7).value;
    assert!(approx_equal(
        round_to_places(epicenter_pga, ROUND_PLACES),
        53.28,
        EPSILON
    ));

    let gmpe_points = calc_gmpe_vec(&vs_30_grid, config_ref, &EQ6);
    assert!(gmpe_points.len() == GRID_SIZE);
    assert!(matches!(gmpe_points[0].kind, GmpePointKind::Pga));
    let value = sum_and_round_values(&gmpe_points);
    assert!(approx_equal(value, 3.4, EPSILON));

    let config_ref = configs.get("config_mf2013_crustal_pgv").unwrap();
    let gmpe_points = calc_gmpe_vec(&vs_30_grid, config_ref, &EQ6);
    assert!(gmpe_points.len() == GRID_SIZE);
    assert!(matches!(gmpe_points[0].kind, GmpePointKind::Pgv));
    let value = sum_and_round_values(&gmpe_points);
    assert!(approx_equal(value, 4.63, EPSILON));

    let config_ref = configs.get("config_mf2013_crustal_psa_10").unwrap();
    let gmpe_points = calc_gmpe_vec(&vs_30_grid, config_ref, &EQ6);
    assert!(gmpe_points.len() == GRID_SIZE);
    assert!(matches!(gmpe_points[0].kind, GmpePointKind::Psa));
    let value = sum_and_round_values(&gmpe_points);
    assert!(approx_equal(value, 5.49, EPSILON));

    let config_ref = configs.get("config_mf2013_crustal_psa_30").unwrap();
    let gmpe_points = calc_gmpe_vec(&vs_30_grid, config_ref, &EQ6);
    assert!(gmpe_points.len() == GRID_SIZE);
    assert!(matches!(gmpe_points[0].kind, GmpePointKind::Psa));
    let value = sum_and_round_values(&gmpe_points);
    assert!(approx_equal(value, 1.42, EPSILON));

    let config_ref = configs.get("config_mf2013_crustal_pga").unwrap();
    let gmpe_points = calc_gmpe_vec(&vs_30_grid, config_ref, &EQ85);
    assert!(gmpe_points.len() == GRID_SIZE);
    assert!(matches!(gmpe_points[0].kind, GmpePointKind::Pga));
    let value = sum_and_round_values(&gmpe_points);
    assert!(approx_equal(value, 37.8, EPSILON));

    let config_ref = configs.get("config_mf2013_intraplate_pga_asid").unwrap();
    let gmpe_points = calc_gmpe_vec(&vs_30_grid, config_ref, &EQ85);
    assert!(gmpe_points.len() == GRID_SIZE);
    assert!(matches!(gmpe_points[0].kind, GmpePointKind::Pga));
    let value = sum_and_round_values(&gmpe_points);
    println!("{value}");
    assert!(approx_equal(value, 74.86, EPSILON));

    Ok(())
}

#[test]
fn test_mf2013_dl_on_grid() -> Result<(), Box<dyn Error>> {
    let vs_30_grid = read_vs30_points(VS_30_WITH_DL_FILE, CSV_DELIMETER)?;
    let configs = get_mf2013_lib_configs();

    let config_ref = configs.get("config_mf2013_crustal_pga").unwrap();
    let gmpe_points = calc_gmpe_vec(&vs_30_grid, config_ref, &EQ6);
    assert!(gmpe_points.len() == GRID_SIZE);
    assert!(matches!(gmpe_points[0].kind, GmpePointKind::Pga));
    let value = sum_and_round_values(&gmpe_points);
    assert!(approx_equal(value, 506.55, EPSILON));

    let config_ref = configs.get("config_mf2013_crustal_pgv").unwrap();
    let gmpe_points = calc_gmpe_vec(&vs_30_grid, config_ref, &EQ85);
    assert!(gmpe_points.len() == GRID_SIZE);
    assert!(matches!(gmpe_points[0].kind, GmpePointKind::Pgv));
    let value = sum_and_round_values(&gmpe_points);
    assert!(approx_equal(value, 2989.47, EPSILON));

    let config_ref = configs.get("config_mf2013_crustal_psa_03").unwrap();
    let gmpe_points = calc_gmpe_vec(&vs_30_grid, config_ref, &EQ85);
    assert!(gmpe_points.len() == GRID_SIZE);
    assert!(matches!(gmpe_points[0].kind, GmpePointKind::Psa));
    let value = sum_and_round_values(&gmpe_points);
    println!("{value}");
    assert!(approx_equal(value, 4177.5, EPSILON));

    Ok(())
}
