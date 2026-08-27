// Copyright (c) 2026 Swayie
// SPDX-License-Identifier: MIT

use crate::{
    error::AppError,
    model::{Color, Document, Resolution},
};

pub fn parse(source: &str) -> Result<Document, AppError> {
    let mut scale = None;
    let mut resolution = None;
    let mut auto = Vec::new();
    let mut palette = std::collections::HashMap::new();
    let mut pixels = Vec::new();
    let mut in_grid = false;

    for (index, raw_line) in source.lines().enumerate() {
        let line_number = index + 1;
        let line = strip_comment(raw_line).trim_end();

        if line.trim().is_empty() {
            continue;
        }

        if in_grid {
            pixels.push(line.chars().collect());
            continue;
        }

        let trimmed = line.trim();

        if trimmed.starts_with('$') {
            parse_setting(trimmed, line_number, &mut scale, &mut resolution, &mut auto)?;
            continue;
        }

        if let Some((symbol, color)) = parse_palette(trimmed, line_number)? {
            if palette.insert(symbol, color).is_some() {
                return Err(AppError::parse(
                    line_number,
                    format!(
                        "palette symbol '{}' is already defined",
                        display_char(symbol)
                    ),
                ));
            }
            continue;
        }

        in_grid = true;
        pixels.push(trimmed.chars().collect());
    }

    validate_document(&pixels, &palette)?;

    Ok(Document {
        scale,
        resolution,
        auto,
        palette,
        pixels,
    })
}

fn validate_document(
    pixels: &[Vec<char>],
    palette: &std::collections::HashMap<char, Color>,
) -> Result<(), AppError> {
    if pixels.is_empty() {
        return Err(AppError::parse(1, "pixel grid is empty"));
    }

    if palette.is_empty() {
        return Err(AppError::parse(1, "palette is empty"));
    }

    let width = pixels[0].len();
    if width == 0 {
        return Err(AppError::parse(1, "pixel grid has zero width"));
    }

    for (row_index, row) in pixels.iter().enumerate() {
        if row.len() != width {
            return Err(AppError::parse(
                row_index + 1,
                format!(
                    "inconsistent row width: expected {width}, found {}",
                    row.len()
                ),
            ));
        }

        for (x, symbol) in row.iter().enumerate() {
            if !palette.contains_key(symbol) {
                return Err(AppError::parse(
                    row_index + 1,
                    format!(
                        "pixel '{}' at ({x}, {}) has no palette entry",
                        display_char(*symbol),
                        row_index
                    ),
                ));
            }
        }
    }

    Ok(())
}

fn strip_comment(line: &str) -> &str {
    line.split_once(';').map_or(line, |(head, _)| head)
}

fn parse_setting(
    line: &str,
    line_number: usize,
    scale: &mut Option<u32>,
    resolution: &mut Option<Resolution>,
    auto: &mut Vec<String>,
) -> Result<(), AppError> {
    let content = &line[1..];
    let (name, value) = content
        .split_once('=')
        .ok_or_else(|| AppError::parse(line_number, "expected '$name = value'"))?;

    let name = name.trim();
    let value = value.trim();

    if name.is_empty() {
        return Err(AppError::parse(line_number, "setting name is empty"));
    }

    if !valid_name(name) {
        return Err(AppError::parse(
            line_number,
            format!("invalid setting name '${name}'"),
        ));
    }

    if value.is_empty() {
        return Err(AppError::parse(
            line_number,
            format!("setting '${name}' has no value"),
        ));
    }

    match name {
        "scale" => {
            if scale.is_some() {
                return Err(AppError::parse(
                    line_number,
                    "'$scale' is defined more than once",
                ));
            }

            let parsed = value.parse::<u32>().map_err(|_| {
                AppError::parse(line_number, "invalid '$scale': expected a positive integer")
            })?;

            if parsed == 0 {
                return Err(AppError::parse(
                    line_number,
                    "'$scale' must be greater than zero",
                ));
            }

            *scale = Some(parsed);
        }
        "resolution" => {
            if resolution.is_some() {
                return Err(AppError::parse(
                    line_number,
                    "'$resolution' is defined more than once",
                ));
            }

            *resolution = Some(parse_resolution(value, line_number)?);
        }
        "auto" => {
            for item in parse_auto(value, line_number)? {
                if !auto.contains(&item) {
                    auto.push(item);
                }
            }
        }
        _ => {
            return Err(AppError::parse(
                line_number,
                format!("unknown setting '${name}'"),
            ));
        }
    }

    Ok(())
}

