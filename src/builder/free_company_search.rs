use crate::{
  LodestoneScraper,
  error::*,
  util::{Either, AsLodestone},
};

use ffxiv_types::{World, DataCenter};

use lodestone_parser::models::{
  GrandCompany,
  search::{
    Paginated,
    free_company::FreeCompanySearchItem,
  },
};

use url::Url;

#[derive(Debug)]
pub struct FreeCompanySearchBuilder<'a> {
  scraper: &'a LodestoneScraper,
  // q
  name: Option<&'a str>,
  // worldname
  world: Option<Either<World, DataCenter>>,
  // gcid
  grand_company: Option<Vec<GrandCompany>>,
  // page
  page: Option<u64>,
}

impl<'a> FreeCompanySearchBuilder<'a> {
  pub fn new(scraper: &'a LodestoneScraper) -> Self {
    FreeCompanySearchBuilder {
      scraper,
      name: None,
      world: None,
      grand_company: None,
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

  pub fn grand_company(&mut self, gc: GrandCompany) -> &mut Self {
    self.grand_company.get_or_insert_with(Default::default).push(gc);
    self
  }

  pub fn page(&mut self, p: u64) -> &mut Self {
    self.page = Some(p);
    self
  }

  pub async fn send(&self) -> Result<Paginated<FreeCompanySearchItem>> {
    let text = self.scraper.text(self.as_url()).await?;
    lodestone_parser::parse_free_company_search(&text).map_err(Error::Parse)
  }

  pub fn as_url(&self) -> Url {
    let mut url = crate::LODESTONE_URL.join("freecompany/").unwrap();

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

      if let Some(ref gcs) = self.grand_company {
        for gc in gcs {
          pairs.append_pair("gcid", &gc.as_lodestone().to_string());
        }
      }
    }

    url
  }
}
