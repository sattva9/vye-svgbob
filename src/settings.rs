use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Settings {
    font_size: usize,
    font_family: String,
    fill_color: String,
    background: String,
    stroke_color: String,
    stroke_width: f32,
    scale: f32,
    include_backdrop: bool,
    include_styles: bool,
    include_defs: bool,
}

impl From<Settings> for svgbob::Settings {
    fn from(s: Settings) -> Self {
        svgbob::Settings {
            font_size: s.font_size,
            font_family: s.font_family,
            fill_color: s.fill_color,
            background: s.background,
            stroke_color: s.stroke_color,
            stroke_width: s.stroke_width,
            scale: s.scale,
            include_backdrop: s.include_backdrop,
            include_styles: s.include_styles,
            include_defs: s.include_defs,
        }
    }
}

#[pymethods]
impl Settings {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn init(
        font_size: Option<usize>,
        font_family: Option<String>,
        fill_color: Option<String>,
        background: Option<String>,
        stroke_color: Option<String>,
        stroke_width: Option<f32>,
        scale: Option<f32>,
        include_backdrop: Option<bool>,
        include_styles: Option<bool>,
        include_defs: Option<bool>,
    ) -> Self {
        let default = svgbob::Settings::default();
        Self {
            font_size: font_size.unwrap_or(default.font_size),
            font_family: font_family.unwrap_or(default.font_family),
            fill_color: fill_color.unwrap_or(default.fill_color),
            background: background.unwrap_or(default.background),
            stroke_color: stroke_color.unwrap_or(default.stroke_color),
            stroke_width: stroke_width.unwrap_or(default.stroke_width),
            scale: scale.unwrap_or(default.scale),
            include_backdrop: include_backdrop.unwrap_or(default.include_backdrop),
            include_styles: include_styles.unwrap_or(default.include_styles),
            include_defs: include_defs.unwrap_or(default.include_defs),
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}