fn parse_resolution(value: &str, line_number: usize) -> Result<Resolution, AppError> {
    let parts: Vec<&str> = value.split_whitespace().collect();

    if parts.len() != 2 {
        return Err(AppError::parse(
            line_number,
            "'$resolution' expects '<width> <height>'",
        ));
    }

    let width = parts[0]
        .parse::<usize>()
        .map_err(|_| AppError::parse(line_number, "invalid resolution width"))?;
    let height = parts[1]
        .parse::<usize>()
        .map_err(|_| AppError::parse(line_number, "invalid resolution height"))?;

    Resolution::new(width, height).map_err(|message| AppError::parse(line_number, message))
}

fn parse_auto(
    value: &str,
    line_number: usize,
) -> Result<Vec<String>, AppError> {
    if !value.starts_with('[') || !value.ends_with(']') {
        return Err(AppError::parse(
            line_number,
            "'$auto' expects '[item, item, ...]'",
        ));
    }

    let inner = value[1..value.len() - 1].trim();

    if inner.is_empty() {
        return Err(AppError::parse(
            line_number,
            "'$auto' cannot be empty",
        ));
    }

    let mut items = Vec::new();

    for item in inner.split(',') {
        let item = item.trim();

        if item.is_empty() {
            return Err(AppError::parse(
                line_number,
                "empty item in '$auto'",
            ));
        }

        if !valid_name(item) {
            return Err(AppError::parse(
                line_number,
                format!("invalid auto feature '{item}'"),
            ));
        }

        match item {
            "resolution" | "scale" => {}

            _ => {
                return Err(AppError::parse(
                    line_number,
                    format!("unknown auto feature '{item}'"),
                ));
            }
        }

        if !items.iter().any(|existing| existing == item) {
            items.push(item.to_string());
        }
    }

    Ok(items)
}

fn parse_palette(line: &str, line_number: usize) -> Result<Option<(char, Color)>, AppError> {
    let Some((left, right)) = line.split_once('=') else {
        return Ok(None);
    };

    let symbol = left.trim();
    let value = right.trim();
    let mut chars = symbol.chars();
    let Some(symbol) = chars.next() else {
        return Err(AppError::parse(line_number, "palette symbol is empty"));
    };

    if chars.next().is_some() {
        return Err(AppError::parse(
            line_number,
            "palette symbols must contain exactly one character",
        ));
    }

    validate_symbol(symbol, line_number)?;
    let color = parse_color(value, line_number)?;

    Ok(Some((symbol, color)))
}

fn validate_symbol(symbol: char, line_number: usize) -> Result<(), AppError> {
    if symbol.is_whitespace() {
        return Err(AppError::parse(
            line_number,
            "whitespace cannot be used as a palette symbol",
        ));
    }

    if symbol.is_ascii_lowercase() {
        return Err(AppError::parse(
            line_number,
            "lowercase letters 'a-z' are reserved",
        ));
    }

    match symbol {
        '$' | '=' | ';' => Err(AppError::parse(
            line_number,
            format!("'{}' is reserved", display_char(symbol)),
        )),
        _ => Ok(()),
    }
}

fn parse_color(value: &str, line_number: usize) -> Result<Color, AppError> {
    let bytes = value.as_bytes();

    if bytes.len() != 7 || bytes[0] != b'#' {
        return Err(AppError::parse(
            line_number,
            format!("invalid color '{value}'; expected '#RRGGBB'"),
        ));
    }

    Ok(Color {
        r: parse_byte(&bytes[1..3], line_number)?,
        g: parse_byte(&bytes[3..5], line_number)?,
        b: parse_byte(&bytes[5..7], line_number)?,
    })
}

fn parse_byte(value: &[u8], line_number: usize) -> Result<u8, AppError> {
    let high = hex_digit(value[0], line_number)?;
    let low = hex_digit(value[1], line_number)?;
    Ok((high << 4) | low)
}

fn hex_digit(value: u8, line_number: usize) -> Result<u8, AppError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        b'A'..=b'F' => Ok(value - b'A' + 10),
        _ => Err(AppError::parse(
            line_number,
            format!(
                "invalid hexadecimal character '{}'; expected #RRGGBB",
                value as char
            ),
        )),
    }
}

fn valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn display_char(value: char) -> String {
    match value {
        ' ' => "space".into(),
        '\t' => "\\t".into(),
        '\r' => "\\r".into(),
        '\n' => "\\n".into(),
        other => other.to_string(),
    }
}