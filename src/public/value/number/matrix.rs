use super::Number;

type MatrixRow = Vec<Number>;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct Matrix {
    rows: Vec<MatrixRow>,
}
