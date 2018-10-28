use crate::{
  LodestoneScraper,
  error::*,
  util::{Either, AsLodestone},
};

use ffxiv_types::{World, DataCenter};

use lodestone_parser::models::search::{
  Paginated,
  linkshell::LinkshellSearchItem,
};

use url::Url;

#[derive(Debug)]
pub struct LinkshellSearchBuilder<'a> {
  scraper: &'a LodestoneScraper,
  // q
  name: Option<&'a str>,
  // worldname
  world: Option<Either<World, DataCenter>>,
  // FIXME: add active members
  // page
  page: Option<u64>,
}

impl<'a> LinkshellSearchBuilder<'a> {
  pub fn new(scraper: &'a LodestoneScraper) -> Self {
    LinkshellSearchBuilder {
      scraper,
      name: None,
      world: None,
      page: None,
    }
  }

  pub fn name(&mut self, n: &'a str) -> &mut Self {
    self.name = Some(n);
    self
  }

  pub fn world(&mut self, w: World) -> &mut Self {
    self.world = Some(Either::Left(w));
    self
  }

  pub fn data_center(&mut self, dc: DataCenter) -> &mut Self {
    self.world = Some(Either::Right(dc));
    self
  }

  pub fn page(&mut self, p: u64) -> &mut Self {
    self.page = Some(p);
    self
  }

  pub fn send(&self) -> Result<Paginated<LinkshellSearchItem>> {
    let text = self.scraper.text(self.as_url())?;
    lodestone_parser::parse_linkshell_search(&text).map_err(Error::Parse)
  }

  pub fn as_url(&self) -> Url {
    let mut url = crate::LODESTONE_URL.join("linkshell/").unwrap();

    {
      let mut pairs = url.query_pairs_mut();

      if let Some(page) = self.page {
        pairs.append_pair("page", &page.to_string());
      }

      if let Some(ref name) = self.name {
        pairs.append_pair("q", name);
      }

      match self.world {
        Some(Either::Left(w)) => { pairs.append_pair("worldname", w.as_str()); },
        Some(Either::Right(dc)) => { pairs.append_pair("worldname", &dc.as_lodestone()); },
        _ => {},
      }
    }

    url
  }
}
