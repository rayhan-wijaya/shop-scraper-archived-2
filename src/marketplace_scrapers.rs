use crate::models::Product;
use reqwest::IntoUrl;
use scraper::{Html, Selector, ElementRef};

pub trait MarketplaceScraper {
    fn parse_document<'a>(search_query: String) -> Result<Html, ScrapingError<'a>>;
    fn get_cheap_products<'a>(search_query: String) -> Result<Vec<Product>, ScrapingError<'a>>;
}

pub struct Tokopedia;
pub struct Shopee;
pub struct Blibli;
pub struct Bukalapak;

pub enum ScrapingError<'a> {
    GetResponseError { url: &'a str },
    ResponseTextError { url: &'a str },
    ParseSelectorError { selectors: &'a str },
    MissingElementError,
    ParseElementError { text: &'a str },
}

impl ScrapingError<'_> {
    fn message(&self) -> &str {
        let message = match self {
            ScrapingError::GetResponseError { url } => &format!("Failed to get a response at {}", url),
            ScrapingError::ResponseTextError { url } => &format!("Failed to get text out of response at {}", url),
            ScrapingError::ParseSelectorError { selectors } => &format!("Failed to parse a dom selector, {}", selectors),
            ScrapingError::MissingElementError => &"A dom element wasn't found",
            ScrapingError::ParseElementError => &format!("Failed to parse a dom element's text node"),
        };

        return message;
    }
}

impl std::error::Error for ScrapingError<'_> {
    fn description(&self) -> &str {
        return self.message();
    }
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

struct DomSelector;

impl DomSelector {
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

    fn get_first_text(parent_element: ElementRef) -> Result<&str, ScrapingError> {
        return parent_element
            .text()
            .next().ok_or(ScrapingError::MissingElementError);
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
