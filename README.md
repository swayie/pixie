<p align="center">
  <img src="assets/icon.png" alt="pixie" width="80">
</p>

# pixie

![Rust](https://img.shields.io/badge/rust-1.98-111111?style=for-the-badge&logo=rust&logoColor=white)
![license](https://img.shields.io/badge/license-MIT-111111?style=for-the-badge)

A tiny text-based pixel art renderer.

## usage

```sh
pixie image.px -o image.png
````

PPM is written directly. Other formats are handled by ImageMagick.

```sh
pixie image.px --terminal
pixie image.px --check
```

## format

```text
$scale = 16

# = #FF0000
% = #000000

%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%
%%%%%%####%%%%%%
%%%%%######%%%%%
%%%%########%%%%
%%%%########%%%%
%%%%%######%%%%%
%%%%%%####%%%%%%
%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%
```

See [`examples/`](examples) for more.

## build

```sh
cargo build --release
```

You need Rust and ImageMagick for formats other than PPM.

## status

work in progress

## license

MIT

made by: swayie
