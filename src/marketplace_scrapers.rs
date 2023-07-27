use crate::models::Product;
use reqwest::IntoUrl;
use scraper::{Html, Selector, ElementRef};
use std::fmt;

pub struct Tokopedia;
pub struct Shopee;
pub struct Blibli;
pub struct Bukalapak;

pub trait MarketplaceScraper {
    fn parse_document(search_query: String) -> Result<Html, ScrapingError>;
    fn get_cheap_products(search_query: String) -> Result<Vec<Product>, ScrapingError>;
}

pub enum ScrapingError {
    GetResponseError,
    ResponseTextError,
    ParseSelectorError,
    MissingElementError,
    ParseElementError,
}

struct ResponseText;

impl ResponseText {
    fn from<T>(url: T) -> Result<String, ScrapingError>
    where
        T: IntoUrl
    {
        return reqwest::blocking::get(url)
            .map_err(|_| ScrapingError::GetResponseError)?
            .text()
            .map_err(|_| ScrapingError::ResponseTextError);
    }
}

struct ScrapingSelector;

impl ScrapingSelector {
    fn parse(selectors: &str) -> Result<Selector, ScrapingError> {
        return Selector::parse(selectors)
            .map_err(|_| ScrapingError::ParseSelectorError);
    }
}

struct DomNode;

impl DomNode {
    fn from_selector<'a>(selector: &Selector, parent_element: ElementRef<'a>) -> Result<ElementRef<'a>, ScrapingError> {
        return parent_element
            .select(selector)
            .next().ok_or(ScrapingError::MissingElementError);
    }

    fn get_first_text<F>(parent_element: ElementRef) -> Result<F, ScrapingError>
    where
        F: std::str::FromStr
    {
        return parent_element
            .text()
            .next().ok_or(ScrapingError::MissingElementError)?
            .parse::<F>().map_err(|_| ScrapingError::ParseElementError);
    }
}

impl fmt::Display for ScrapingError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            ScrapingError::GetResponseError => "Failed to get a response",
            ScrapingError::ResponseTextError => "Failed to get text out of response",
            ScrapingError::ParseSelectorError => "Failed to parse a dom selector",
            ScrapingError::MissingElementError => "A dom element wasn't found",
            ScrapingError::ParseElementError => "Failed to parse a dom element's text node",
        };

        return write!(formatter, "{}", message);
    }
}

impl MarketplaceScraper for Tokopedia {
    fn parse_document(search_query: String) -> Result<Html, ScrapingError> {
        let url = format!("https://tokopedia.com/search?q={}", search_query);

        let response_text = ResponseText::from(&url)?;
        let document = Html::parse_document(&response_text);

        return Ok(document);
    }

    fn get_cheap_products(search_query: String) -> Result<Vec<Product>, ScrapingError> {
        let document = Self::parse_document(search_query)?;

        todo!()
    }
}

impl MarketplaceScraper for Shopee {
    fn parse_document(search_query: String) -> Result<Html, ScrapingError> {
        let url = format!("https://shopee.com/search?keyword={}", search_query);

        let response_text = ResponseText::from(&url)?;
        let document = Html::parse_document(&response_text);

        return Ok(document);
    }

    fn get_cheap_products(search_query: String) -> Result<Vec<Product>, ScrapingError> {
        let document = Self::parse_document(search_query)?;

        todo!()
    }
}

impl MarketplaceScraper for Blibli {
    fn parse_document(search_query: String) -> Result<Html, ScrapingError> {
        todo!()
    }

    fn get_cheap_products(search_query: String) -> Result<Vec<Product>, ScrapingError> {
        let document = Self::parse_document(search_query)?;

        todo!()
    }
}

impl MarketplaceScraper for Bukalapak {
    fn parse_document(search_query: String) -> Result<Html, ScrapingError> {
        todo!()
    }

    fn get_cheap_products(search_query: String) -> Result<Vec<Product>, ScrapingError> {
        let document = Self::parse_document(search_query)?;

        todo!()
    }
}
