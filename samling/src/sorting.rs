pub(crate) trait Sortable {
    type Type;
    fn sort(&self, values: Vec<Self::Type>) -> Vec<Self::Type>;
}
