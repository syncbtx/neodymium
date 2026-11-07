pub trait Sanitize {
    fn sanitize(&mut self);
}

// pub trait ConfigValidate {
//     fn validate(&self) -> Result<(), Vec<ValidationError>>;
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum ValidationError {
//     SectionEmptyModules {
//         edge: Edge,
//         section_idx: usize,
//     },
//     PaddingExceedsThickness {
//         edge: Edge,
//         total_padding: u32,
//         thickness: u32,
//     },
//     InvalidColor,
// }

// impl std::fmt::Display for ValidationError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::SectionEmptyModules { edge, section_idx } => {
//                 write!(
//                     f,
//                     "{edge:?}.sections[{section_idx}] must contain at least one module"
//                 )
//             }
//             Self::PaddingExceedsThickness {
//                 edge,
//                 total_padding,
//                 thickness,
//             } => {
//                 write!(
//                     f,
//                     "{edge:?}.padding ({total_padding}px) exceeds thickness ({thickness}px)"
//                 )
//             }
//             Self::InvalidColor => {
//                 write!(f, "Invalid Color")
//             }
//         }
//     }
// }

// impl std::error::Error for ValidationError {}
