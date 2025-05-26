# ground-motion-lib

[![crates.io](https://img.shields.io/crates/v/ground-motion-lib.svg)](https://crates.io/crates/ground-motion-lib)  
[![docs.rs](https://docs.rs/ground-motion-lib/badge.svg)](https://docs.rs/ground-motion-lib)  
[![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)

---

## Overview

`ground-motion-lib` is a Rust library for seismic ground motion prediction
using state-of-the-art Ground Motion Prediction Equations (GMPEs), including
the Morikawa & Fujiwara (2013) models. It supports efficient, vectorized
computations over site grids, configurable earthquake scenarios, and provides
tools for loading site data, performing parallelized calculations, and
exporting results.

Designed for researchers, engineers, and seismologists, this library enables
robust and scalable seismic hazard modeling workflows in Rust.

## Features

- **Comprehensive GMPE models:** Morikawa & Fujiwara (2013) with crustal,
  interplate, and intraplate tectonic settings.
- **Vectorized & parallelized prediction:** Leverages Rayon for efficient
  computation across large site datasets.
- **Flexible site input loading:** CSV reading for VS30 and site-specific
  parameters.
- **Statistical summary tools:** Compute mean, median, standard deviation, min,
  and max of ground motion outputs.
- **Configurable models:** Supports loading GMPE configurations for flexible
  modeling.
- **Robust CSV output:** Export computed ground motion points for visualization
   or downstream use.
- **Modular design:** Clear separation of data models, computation, IO, and
  config handling.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ground-motion-lib = "0.1"
```

## Quick Start Example

```rust
use ground_motion_lib::gmm::{Vs30Point, Earthquake, Magnitude, GroundMotionModeling};
use ground_motion_lib::mf2013::MF2013;
use ground_motion_lib::configs::get_mf2013_lib_configs;
use ground_motion_lib::vectorized::calc_gmpe_vec;

fn main() {
    let points = vec![
        Vs30Point::new(142.5, 50.0, 400,, Some(200.), Some(0)),
        Vs30Point::new(142.6, 50.1, 350., Some(150.), Some(1)),
    ];

    let eq = Earthquake {
        lon: 142.4,
        lat: 50.0,
        depth: 10.0,
        magnitude: 6.5,
        magnitude_kind: Magnitude::Mw,
    };

    let gmpe_ref = get_mf2013_lib_configs().get("config_mf2013_crustal_pga").unwrap();

    let results = calc_gmpe_vec(&points, gmpe_ref, &eq);

    for r in results {
        println!("{:?}", r);
    }
}
```

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/ground-motion-lib).

## Contributing

Contributions, bug reports, and feature requests are welcome! Please open an
issue or submit a pull request.

## License

Licensed under the Apache License, Version 2.0. See
[LICENSE](https://github.com/geophystech/GroundMotion.rs/blob/main/LICENSE) for
details.

## Citation

If you use this library in academic work, please cite:

```text
@article{konovalov2022new,
  title={New Tools for Rapid Assessment of Felt Reports and a Case Study on Sakhalin Island},
  author={Konovalov, AV and Stepnov, AA and Bogdanov, ES and Dmitrienko, R Yu and Orlin, ID and Sychev, AS and Gavrilov, AV and Manaychev, KA and Tsoy, AT and Stepnova, Yu A},
  journal={Seismic Instruments},
  volume={58},
  number={6},
  pages={676--693},
  year={2022},
  publisher={Springer}
}
```
