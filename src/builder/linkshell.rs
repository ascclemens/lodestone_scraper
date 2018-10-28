use crate::{
  LodestoneScraper,
  error::*,
};

use ffxiv_types::{World, DataCenter};

use lodestone_parser::models::linkshell::Linkshell;

use url::Url;

#[derive(Debug)]
pub struct LinkshellBuilder<'a> {
  scraper: &'a LodestoneScraper,
  id: u64,
  // page
  page: Option<u64>,
}

impl<'a> LinkshellBuilder<'a> {
  pub fn new(scraper: &'a LodestoneScraper, id: u64) -> Self {
    LinkshellBuilder {
      scraper,
      id,
      page: None,
    }
  }

  pub fn page(&mut self, p: u64) -> &mut Self {
    self.page = Some(p);
    self
  }

  pub fn send(&self) -> Result<Linkshell> {
    let text = self.scraper.text(self.as_url())?;
    lodestone_parser::parse_linkshell(self.id, &text).map_err(Error::Parse)
  }

  pub fn as_url(&self) -> Url {
    let mut url = LodestoneScraper::route(&format!("linkshell/{}", self.id)).unwrap();

    {
      let mut pairs = url.query_pairs_mut();

      if let Some(page) = self.page {
        pairs.append_pair("page", &page.to_string());
      }
    }

    url
  }
}
