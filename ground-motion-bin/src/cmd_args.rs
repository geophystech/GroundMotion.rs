use clap::{ArgGroup, Parser};

/// Input command line arguments.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("input_mode")
        .required(true)
        .args(&["in_file", "list_configs", "show_config"]),
))]
#[command(group(
    ArgGroup::new("config_source")
        .args(&["use_config", "custom_config"])
        .multiple(false) // make them mutually exclusive
))]
pub struct CmdArgs {
    /// Input VS30 CSV file containing site data.
    ///
    /// Requires earthquake parameters (`--earthquake`) and a config source (`--use-config` or `--custom-config`).
    #[arg(short, long, requires_all = &["earthquake"],  requires = "config_source")]
    pub in_file: Option<String>,


    /// Use a predefined GMPE configuration by name.
    ///
    /// Mutually exclusive with `--custom-config`.
    #[arg(short, long)]
    pub use_config: Option<String>,


    /// Provide a custom GMPE configuration TOML file.
    ///
    /// *Not implemented yet.*
    #[arg(short, long)]
    pub custom_config: Option<String>,

    /// Earthquake parameters e.g. --earthquake 141.1 50.2 10.0 4.5 (Mw assumed).
    ///
    /// Requires `--in-file` to be set.
    #[arg(short, long, num_args = 4, value_names = ["lon", "lat", "depth", "magnitude"])]
    pub earthquake: Option<Vec<f64>>,

    /// Output CSV file to write computed GMPE values.
    ///
    /// Defaults to `out_gmpe_grid.txt`.
    #[arg(short, long, default_value = "out_gmpe_grid.txt")]
    pub out_file: String,

    /// Delimiter character for input and output CSV files.
    ///
    /// Defaults to tab (`'\t'`).
    #[arg(short, long, default_value = "\t")]
    pub delimeter: char,

    /// List all available GMPE configurations.
    #[arg(short, long)]
    pub list_configs: bool,

    /// Show details of a specific GMPE configuration by name.
    #[arg(short, long)]
    pub show_config: Option<String>,
}
