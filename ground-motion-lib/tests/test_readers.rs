use std::error::Error;

use ground_motion_lib::auxilary::approx_equal;
use ground_motion_lib::readers::read_vs30_points;

const EPSILON: f64 = 1e-6;
const CSV_DELIMETER: u8 = b'\t';

#[test]
fn test_read_usgs_vs_30_grid_wo_dl() -> Result<(), Box<dyn Error>> {
    let vs_30_file = "tests/data/testvs30.txt";
    let vs_30_grid = read_vs30_points(vs_30_file, CSV_DELIMETER)?;
    let mut lon: f64 = 0.;
    let mut lat: f64 = 0.;
    let mut vs30: f64 = 0.;

    for point in &vs_30_grid {
        lon += point.lon;
        lat += point.lat;
        vs30 += point.vs30;
        assert!(point.dl.is_none());
    }
    assert!(approx_equal(lon, 2395.229157, EPSILON));
    assert!(approx_equal(lat, 910.704195, EPSILON));
    assert!(approx_equal(vs30, 12400., EPSILON));

    Ok(())
}

#[test]
fn test_read_usgs_vs_30_grid_with_dl() -> Result<(), Box<dyn Error>> {
    let vs_30_file = "tests/data/testvs30dl.txt";
    let vs_30_grid = read_vs30_points(vs_30_file, CSV_DELIMETER)?;
    let mut lon: f64 = 0.;
    let mut lat: f64 = 0.;
    let mut vs30: f64 = 0.;
    let mut dl: f64 = 0.;

    for point in &vs_30_grid {
        lon += point.lon;
        lat += point.lat;
        vs30 += point.vs30;
        match &point.dl {
            None => return Err("That value should be not processed as None".into()),
            Some(value) => {
                dl += value;
            }
        }
    }
    assert!(approx_equal(lon, 2431.68, EPSILON));
    assert!(approx_equal(lat, 882.64, EPSILON));
    assert!(approx_equal(vs30, 5100., EPSILON));
    assert!(approx_equal(dl, 15300., EPSILON));

    Ok(())
}
