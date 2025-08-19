mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let rect_area = areas_volumes::rectangle_area(x, y) as f64;
    let shape_area;

    match kind {
        areas_volumes::GeometricalShapes::Circle => shape_area = areas_volumes::circle_area(a),
        areas_volumes::GeometricalShapes::Rectangle => {
            shape_area = areas_volumes::rectangle_area(a, b) as f64
        }
        areas_volumes::GeometricalShapes::Square => {
            shape_area = areas_volumes::square_area(a) as f64
        }
        areas_volumes::GeometricalShapes::Triangle => {
            shape_area = areas_volumes::triangle_area(a, b)
        }
    }

    println!(
        "times: {}, rect / shape: {}",
        times,
        (rect_area / shape_area)
    );

    shape_area == 0.0 || (rect_area / shape_area) >= times as f64
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let box_volume = (x * y * z) as f64;
    let shape_volume;

    match kind {
        areas_volumes::GeometricalVolumes::Cone => shape_volume = areas_volumes::cone_volume(a, b),
        areas_volumes::GeometricalVolumes::Cube => {
            shape_volume = areas_volumes::cube_volume(a) as f64
        }
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            shape_volume = areas_volumes::parallelepiped_volume(a, b, c) as f64
        }
        areas_volumes::GeometricalVolumes::TriangularPyramid => {
            shape_volume = areas_volumes::triangular_pyramid_volume(a as f64, b)
        }
        areas_volumes::GeometricalVolumes::Sphere => shape_volume = areas_volumes::sphere_volume(a),
    }

    shape_volume == 0.0 || (box_volume / shape_volume) >= times as f64
}

#[cfg(test)]
mod tests;
