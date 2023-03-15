use serde::{Deserialize, Serialize};

/// Input provided by the user, to create a new property.
#[derive(Serialize, Deserialize, Clone)]
pub struct PropertyInput {
    /// Name of the property
    pub name: String,
    /// Price of the property
    pub price: Option<f64>,
    /// Number of bedrooms
    pub bedrooms: u16,
    /// Number of bathrooms
    pub bathrooms: u16,
    /// Area size
    pub sqft: f64,
    /// Address of property
    pub address: String,
    /// ID of Agent representing the property
    pub agentid: Option<i64>,
    /// Description of the property
    pub description: String,
}
