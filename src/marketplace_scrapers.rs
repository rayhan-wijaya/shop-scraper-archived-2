use crate::models::Product;
use scraper::Html;

pub struct Tokopedia;
pub struct Shopee;
pub struct Blibli;
pub struct Bukalapak;

pub trait MarketplaceScraper {
    fn parse_document(search_query: String) -> Html;
    fn get_products(search_query: String) -> Vec<Product>;
}

impl MarketplaceScraper for Tokopedia {
    fn parse_document(search_query: String) -> Html {
        todo!()
    }

    fn get_products(search_query: String) -> Vec<Product> {
        let document = Self::parse_document(search_query);

        todo!()
    }
}

impl MarketplaceScraper for Shopee {
    fn parse_document(search_query: String) -> Html {
        todo!()
    }

    fn get_products(search_query: String) -> Vec<Product> {
        let document = Self::parse_document(search_query);

        todo!()
    }
}

impl MarketplaceScraper for Blibli {
    fn parse_document(search_query: String) -> Html {
        todo!()
    }

    fn get_products(search_query: String) -> Vec<Product> {
        let document = Self::parse_document(search_query);

        todo!()
    }
}

impl MarketplaceScraper for Bukalapak {
    fn parse_document(search_query: String) -> Html {
        todo!()
    }

    fn get_products(search_query: String) -> Vec<Product> {
        let document = Self::parse_document(search_query);

        todo!()
    }
}
