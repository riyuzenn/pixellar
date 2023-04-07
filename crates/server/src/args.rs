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

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    
    /// The host of the server to be binded to.
    /// Default: 0.0.0.0
    #[arg(short, long)]
    pub host: Option<String>,

    /// The port of the server to be binded to.
    /// Default: 9001
    #[arg(short, long)]
    pub port: Option<u16>,

    /// Number of client the server will accept
    /// Default Value: 10 connections
    #[arg(short, long)]
    pub size: Option<u64>,

    /// Set the debug to true or false
    /// Default: true
    #[arg(short, long)]
    pub debug: Option<bool>,

    /// Set the server version
    /// Default: cargo version
    #[arg(short, long)]
    pub ver: Option<String>,
    
}
