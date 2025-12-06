use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Dell Inspiron 15 15.6 Touchscreen Laptop (Intel Core i5 1334U/8GB RAM/512GB SSD/Windows 11)".to_string(),
            price: 499.99,
            description: "Enjoy seamless computing wherever you go with the Dell Inspiron 15 15.6-inch touchscreen laptop. Powered by an Intel Core i5 1334U processor and 8GB RAM, it delivers responsive performance and smooth multitasking. The lift hinge ensures an ergonomic typing angle, while the built-in HD webcam lets you enjoy high-quality video chats.".to_string(),
            image: "/dell-laptop.png".to_string(),
            rating: 4.5,
            reviews: 499,
            discount: 100.0,
        },
        Product {
            id: 2,
            name: "Apple MacBook Air 13.6-inch Laptop (M2 Chip/8GB RAM/256GB SSD/Gold)".to_string(),
            price: 999.99,
            description: "Experience the power of the Apple M2 chip with the MacBook Air 13.6-inch laptop. Featuring an 8-core CPU and 10-core GPU, this laptop delivers exceptional performance for everyday tasks and creative projects. The stunning Liquid Retina display offers vibrant colors and sharp details, while the Magic Keyboard provides a comfortable typing experience.".to_string(),
            image: "/macbook-air.png".to_string(),
            rating: 4.8,
            reviews: 1200,
            discount: 150.0,
        },
        Product{
            id: 3,
            name: "Acer Aspire Lite 15.6\" Laptop - Silver (Intel N150/8GB RAM/512GB SSD/Windows 11)".to_string(),
            price: 349.99,
            description: "Experience seamless computing with the Acer Aspire Lite laptop. It features an Intel N150 processor and 8GB DDR5 RAM for effortless multitasking. Enjoy vibrant clarity on the 15.6-inch LED screen, while 512GB solid state drive enables fast data access. It comes preloaded with Windows 11 for smooth user performance. The nine-hour battery life ensures lasting productivity.".to_string(),
            image: "/acer-aspire-lite.png".to_string(),
            rating: 4.8,
            reviews: 5,
            discount: 200.0,
        },
        Product{
            id: 4,
            name: "HP 15 Laptop, 11th Gen Intel Core i3-1115G4, 8 GB RAM, 256 GB SSD, 15.6\" HD Display, Windows 11 Home".to_string(),
            price: 429.99,
            description: "The HP 15 Laptop is designed to keep you productive and entertained. Powered by an 11th Gen Intel Core i3-1115G4 processor and 8 GB RAM, it delivers smooth performance for everyday tasks. The 256 GB SSD provides fast boot times and ample storage for your files. Enjoy clear visuals on the 15.6-inch HD display, perfect for streaming and browsing.".to_string(),
            image: "/hp-15-laptop.png".to_string(),
            rating: 4.3,
            reviews: 250,
            discount: 80.0,
        },
        Product{
            id: 5,
            name: "Lenovo IdeaPad 3 Laptop, 15.6\" HD Display, AMD Ryzen 5 5500U, 8GB RAM, 256GB SSD, Windows 11".to_string(),
            price: 459.99,
            description: "The Lenovo IdeaPad 3 Laptop combines performance and portability. Equipped with an AMD Ryzen 5 5500U processor and 8GB RAM, it handles multitasking with ease. The 15.6-inch HD display offers vibrant visuals for work and entertainment. With a 256GB SSD, enjoy quick access to your files and applications. Preloaded with Windows 11 for a user-friendly experience.".to_string(),
            image: "/lenovo-ideapad-3.png".to_string(),
            rating: 4.6,
            reviews: 300,
            discount: 120.0,
        },
        
    ]
}