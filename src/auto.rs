use crate::{image::Image, model::Color};

pub fn reconstruct(image: &Image) -> Vec<Color> {
    let mut output = Vec::with_capacity(image.output.width * image.output.height);

    for y in 0..image.output.height {
        for x in 0..image.output.width {
            let symbol = sample_symbol(image, x, y);

            output.push(image.palette[&symbol]);
        }
    }

    output
}

fn sample_symbol(image: &Image, output_x: usize, output_y: usize) -> char {
    let source_x = map_coordinate(output_x, image.output.width, image.source.width);

    let source_y = map_coordinate(output_y, image.output.height, image.source.height);

    let x0 = source_x.floor() as usize;
    let y0 = source_y.floor() as usize;

    let x1 = (x0 + 1).min(image.source.width - 1);
    let y1 = (y0 + 1).min(image.source.height - 1);

    let fx = source_x - x0 as f64;
    let fy = source_y - y0 as f64;

    let samples = [
        (image.pixels[y0][x0], (1.0 - fx) * (1.0 - fy)),
        (image.pixels[y0][x1], fx * (1.0 - fy)),
        (image.pixels[y1][x0], (1.0 - fx) * fy),
        (image.pixels[y1][x1], fx * fy),
    ];

    choose_symbol(&samples)
}

fn choose_symbol(samples: &[(char, f64)]) -> char {
    let mut scores = Vec::<(char, f64)>::new();

    for &(symbol, weight) in samples {
        if weight <= 0.0 {
            continue;
        }

        if let Some((_, score)) = scores.iter_mut().find(|(existing, _)| *existing == symbol) {
            *score += weight;
        } else {
            scores.push((symbol, weight));
        }
    }

    scores
        .into_iter()
        .max_by(|(symbol_a, score_a), (symbol_b, score_b)| {
            score_a
                .partial_cmp(score_b)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| symbol_b.cmp(symbol_a))
        })
        .map(|(symbol, _)| symbol)
        .unwrap_or(' ')
}

fn map_coordinate(position: usize, output_size: usize, source_size: usize) -> f64 {
    if output_size <= 1 || source_size <= 1 {
        return 0.0;
    }

    position as f64 * (source_size - 1) as f64 / (output_size - 1) as f64
}

#[allow(dead_code)]
fn dominant_color(image: &Image, symbol: char) -> Color {
    image.palette[&symbol]
}
