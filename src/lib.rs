use std::collections::HashMap;
use pyo3::prelude::*;

fn out_of_bounds(pos: (i32, i32), width: i32, height: i32) -> bool {
    let (x, y) = pos;
    x < 0 || x >= width || y < 0 || y >= height
}

// This version is almost verbatim from the pure Python version
#[pyfunction]
fn get_neighborhood_hashmap(
    pos: (i32, i32),
    moore: bool,
    include_center: bool,
    radius: i32,
    torus: bool,
    width: i32,
    height: i32,
) -> PyResult<Vec<(i32, i32)>> {
    let (x, y) = pos;
    let mut neighborhood = HashMap::new();

    if x >= radius && width - x > radius && y >= radius && height - y > radius {
        for new_x in (x - radius)..=(x + radius) {
            for new_y in (y - radius)..=(y + radius) {
                if !moore && (new_x - x).abs() + (new_y - y).abs() > radius {
                    continue;
                }
                neighborhood.insert((new_x, new_y), true);
            }
        }
    } else {
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                if !moore && dx.abs() + dy.abs() > radius {
                    continue;
                }

                let mut new_x = x + dx;
                let mut new_y = y + dy;

                if torus {
                    // However, there's a nuance to be aware of: the % operator in Rust, when used
                    // with signed integers, can return a negative result if the left operand
                    // (new_x in this case) is negative. If you're implementing a torus and
                    // expecting new_x to always be a positive value within the range [0, width),
                    // you might need to adjust the result to ensure it's non-negative. This
                    // adjustment was included in the previous Rust code example:
                    new_x = ((new_x % width) + width) % width;
                    new_y = ((new_y % height) + height) % height;
                }

                if !out_of_bounds((new_x, new_y), width, height) {
                    neighborhood.insert((new_x, new_y), true);
                }
            }
        }
    }

    if !include_center {
        neighborhood.remove(&pos);
    }

    // Collect the keys from the HashMap into a Vec<(i32, i32)>
    Ok(neighborhood.keys().cloned().collect())
}

#[pyfunction]
fn get_neighborhood_vector(
    pos: (i32, i32),
    moore: bool,
    include_center: bool,
    radius: i32,
    torus: bool,
    width: i32,
    height: i32,
) -> PyResult<Vec<(i32, i32)>> {
    let (x, y) = pos;
    // Estimate the maximum possible size of the neighborhood
    let capacity = (2 * radius + 1).pow(2) as usize;
    let mut neighborhood = Vec::with_capacity(capacity);

    let check_bounds = |x: i32, y: i32| -> bool {
        !(x < 0 || x >= width || y < 0 || y >= height)
    };

    for dx in -radius..=radius {
        for dy in -radius..=radius {
            if !moore && dx.abs() + dy.abs() > radius {
                continue;
            }

            let mut new_x = x + dx;
            let mut new_y = y + dy;

            if torus {
                new_x = ((new_x % width) + width) % width;
                new_y = ((new_y % height) + height) % height;
            } else if !check_bounds(new_x, new_y) {
                continue;
            }

            if !(!include_center && dx == 0 && dy == 0) {
                neighborhood.push((new_x, new_y));
            }
        }
    }

    Ok(neighborhood)
}

#[pyfunction]
fn get_neighborhood_old_method(
    pos: (i32, i32),
    moore: bool,
    radius: i32,
    width: i32,
    height: i32,
) -> Vec<(i32, i32)> {
    let (x, y) = pos;
    let xfrom = std::cmp::max(0, x - radius);
    let xto = std::cmp::min(width, x + radius + 1);
    let yfrom = std::cmp::max(0, y - radius);
    let yto = std::cmp::min(height, y + radius + 1);

    let max_neighborhood_count = (xto - xfrom) as usize * (yto - yfrom) as usize;
    let mut neighborhood: Vec<(i32, i32)> = Vec::with_capacity(max_neighborhood_count);

    for nx in xfrom..xto {
        for ny in yfrom..yto {
            if !moore && (nx - x).abs() + (ny - y).abs() > radius {
                continue;
            }
            neighborhood.push((nx, ny));
        }
    }

    neighborhood
}

/// A Python module implemented in Rust.
#[pymodule]
fn mesa(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_neighborhood_hashmap, m)?)?;
    m.add_function(wrap_pyfunction!(get_neighborhood_vector, m)?)?;
    m.add_function(wrap_pyfunction!(get_neighborhood_old_method, m)?)?;
    Ok(())
}
