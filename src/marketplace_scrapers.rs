use crate::models::Product;
use scraper::Html;

pub struct Tokopedia;
pub struct Shopee;
pub struct Blibli;
pub struct Bukalapak;

pub trait MarketplaceScraper {
    fn parse_document(search_query: String) -> Result<Html, ParseDocumentError>;
    fn get_products(search_query: String) -> Vec<Product>;
}

#[derive(Debug)]
pub enum ScrapingError {
    GetResponseError,
    ResponseTextError,
}

impl MarketplaceScraper for Tokopedia {
    fn parse_document(search_query: String) -> Result<Html, ParseDocumentError> {
        let url = format!("https://tokopedia.com/search?q={}", search_query);

        let response_text = reqwest::blocking::get(url)
            .map_err(|_| ParseDocumentError::GetResponseError)?
            .text()
            .map_err(|_| ParseDocumentError::ResponseTextError)?;

        return Ok(Html::parse_document(&response_text));
    }

    fn get_products(search_query: String) -> Vec<Product> {
        let document = Self::parse_document(search_query);

        todo!()
    }
}

impl MarketplaceScraper for Shopee {
    fn parse_document(search_query: String) -> Result<Html, ParseDocumentError> {
        todo!()
    }

    fn get_products(search_query: String) -> Vec<Product> {
        let document = Self::parse_document(search_query);

        todo!()
    }
}

impl MarketplaceScraper for Blibli {
    fn parse_document(search_query: String) -> Result<Html, ParseDocumentError> {
        todo!()
    }

    fn get_products(search_query: String) -> Vec<Product> {
        let document = Self::parse_document(search_query);

        todo!()
    }
}

impl MarketplaceScraper for Bukalapak {
    fn parse_document(search_query: String) -> Result<Html, ParseDocumentError> {
        todo!()
    }

    fn get_products(search_query: String) -> Vec<Product> {
        let document = Self::parse_document(search_query);

        todo!()
    }
}
