use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung Galaxy S24 128GB - Onyx Black".to_string(),
            price: 1099.97,
            description: "Seize the moment with the Samsung Galaxy S24. Powered with Galaxy AI, the S24 adapts to your passions and favourites on your phone for a personalized mobile experience. Enjoy powerful, helpful, and useful features, like Circle to Search, Live Translate, Note Assist, and Chat Assist to help you achieve more. Built to open new possibilities, it's epic, just like that.".to_string(),
            image: "/SamsungGalaxyS24.png".to_string()
        },
        Product {
            id: 2,
            name: "Apple iPhone 16 Pro Max 256GB - Black Titanium".to_string(),
            price: 1644.99,
            description: "Profoundly powerful, Apple iPhone 16 Pro Max is built for Apple Intelligence (Apple Intelligence coming December 2024). Featuring a stunning titanium design. Camera Control. 4K 120fps Dolby Vision. And A18 Pro chip.".to_string(),
            image: "/AppleiPhone16ProMax.png".to_string()
        },
        Product {
            id: 3,
            name: "Lenovo Yoga Slim 7x 14.5 OLED Touchscreen Copilot".to_string(),
            price: 1299.99,
            description: "The Lenovo Yoga Slim 7x is not just smart — it's your creative companion. This laptop is less about the specs and more about the flex. This sleek AI powerhouse boasts a Snapdragon X Elite processor that features next-gen Neural Processing Unit (NPU) to deliver exceptional power. Make every moment of creation as lit as your ideas. Welcome to the next evolution of computing.".to_string(),
            image: "/LenovoYogaSlime.png".to_string()
        },
        Product {
            id: 4,
            name: "HP 17.3 Laptop - Natural Silver".to_string(),
            price: 849.99,
            description: "For work, play, and everything in between this HP laptop offers everything you need to get through your day with efficient performance. It features a vibrant 17.3 Full HD display for stunning clarity, a powerful Intel Core processor and 16GB of RAM, plus a 1TB SSD so you have plenty of room to store photos, documents, apps, and more.".to_string(),
            image: "/HP17.3.png".to_string()
        },
        Product {
            id: 5,
            name: "Dell Inspiron 15.6 FHD Touchscreen Laptop, Intel Core i7-1255U, 1TB SSD, 32GB RAM, Intel Iris Xe Graphics, Win11,Black".to_string(),
            price: 1149,
            description: "With its 15.6 FHD touchscreen display, enjoy crisp and vibrant visuals in stunning detail. Equipped with an Intel Core i7-1255U processor and Intel Iris Xe Graphics card, which ensures smooth multitasking and allows you to enjoy high-quality gaming and media experiences.".to_string(),
            image: "/DellInspiron15.6.png".to_string()
        },
        Product {
            id: 6,
            name: "ASUS Vivobook 16 Laptop - Indie Black (Intel Core i5-1335U/16GB RAM/1TB SSD/Win 11)".to_string(),
            price: 699.95,
            description: "The ASUS Vivobook 16 laptop combines performance and style in a sleek, lightweight design. Its 16-inch display with NanoEdge ultra-thin bezels delivers crisp, vibrant visuals, while TUV Rheinland certification for low blue light ensures eye comfort during prolonged use. This laptop features a fingerprint sensor and an HD camera with a privacy shutter for enhanced security.".to_string(),
            image: "/Asus16inch.png".to_string()
        },
        Product {
            id: 7,
            name: "Acer Aspire 3 15.6 Laptop - Silver (AMD Ryzen 5 7520U/512GB SSD/16GB RAM/Windows 11 Home)".to_string(),
            price: 599.99,
            description: "Boost your productivity with this Acer Aspire 3 15.6" laptop. This laptop features Zen 2" Core architecture, AMD Ryzen 5 7520U processor, and 16GB LPDDR5 RAM to deliver smooth performance. The 512GB solid state drive provides efficient storage space with quick read and write times. The integrated Bluetooth 5.2 allows for efficient wireless connectivity.".to_string(),
            image: "/AcerAspire3.png".to_string()
        },
        Product {
            id: 8,
            name: "Apple iMac 24 (Fall 2024) - Silver (Apple M4 Chip / 8-Core GPU / 16GB RAM / 512GB SSD)".to_string(),
            price: 1699.99,
            description: "The all-in-one desktop design is strikingly thin, comes in seven vibrant colours and elevates any space with style Apple Intelligence is the personal intelligence system that helps you write, express yourself and get things done effortlessly. With groundbreaking privacy protections, it gives you peace of mind that no one else can access your data—not even Apple (Apple Intelligence is available with Siri and device language set to U.S. English. Canadian English support available December 2024. Some features and languages, like French, will be coming over the next year.)".to_string(),
            image: "/AppleiMac24.png".to_string()
        },
        Product {
            id: 9,
            name: "TCL 98 Q6-Series 4K UHD HDR QLED Smart Google TV (98Q651G-CA)".to_string(),
            price: 2499.99,
            description: "Get outstanding picture and audio quality when you have the TCL 98” Q-Series 4K UHD HDR QLED smart Google TV. Offering outstanding value and incredible picture quality, this TV hits the sweet spot between top-notch imaging and audio quality. The TCL AIPQ processor, with Deep Learning AI, is an advanced processor that optimizes performance for top-notch viewing.".to_string(),
            image: "/TCL98.png".to_string()
        },
        Product {
            id: 10,
            name: "Apple AirPods 4 In-Ear Active Noise Cancelling True Wireless Earbuds with USB-C Charging Case".to_string(),
            price: 249.99,
            description: "Apple AirPods 4 is rebuilt for exceptional comfort and audio performance. It features Active Noise Cancellation, Adaptive Audio and Transparency mode for the first time in this design. The powerful H2 chip delivers clearer calls with Voice Isolation, the magic of Conversation Awareness and a new, hands-free way to interact with Siri. Personalized Spatial Audio with dynamic head tracking places sound all around you.".to_string(),
            image: "/AppleAIrPods4.png".to_string()
        }
    ]
}