use crate::domain::bars::Edge;

pub trait ConfigValidate {
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    OpacityOutOfRange {
        edge: Edge,
        section_idx: Option<usize>,
        actual: u32,
    },
    SectionEmptyModules {
        edge: Edge,
        section_idx: usize,
    },
    PaddingExceedsThickness {
        edge: Edge,
        total_padding: u32,
        thickness: u32,
    },
    InvalidColor,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpacityOutOfRange {
                edge,
                section_idx,
                actual,
            } => match section_idx {
                Some(idx) => write!(
                    f,
                    "{edge:?}.sections[{idx}].opacity must be 0..=100, got {actual}"
                ),
                None => write!(f, "{edge:?}.opacity must be 0..=100, got {actual}"),
            },
            Self::SectionEmptyModules { edge, section_idx } => {
                write!(
                    f,
                    "{edge:?}.sections[{section_idx}] must contain at least one module"
                )
            }
            Self::PaddingExceedsThickness {
                edge,
                total_padding,
                thickness,
            } => {
                write!(
                    f,
                    "{edge:?}.padding ({total_padding}px) exceeds thickness ({thickness}px)"
                )
            }
            Self::InvalidColor => {
                write!(f, "Invalid Color")
            }
        }
    }
}

impl std::error::Error for ValidationError {}
