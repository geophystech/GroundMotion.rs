# GroundMotion.rs 📈🌏

A fast, parallelized Rust implementation of seismic ground motion prediction
models (GMPEs) with a clean separation between a reusable library
(`ground-motion-lib`) and a command-line application (`ground-motion-bin`) for
batch site-specific predictions.

This project is built around a modern, scalable Rust workspace structure
designed for both research and operational hazard modeling workflows.

<!-- vim-markdown-toc GFM -->

* [📦 Project Structure](#-project-structure)
* [Features](#features)
* [Implemented models](#implemented-models)
  * [Morikawa & Fujiwara (2013)](#morikawa--fujiwara-2013)
* [ground-motion-lib](#ground-motion-lib)
* [ground-motion-bin](#ground-motion-bin)
* [Full example](#full-example)
  * [Precompiled Release](#precompiled-release)
  * [Compile the Binary](#compile-the-binary)
  * [Get VS30 Input Data](#get-vs30-input-data)
  * [Run a Test Prediction — Neftegorsk Earthquake (1995)](#run-a-test-prediction--neftegorsk-earthquake-1995)
  * [Check Output Files](#check-output-files)

<!-- vim-markdown-toc -->

---

## 📦 Project Structure

```text
GroundMotion.rs/
├── Cargo.toml              # Workspace manifest
├── README.md               # This readme
├── ground-motion-lib/      # Core GMPE library crate
└── ground-motion-bin/      # Command-line application crate
```

## Features

⚡ Rayon-powered parallelized ground motion calculations.

📄 CSV-based data loaders and writers for site grids and GMPE outputs.

📊 Statistical summaries of predicted ground motions.

🛠️ Configurable GMPE scenarios via built-in or user-provided configurations.

🔌 Modular, workspace-based Rust project structure.

## Implemented models

### Morikawa & Fujiwara (2013)

* Crustal events: PGA, PGV, PSA at multiple periods.
* Interplate and intraplate events: Fully supported.
* ASID (site classification) adjustment support.
* Predefined GMPE configurations included.

## ground-motion-lib

[![crates.io](https://img.shields.io/crates/v/ground-motion-lib.svg)](https://crates.io/crates/ground-motion-lib)  
[![docs.rs](https://docs.rs/ground-motion-lib/badge.svg)](https://docs.rs/ground-motion-lib)  
[![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)

The `ground-motion-lib` crate is the core Rust library powering
`GroundMotion.rs`. It provides a modular, high-performance framework for
seismic ground motion prediction using GMPE models, featuring a clean,
extensible design.  Please follow the links above to access the full API
documentation or to browse the source code of this project.

## ground-motion-bin

A command-line interface (CLI) tool for seismic ground motion prediction using
the `ground-motion-lib` Rust library.

`ground-motion-bin` allows users to perform ground motion predictions on
site-specific VS30 points by leveraging configured GMPE models and earthquake
scenarios directly from the terminal.

Workflow:

* Load VS30 site points from CSV files
* Select from preconfigured GMPE models or specify custom configuration files (future)
* Define earthquake parameters (location, depth, magnitude) via CLI
* Perform parallelized ground motion predictions
* Export prediction results as CSV files
* List available GMPE models or display their configuration details

CLI Arguments:

```bash
ground-motion-bin -h
Input command line arguments

Usage: ground-motion-bin [OPTIONS] <--in-file <IN_FILE>|--list-configs|--show-config <SHOW_CONFIG>>

Options:
  -i, --in-file <IN_FILE>
          Input VS30 CSV file containing site data
  -u, --use-config <USE_CONFIG>
          Use a predefined GMPE configuration by name
  -c, --custom-config <CUSTOM_CONFIG>
          Provide a custom GMPE configuration TOML file
  -e, --earthquake <lon> <lat> <depth> <magnitude>
          Earthquake parameters e.g. --earthquake 141.1 50.2 10.0 4.5 (Mw assumed)
  -o, --out-file <OUT_FILE>
          Output CSV file to write computed GMPE values [default: out_gmpe_grid.txt]
  -d, --delimeter <DELIMETER>
          Delimiter character for input and output CSV files [default: "\t"]
  -l, --list-configs
          List all available GMPE configurations
  -s, --show-config <SHOW_CONFIG>
          Show details of a specific GMPE configuration by name
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```

## Full example

### Precompiled Release

You can download a precompiled binary for your platform from the
[Releases](https://github.com/geophystech/GroundMotion.rs/releases) page.

If you prefer, you can also compile the binary from source — see the
instructions in the next section.

### Compile the Binary

Clone the repo:

```bash
git clone https://github.com/geophystech/GroundMotion.rs.git .
cd GroundMotion.rs
```

Install Rust for your OS if you haven’t already, then build the project:

```bash
cargo build --release
```

Check that the CLI is working:

```bash
target/release/ground-motion-bin -h
```

### Get VS30 Input Data

Download GMT `grd` file from the [USGS Vs30 Models and Data
page](https://earthquake.usgs.gov/data/vs30/).

```bash
curl -O https://apps.usgs.gov/shakemap_geodata/vs30/global_vs30.grd
ls -lh global_vs30.grd
# Example output:
# -rw-r--r-- 1 user user 582M May 19 23:52 global_vs30.grd
```

Convert the grd file into a tab-delimited XYZ text file using
[GMT2XYZ](https://www.soest.hawaii.edu/gmt/gmt/html/man/grd2xyz.html) from
[GMT](https://www.soest.hawaii.edu/gmt/):

```bash
# Example: extract Sakhalin region
gmt grd2xyz global_vs30.grd -R139.0/146.0/39.0/56.0 > test_sakh_vs30.txt

# Check number of rows
cat test_sakh_vs30.txt |wc -l
# 1716481

# Preview file content
head test_sakh_vs30.txt
```

The output file will be tab-delimited by default.

### Run a Test Prediction — Neftegorsk Earthquake (1995)

Use the [1995 Neftegorsk
earthquake](https://en.wikipedia.org/wiki/1995_Neftegorsk_earthquake) as a test
case.

First, list available GMPE configurations and inspect one:

```bash
# List configs
target/release/ground-motion-bin -l

# Show details of a specific config
target/release/ground-motion-bin -s config_mf2013_crustal_pga
```

Now, run a PGA prediction using the `config_mf2013_crustal_pga` config:

```bash
target/release/ground-motion-bin -i test_sakh_vs30.txt \ 
  -u config_mf2013_crustal_pga --earthquake 142.83 52.63 11.0 7.1 \
  -o neftegorsk_pga.txt
```

Example stdout:

```text
Use test_sakh_vs30.txt as input grid...
Use config MF2013 {
    mw0: 8.1,
    a: 0.5507,
    b: -0.004531,
    c: 0.4631,
    d: 0.006875,
    e: 0.5,
    sigma: 0.377556,
    pd: 0.0663,
    dl_min: 100.0,
    d0: 250.0,
    ps: -0.3709,
    vs_max: 1950.0,
    v0: 350.0,
    gamma: 7.602e-5,
    asid: false,
    motion_kind: Pga,
}
Use Earthquake with parameters Earthquake {
    lon: 142.83,
    lat: 52.63,
    depth: 11.0,
    magnitude: 7.1,
    magnitude_kind: Mw,
}
Stats for out grid:
Stats {
    mean: 0.9098975207980508,
    std_dev: 3.5239501645854148,
    min: 1.270494568926633e-7,
    max: 68.434148866177,
    median: 0.006957938777551972,
}
Write gmpe points to neftegorsk_pga.txt...
Done
```

The operation should complete in less than a second on modern CPUs, including
IO.

### Check Output Files

Confirm the output content and number of points::

```bash
head test_sakh_vs30.txt

cat test_sakh_vs30.txt |wc -l # Should match input grid size
```

Tip:

You can also extend the base `vs30` file with additional `dl` (distance) and
`xvf` (optional site variable factor) columns. The tool will automatically
handle those extra columns during predictions.
