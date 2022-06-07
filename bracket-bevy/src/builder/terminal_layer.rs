use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum TerminalLayer {
    Simple {
        font_index: usize,
        width: usize,
        height: usize,
        features: HashSet<SimpleConsoleFeatures>,
    },
    Sparse {
        font_index: usize,
        width: usize,
        height: usize,
    },
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SimpleConsoleFeatures {
    WithoutBackground,
    NoDirtyOptimization,
}
