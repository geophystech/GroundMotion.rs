mod cmd_args;
use clap::Parser;
use ground_motion_lib::configs::get_mf2013_lib_configs;
use ground_motion_lib::gmm::Earthquake;
use ground_motion_lib::readers::read_vs30_points;
use ground_motion_lib::vectorized::{calc_gmpe_vec, compute_stats};
use ground_motion_lib::writers::write_gmpe_points;

use crate::cmd_args::CmdArgs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cmd_args = CmdArgs::parse();
    println!("{cmd_args:?}");

    let configs = get_mf2013_lib_configs();

    if cmd_args.list_configs {
        let keys: Vec<_> = configs.keys().cloned().collect();
        for key in keys {
            println!("{}", key);
        }
    };

    if let Some(config_name) = cmd_args.show_config {
        let conf = configs.get(config_name.as_str());
        match conf {
            None => {
                println!("Config not found by name, use `--list-configs` to see avaliable keys.")
            }
            Some(cfg) => println!("{cfg:#?}"),
        }
    };

    if let (Some(ref vs_30_file), Some(ref config_name), Some(ref eq)) =
        (cmd_args.in_file, cmd_args.use_config, cmd_args.earthquake)
    {
        println!("Use {vs_30_file} as input grid...");
        let delim = cmd_args.delimeter as u8;
        let vs30_grid = read_vs30_points(vs_30_file, delim)?;

        let conf = configs.get(config_name.as_str());
        let running_config = match conf {
            None => {
                return Err(
                    "Config not found by name, use `--list-configs` to see avaliable keys.".into(),
                );
            }
            Some(cfg) => {
                println!("Use config {cfg:#?}");
                cfg
            }
        };

        let eq = Earthquake::new_mw(eq[0], eq[1], eq[2], eq[3]);
        println!("Use Earthquake with parameters {eq:#?}");

        let out_grid = calc_gmpe_vec(&vs30_grid, running_config, &eq);
        let grid_stat = compute_stats(&out_grid);
        println!("Stats for out grid:");
        println!("{grid_stat:#?}");

        let out_file = &cmd_args.out_file;
        println!("Write gmpe points to {out_file}...");
        write_gmpe_points(out_file, delim, &out_grid)?;
        println!("Done");


    };

    Ok(())
}
