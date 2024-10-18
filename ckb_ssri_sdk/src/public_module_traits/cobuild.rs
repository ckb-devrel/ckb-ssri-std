pub trait CoBuild {
  type Recipe;
  fn parse_cobuild(&self) -> Self::Recipe;
}