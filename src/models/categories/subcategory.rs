use super::category::Category;

#[derive(Debug)]
pub struct Subcategory{
    id: u32,
    name: String,
    category: Category,
}