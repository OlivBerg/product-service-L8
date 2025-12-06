use serde::{Deserialize, Serialize};
use crate::localwasmtime::WasmProduct;

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub description: String,
    pub image: String,
    pub rating: f32,
    pub reviews: i32,
    pub discount: f32,
}

#[derive(Deserialize)]
pub struct ProductInfo {
    pub product_id: i32,
}


impl Into<WasmProduct> for Product {
    fn into(self) -> WasmProduct {
        WasmProduct {
            id: self.id,
            name: self.name,
            description: self.description,
            price: self.price,
            image: self.image,
            rating: self.rating,
            reviews: self.reviews,
            discount: self.discount,
        }
    }
}

impl From<WasmProduct> for Product {
    fn from(product: WasmProduct) -> Self {
        Self {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            image: product.image,
            rating: product.rating,
            reviews: product.reviews,
            discount: product.discount,
        }
    }
}
