pub trait Deterministic: CoBuild {
  fn get_output(
      &self,
      recipe: Self::Recipe,
      input: Vec<CellInput>,
      cell_dep: Vec<CellDep>,
      header_dep: Vec<HeaderView>,
  ) -> Result<Transaction, SSRIError>;
}
