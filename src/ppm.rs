// Copyright (c) 2026 Swayie
// SPDX-License-Identifier: MIT

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{error::AppError, image::Image};

pub fn write(image: &Image, output: &Path) -> Result<(), AppError> {
    let file = File::create(output)
        .map_err(|error| AppError::io(format!("cannot create '{}'", output.display()), error))?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P6").map_err(|error| AppError::io("cannot write PPM header", error))?;
    writeln!(writer, "{} {}", image.output.width, image.output.height)
        .map_err(|error| AppError::io("cannot write PPM header", error))?;
    writeln!(writer, "255").map_err(|error| AppError::io("cannot write PPM header", error))?;

    // Reconstruction resamples the image; otherwise, each output pixel maps directly to a source pixel.
    if image.reconstruction {
        for color in crate::auto::reconstruct(image) {
            write_pixel(&mut writer, color)?;
        }
    } else {
        for y in 0..image.output.height {
            let source_y = y * image.source.height / image.output.height;

            for x in 0..image.output.width {
                let source_x = x * image.source.width / image.output.width;
                write_pixel(&mut writer, image.color_at(source_x, source_y))?;
            }
        }
    }

    writer
        .flush()
        .map_err(|error| AppError::io("cannot flush PPM output", error))
}

fn write_pixel<W: Write>(writer: &mut W, color: crate::model::Color) -> Result<(), AppError> {
    writer
        .write_all(&[color.r, color.g, color.b])
        .map_err(|error| AppError::io("cannot write image data", error))
}