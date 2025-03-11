use super::ids::HouseId;

#[derive(Debug, Clone)]
pub struct House {
    pub id: HouseId,
    pub name: String,
}
