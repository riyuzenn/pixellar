/*
 * Copyright (c) 2023 riyuzenn <riyuzenn@gmail.com>
 * See the license file for more info
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, info, warn};

use lime::address::Address;
use lime::server::Server;
use lime::utils::bunny;
use lime::utils::Version;

mod args;
use crate::args::Arguments;

fn main() -> Result<()> {
    let arg = Arguments::parse();

    let host = arg.host.or(Some("0.0.0.0".to_string())).unwrap();
    let port = arg.port.or(Some(9001)).unwrap();

    let debug = arg.debug.or(Some(true)).unwrap();
    let size = arg.size.or(Some(10)).unwrap();
    let ver = arg
        .ver
        .or(Some(env!("CARGO_PKG_VERSION").to_string()))
        .unwrap();

    if debug {
        setup_logging();
    } else {
        warn!("Please use debugger to debug the server when it crashed");
    }

    let addr = Address::new(&host, port);
    let version = Version::from(&ver)?;

    let name = "Pixellar".red();
    let text = format!(
        "▲ Welcome to {} Server v{}",
        name,
        version.to_string().yellow()
    );

    bunny::say(&text);

    debug!("Creating Server object");
    let server = Server::new(addr.host, addr.port, size, debug, version.clone());
    info!("Server object is created & running");
    server.run_server(0);

    Ok(())
}

fn setup_logging() {
    // configure colors for the whole line
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        // we actually don't need to specify the color for debug and info, they are white by default
        .info(Color::White)
        .debug(Color::BrightBlack);
    // configure colors for the name of the level.
    // since almost all of them are the same as the color for the whole line, we
    // just clone `colors_line` and overwrite our changes
    let colors_level = colors_line.clone().info(Color::Green);

    let file = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file("server.log").unwrap());

    // here we set up our fern Dispatch
    let stdout = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        // change log levels for individual modules. Note: This looks for the record's target
        // field which defaults to the module path but can be overwritten with the `target`
        // parameter:
        // `info!(target="special_target", "This log message is about special_target");`
        // .level_for("pretty_colored", log::LevelFilter::Trace)
        // output to stdout
        .chain(std::io::stdout());

    fern::Dispatch::new()
        .level(log::LevelFilter::Trace)
        .chain(file)
        .chain(stdout)
        .apply()
        .unwrap();

    debug!("Logging setup finisehd");
}
