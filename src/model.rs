// Copyright (c) 2026 Swayie
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(unused)]
impl Color {
    pub fn distance_squared(self, other: Self) -> u32 {
        let dr = self.r as i32 - other.r as i32;
        let dg = self.g as i32 - other.g as i32;
        let db = self.b as i32 - other.b as i32;

        (dr * dr + dg * dg + db * db) as u32
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

impl Resolution {
    pub fn new(width: usize, height: usize) -> Result<Self, String> {
        if width == 0 || height == 0 {
            return Err("resolution dimensions must be greater than zero".into());
        }

        Ok(Self { width, height })
    }

    pub fn scaled(self, scale: u32) -> Result<Self, String> {
        let scale = scale as usize;

        let width = self
            .width
            .checked_mul(scale)
            .ok_or("scaled width overflows usize")?;

        let height = self
            .height
            .checked_mul(scale)
            .ok_or("scaled height overflows usize")?;

        Self::new(width, height)
    }

    pub fn same_aspect_ratio(self, other: Self) -> bool {
        self.width * other.height == self.height * other.width
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    pub scale: Option<u32>,
    pub resolution: Option<Resolution>,
    pub auto: Vec<String>,
    pub palette: HashMap<char, Color>,
    pub pixels: Vec<Vec<char>>,
}