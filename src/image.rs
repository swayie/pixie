// Copyright (c) 2026 Swayie
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use crate::{
    error::AppError,
    model::{Color, Document, Resolution},
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Image {
    pub source: Resolution,
    pub output: Resolution,
    pub palette: HashMap<char, Color>,
    pub pixels: Vec<Vec<char>>,
    pub reconstruction: bool,
    pub auto_resolution: bool,
    pub auto_scale: bool,
    pub effective_scale: Option<f64>,
}

impl Image {
    pub fn from_document(document: Document) -> Result<Self, AppError> {
        let source = Resolution::new(
            document.pixels[0].len(),
            document.pixels.len(),
        )
        .map_err(AppError::usage)?;

        let auto_resolution = has_auto(&document.auto, "resolution");
        let auto_scale = has_auto(&document.auto, "scale");

        let output = resolve_output_resolution(
            source,
            document.scale,
            document.resolution,
            auto_resolution,
            auto_scale,
        )?;

        let effective_scale = Some(output_scale(source, output));

        Ok(Self {
            source,
            output,
            palette: document.palette,
            pixels: document.pixels,
            reconstruction: auto_resolution || auto_scale,
            auto_resolution,
            auto_scale,
            effective_scale,
        })
    }

    pub fn color_at(&self, x: usize, y: usize) -> Color {
        self.palette[&self.pixels[y][x]]
    }
}

fn has_auto(auto: &[String], name: &str) -> bool {
    auto.iter().any(|item| item == name)
}

fn resolve_output_resolution(
    source: Resolution,
    scale: Option<u32>,
    resolution: Option<Resolution>,
    auto_resolution: bool,
    auto_scale: bool,
) -> Result<Resolution, AppError> {
    match (auto_resolution, auto_scale) {
        (true, true) => resolve_auto_resolution_and_scale(
            source,
            resolution,
        ),

        (true, false) => {
            let target = resolution.ok_or_else(|| {
                AppError::usage(
                    "'resolution' is enabled in '$auto', \
                     but '$resolution' is missing",
                )
            })?;

            validate_resolution_target(source, target)?;
            Ok(target)
        }

        (false, true) => {
            let scale = scale.ok_or_else(|| {
                AppError::usage(
                    "'scale' is enabled in '$auto', \
                     but '$scale' is missing",
                )
            })?;

            source
                .scaled(scale)
                .map_err(AppError::usage)
        }

        (false, false) => {
            if let Some(resolution) = resolution {
                return Ok(resolution);
            }

            if let Some(scale) = scale {
                return source
                    .scaled(scale)
                    .map_err(AppError::usage);
            }

            Ok(source)
        }
    }
}

fn resolve_auto_resolution_and_scale(
    source: Resolution,
    resolution: Option<Resolution>,
) -> Result<Resolution, AppError> {
    let target = resolution.ok_or_else(|| {
        AppError::usage(
            "'resolution' is enabled in '$auto', \
             but '$resolution' is missing",
        )
    })?;

    validate_resolution_target(source, target)?;

    Ok(target)
}

fn validate_resolution_target(
    source: Resolution,
    target: Resolution,
) -> Result<(), AppError> {
    if target.width < source.width || target.height < source.height {
        return Err(AppError::usage(
            "auto resolution cannot reduce the source dimensions",
        ));
    }

    if !target.same_aspect_ratio(source) {
        return Err(AppError::usage(
            "auto resolution must preserve the source aspect ratio",
        ));
    }

    Ok(())
}

fn output_scale(source: Resolution, output: Resolution) -> f64 {
    output.width as f64 / source.width as f64
}