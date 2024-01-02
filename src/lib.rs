mod settings;

pub use pyo3::prelude::*;
pub use settings::Settings;

/// converts svgbob ascii art to svg string.
#[pyfunction]
pub fn to_svg(ascii: &str) -> String {
    svgbob::to_svg(ascii)
}

/// convert svgbob ascii art to svg compressed string.
#[pyfunction]
pub fn to_svg_string_compressed(ascii: &str) -> String {
    svgbob::to_svg_string_compressed(ascii)
}

/// convert svgbob ascii art to svg string with indention.
#[pyfunction]
pub fn to_svg_string_pretty(ascii: &str) -> String {
    svgbob::to_svg_string_pretty(ascii)
}

/// convert ascii art to svg using the size supplied.
#[pyfunction]
pub fn to_svg_with_override_size(ascii: &str, settings: Settings, w: f32, h: f32) -> String {
    svgbob::to_svg_with_override_size(ascii, &settings.into(), w, h)
}

/// convert ascii art into an svg.
#[pyfunction]
pub fn to_svg_with_settings(ascii: &str, settings: Settings) -> String {
    svgbob::to_svg_with_settings(ascii, &settings.into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn vye_svgbob(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Settings>()?;
    m.add_function(wrap_pyfunction!(to_svg, m)?)?;
    m.add_function(wrap_pyfunction!(to_svg_string_compressed, m)?)?;
    m.add_function(wrap_pyfunction!(to_svg_string_pretty, m)?)?;
    m.add_function(wrap_pyfunction!(to_svg_with_override_size, m)?)?;
    m.add_function(wrap_pyfunction!(to_svg_with_settings, m)?)?;
    Ok(())
}
