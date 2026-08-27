// Copyright (c) 2026 Swayie
// SPDX-License-Identifier: MIT

use std::{fmt, io};

#[derive(Debug)]
pub struct AppError {
    kind: Kind,
    message: String,
    line: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Usage,
    Parse,
    Io,
    Process,
}

impl AppError {
    pub fn usage(message: impl Into<String>) -> Self {
        Self {
            kind: Kind::Usage,
            message: message.into(),
            line: None,
        }
    }

    pub fn parse(line: usize, message: impl Into<String>) -> Self {
        Self {
            kind: Kind::Parse,
            message: message.into(),
            line: Some(line),
        }
    }

    pub fn io(message: impl Into<String>, source: io::Error) -> Self {
        Self {
            kind: Kind::Io,
            message: format!("{}: {source}", message.into()),
            line: None,
        }
    }

    pub fn process(message: impl Into<String>) -> Self {
        Self {
            kind: Kind::Process,
            message: message.into(),
            line: None,
        }
    }

    // Keep command-line misuse distinct from failures during normal execution.
    pub fn report(&self) {
        let label = match self.kind {
            Kind::Usage => "usage",
            Kind::Parse | Kind::Io | Kind::Process => "error",
        };

        match self.line {
            Some(line) => eprintln!("pixie {label}: line {line}: {}", self.message),
            None => eprintln!("pixie {label}: {}", self.message),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}