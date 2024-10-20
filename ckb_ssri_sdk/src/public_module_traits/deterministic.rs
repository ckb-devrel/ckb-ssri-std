use ckb_std::ckb_types::packed::{CellDep, CellInput, HeaderView, RawTransaction};

use crate::SSRIError;

pub trait Deterministic {
    type Recipe;
    fn get_output(
        &self,
        recipe: Self::Recipe,
        input: Vec<CellInput>,
        cell_dep: Vec<CellDep>,
        header_dep: Vec<HeaderView>,
    ) -> Result<RawTransaction, SSRIError>;
}
