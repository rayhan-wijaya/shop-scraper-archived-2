use crate::models::Product;
use scraper::Html;
use std::fmt;

pub struct Tokopedia;
pub struct Shopee;
pub struct Blibli;
pub struct Bukalapak;

pub trait MarketplaceScraper {
    fn parse_document(search_query: String) -> Result<Html, ParseDocumentError>;
    fn get_products(search_query: String) -> Vec<Product>;
}

pub enum ScrapingError {
    GetResponseError,
    ResponseTextError,
}

impl fmt::Display for ScrapingError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            ScrapingError::GetResponseError => "Failed to get a response",
            ScrapingError::ResponseTextError => "Failed to get text out of response",
        };

        return write!(formatter, "{}", message);
    }
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
