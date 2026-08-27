// Copyright (c) 2026 Swayie
// SPDX-License-Identifier: MIT

use std::fs;

use crate::{cli, error::AppError, image::Image, parser, renderer};

pub fn run() -> Result<(), AppError> {
    let cli = cli::Cli::parse()?;

    if cli.version {
        println!("pixie {}", cli::VERSION);
        return Ok(());
    }

    if cli.help {
        cli::print_help();
        return Ok(());
    }

    let source = fs::read_to_string(&cli.input)
        .map_err(|error| AppError::io(format!("cannot read '{}'", cli.input.display()), error))?;

    let document = parser::parse(&source)?;
    let image = Image::from_document(document)?;

    // Keep validation on the same parsing and conversion path as rendering.
    if cli.check {
        if cli.verbose {
            println!(
                "valid: {}x{} source, {}x{} output",
                image.source.width, image.source.height, image.output.width, image.output.height
            );
        } else {
            println!("valid");
        }
        return Ok(());
    }

    if cli.terminal {
        if cli.verbose {
            eprintln!(
                "terminal: {}x{} source",
                image.source.width, image.source.height
            );
        }

        renderer::terminal(&image)?;
        return Ok(());
    }

    let output = cli
        .output
        .ok_or_else(|| AppError::usage("missing output path; use '-o <file>'"))?;

    if cli.verbose {
        eprintln!(
            "rendering {}x{} -> {}x{}",
            image.source.width, image.source.height, image.output.width, image.output.height
        );
    }

    renderer::file(&image, &output)?;

    if cli.verbose {
        println!("wrote {}", output.display());
    }

    Ok(())
}