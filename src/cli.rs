// Copyright (c) 2026 Swayie
// SPDX-License-Identifier: MIT

use std::{env, ffi::OsString, path::PathBuf};

use crate::error::AppError;

pub const VERSION: &str = "0.1.0";

#[derive(Debug, Default)]
pub struct Cli {
    pub input: PathBuf,
    pub output: Option<PathBuf>,
    pub terminal: bool,
    pub check: bool,
    pub verbose: bool,
    pub help: bool,
    pub version: bool,
}

impl Cli {
    pub fn parse() -> Result<Self, AppError> {
        Self::parse_args(env::args_os())
    }

    fn parse_args<I>(args: I) -> Result<Self, AppError>
    where
        I: IntoIterator<Item = OsString>,
    {
        let mut args = args.into_iter();
        let _program = args.next();
        let mut cli = Self::default();
        let mut input = None;
        let mut end_of_options = false;

        while let Some(arg) = args.next() {
            if !end_of_options && arg == "--" {
                end_of_options = true;
                continue;
            }

            if !end_of_options {
                match arg.to_str() {
                    Some("-h") | Some("--help") => cli.help = true,
                    Some("-V") | Some("--version") => cli.version = true,
                    Some("-t") | Some("--terminal") => cli.terminal = true,
                    Some("-c") | Some("--check") => cli.check = true,
                    Some("-v") | Some("--verbose") => cli.verbose = true,
                    Some("-o") | Some("--output") => {
                        let value = args
                            .next()
                            .ok_or_else(|| AppError::usage("'-o' expects an output path"))?;

                        if cli.output.is_some() {
                            return Err(AppError::usage("output path specified more than once"));
                        }

                        cli.output = Some(PathBuf::from(value));
                    }
                    Some(value) if value.starts_with('-') => {
                        return Err(AppError::usage(format!("unknown option '{value}'")));
                    }
                    _ => assign_input(&mut input, arg)?,
                }
            } else {
                // After `--`, every argument is treated as an input path.
                assign_input(&mut input, arg)?;
            }
        }

        if cli.help || cli.version {
            return Ok(cli);
        }

        cli.input = input.ok_or_else(|| AppError::usage("missing input file; see `pixie --help`"))?;

        // Validate incompatible flags and required output settings once parsing is complete.
        if cli.terminal && cli.output.is_some() {
            return Err(AppError::usage("'-o' cannot be used with '--terminal'"));
        }

        if cli.check && cli.output.is_some() {
            return Err(AppError::usage("'-o' cannot be used with '--check'"));
        }

        if cli.terminal && cli.check {
            return Err(AppError::usage(
                "'--terminal' cannot be combined with '--check'",
            ));
        }

        if !cli.terminal && !cli.check && cli.output.is_none() {
            return Err(AppError::usage("missing output path; use '-o <file>'"));
        }

        Ok(cli)
    }
}

fn assign_input(input: &mut Option<PathBuf>, value: OsString) -> Result<(), AppError> {
    if input.is_some() {
        return Err(AppError::usage("expected exactly one input file"));
    }

    *input = Some(PathBuf::from(value));
    Ok(())
}

pub fn print_help() {
    println!(
        "pixie {VERSION}

A small text-based pixel art renderer.

USAGE:
    pixie <INPUT> -o <OUTPUT>
    pixie <INPUT> --terminal
    pixie <INPUT> --check

OPTIONS:
    -o, --output <FILE>   write the rendered image
    -t, --terminal        render the source grid in the terminal
    -c, --check           validate the source without writing an image
    -v, --verbose         print processing details
    -V, --version         print the version
    -h, --help            print this help

OUTPUT:
    .ppm                  written directly by pixie
    other extensions      converted through ImageMagick's `magick`

EXAMPLES:
    pixie image.px -o image.png
    pixie image.px --terminal
    pixie image.px --check
"
    );
}