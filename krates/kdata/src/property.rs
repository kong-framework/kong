use serde::{Deserialize, Serialize};

/// Data type to represent a physical property such as a house.
#[derive(Serialize, Deserialize, Clone)]
pub struct Property {
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
    /// Number of online views the property got
    pub online_views: u64,
    /// Number of physical irl views the property got
    pub physical_view: u16,
    /// Number of likes for the property
    pub likes: u64,
    /// Number of bookmarks for the property
    pub bookmarks: u64,
    /// Contains a JSON string with paths to the actual images
    pub photos: String,
    /// Date when the property was added
    pub added: String,
}
