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
            ScrapingError::ParseElementError { text } => &format!("Failed to parse a dom element's text node, {}", text),
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
    fn from<'a, T>(url: T) -> Result<String, ScrapingError<'a>>
    where
        T: IntoUrl
    {
        return reqwest::blocking::get(url)
            .map_err(|_| ScrapingError::GetResponseError { url })?
            .text()
            .map_err(|_| ScrapingError::ResponseTextError);
    }
}

struct DomSelector;

impl DomSelector {
    fn parse<'a>(selectors: &str) -> Result<Selector, ScrapingError<'_>> {
        return Selector::parse(selectors)
            .map_err(|_| ScrapingError::ParseSelectorError);
    }
}

struct DomNode;

impl DomNode {
    fn from_selector<'a>(selector: &Selector, parent_element: ElementRef<'a>) -> Result<ElementRef<'a>, ScrapingError<'a>> {
        return parent_element
            .select(selector)
            .next().ok_or(ScrapingError::MissingElementError);
    }

    fn get_first_text(parent_element: ElementRef) -> Result<&str, ScrapingError<'_>> {
        return parent_element
            .text()
            .next().ok_or(ScrapingError::MissingElementError);
    }
}

impl MarketplaceScraper for Tokopedia {
    fn parse_document<'a>(search_query: String) -> Result<Html, ScrapingError<'a>> {
        let url = format!("https://tokopedia.com/search?q={}", search_query);

        let response_text = ResponseText::from(&url)?;
        let document = Html::parse_document(&response_text);

        return Ok(document);
    }

    fn get_cheap_products<'a>(search_query: String) -> Result<Vec<Product>, ScrapingError<'a>> {
        let document = Self::parse_document(search_query)?;
        let mut products: Vec<Product> = Vec::new();

        let product_selector = DomSelector::parse(r#"div.pcv3__container"#)?;
        let name_selector = DomSelector::parse(r#"div.prd_link-product-name"#)?;
        let price_selector = DomSelector::parse(r#"div.prd_link-product-price"#)?;
        let rating_selector = DomSelector::parse(r#"span.prd_rating-average-text"#)?;
        let image_selector = DomSelector::parse(r#"img.css-1q90pod"#)?;
        let link_selector = DomSelector::parse(r#"a.pcv3__info-content"#)?;

        return Ok(products);
    }
}

impl MarketplaceScraper for Shopee {
    fn parse_document<'a>(search_query: String) -> Result<Html, ScrapingError<'a>> {
        let url = format!("https://shopee.com/search?keyword={}", search_query);

        let response_text = ResponseText::from(&url)?;
        let document = Html::parse_document(&response_text);

        return Ok(document);
    }

    fn get_cheap_products<'a>(search_query: String) -> Result<Vec<Product>, ScrapingError<'a>> {
        let document = Self::parse_document(search_query)?;

        todo!()
    }
}

impl MarketplaceScraper for Blibli {
    fn parse_document<'a>(search_query: String) -> Result<Html, ScrapingError<'a>> {
        todo!()
    }

    fn get_cheap_products<'a>(search_query: String) -> Result<Vec<Product>, ScrapingError<'a>> {
        let document = Self::parse_document(search_query)?;

        todo!()
    }
}

impl MarketplaceScraper for Bukalapak {
    fn parse_document<'a>(search_query: String) -> Result<Html, ScrapingError<'a>> {
        todo!()
    }

    fn get_cheap_products<'a>(search_query: String) -> Result<Vec<Product>, ScrapingError<'a>> {
        let document = Self::parse_document(search_query)?;

        todo!()
    }
}
