use crate::models::Product;
use reqwest::IntoUrl;
use scraper::{Html, Selector, ElementRef};
use std::{fmt::Display, error::Error as StdError};

pub trait MarketplaceScraper {
    fn parse_document(search_query: String) -> Result<Html, ScrapingError>;
    fn get_cheap_products(search_query: String) -> Result<Vec<Product>, ScrapingError>;
}

pub struct Tokopedia;
pub struct Shopee;
pub struct Blibli;
pub struct Bukalapak;

#[derive(Debug)]
pub enum ScrapingError {
    GetResponseError(reqwest::Error),
    ResponseTextError(reqwest::Error),
    ParseSelectorError,
    MissingElementError,
    ParseElementError,
}

impl ScrapingError {
    fn message(&self) -> String {
        return match self {
            ScrapingError::GetResponseError(error) => format!("Failed to get a response at {:?}", error.url()),
            ScrapingError::ResponseTextError(error) => format!("Failed to get text out of response at {:?}", error.url()),
            ScrapingError::ParseSelectorError => String::from("Failed to parse a dom selector"),
            ScrapingError::MissingElementError => String::from("A dom element wasn't found"),
            ScrapingError::ParseElementError => String::from("Failed to parse a dom element's text node"),
        };
    }
}

impl std::error::Error for ScrapingError<'_> {
    fn description(&self) -> &str {
        return self.message();
    }
}

impl Display for ScrapingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self.message());
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

        for product_element in document.select(&product_selector) {
            let link_element = DomNode::from_selector(&link_selector, product_element)?;
            let url = link_element.value().attr("href").map(|link_url| String::from(link_url));

            if let Some(url) = url {
                if url.starts_with("https://ta.tokopedia.com") {
                    continue;
                }

                let name_element = DomNode::from_selector(&name_selector, product_element)?;
                let name = DomNode::get_first_text(name_element)?.to_string();

                let price_element = DomNode::from_selector(&price_selector, product_element)?;
                let price_in_idr_text = DomNode::get_first_text(price_element)?
                    .replace("Rp", "")
                    .replace(".", "");
                let price_in_idr = price_in_idr_text
                    .parse::<f32>()
                    .map_err(|_| ScrapingError::ParseElementError { text: price_in_idr_text })?;

                let rating_element = DomNode::from_selector(&rating_selector, product_element).ok();
                let rating = rating_element
                    .and_then(|rating_element| DomNode::get_first_text(rating_element).ok())
                    .and_then(|rating_text| rating_text.parse::<f32>().ok());

                let image_element = DomNode::from_selector(&image_selector, product_element)?;
                let image_url = image_element
                    .value()
                    .attr("src")
                    .map(|image_url| image_url.to_string());

                let product = Product {
                    id: 69420,
                    name,
                    rating,
                    image_url,
                    price_in_idr,
                    url,
                };

                products.push(product);
            }
        }

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
