// Copyright (c) 2026 Swayie
// SPDX-License-Identifier: MIT

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{error::AppError, image::Image, ppm};

pub fn convert(image: &Image, output: &Path) -> Result<(), AppError> {
    let temporary = temporary_ppm_path();
    let result = (|| {
        // Use PPM as a dependency-free intermediate format before handing the output to ImageMagick.
        ppm::write(image, &temporary)?;

        let status = Command::new("magick")
            .arg(&temporary)
            .arg(output)
            .status()
            .map_err(|error| {
                AppError::process(format!("cannot run ImageMagick 'magick': {error}"))
            })?;

        if !status.success() {
            return Err(AppError::process(format!(
                "ImageMagick failed while writing '{}'",
                output.display()
            )));
        }

        Ok(())
    })();

    let _ = fs::remove_file(&temporary);
    result
}

fn temporary_ppm_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_nanos())
        .unwrap_or_default();

    env::temp_dir().join(format!("pixie-{}-{timestamp}.ppm", std::process::id()))
}