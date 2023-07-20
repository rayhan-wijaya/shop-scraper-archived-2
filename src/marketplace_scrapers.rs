use crate::models::Product;

pub trait MarketplaceScraper {
    fn get_products() -> Vec<Product>;
}

pub struct Tokopedia;
pub struct Shopee;
pub struct Blibli;
pub struct Bukalapak;

impl MarketplaceScraper for Tokopedia {
    fn get_products() -> Vec<Product> {
        todo!()
    }
}

impl MarketplaceScraper for Shopee {
    fn get_products() -> Vec<Product> {
        todo!()
    }
}

impl MarketplaceScraper for Blibli {
    fn get_products() -> Vec<Product> {
        todo!()
    }
}

impl MarketplaceScraper for Bukalapak {
    fn get_products() -> Vec<Product> {
        todo!()
    }
}
