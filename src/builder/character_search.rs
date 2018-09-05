use crate::{
  LodestoneScraper,
  error::*,
  models::ClassJobRole,
  util::{Either, AsLodestone},
};

use ffxiv_types::{World, Race, Clan, DataCenter};

use lodestone_parser::models::{
  GrandCompany,
  character::Job,
  search::{
    Paginated,
    character::CharacterSearchItem,
  },
};

use url::Url;

#[derive(Debug)]
pub struct CharacterSearchBuilder<'a> {
  scraper: &'a LodestoneScraper,
  // q
  name: Option<&'a str>,
  // worldname
  world: Option<Either<World, DataCenter>>,
  // classjob
  job: Option<Either<Job, ClassJobRole>>,
  // race_tribe
  race: Option<Either<Race, Clan>>,
  // gcid
  grand_company: Option<Vec<GrandCompany>>,
  // page
  page: Option<u64>,
}

impl<'a> CharacterSearchBuilder<'a> {
  pub fn new(scraper: &'a LodestoneScraper) -> Self {
    CharacterSearchBuilder {
      scraper,
      name: None,
      world: None,
      job: None,
      race: None,
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

  pub fn job(&mut self, j: Job) -> &mut Self {
    self.job = Some(Either::Left(j));
    self
  }

  pub fn role(&mut self, r: ClassJobRole) -> &mut Self {
    self.job = Some(Either::Right(r));
    self
  }

  pub fn race(&mut self, r: Race) -> &mut Self {
    self.race = Some(Either::Left(r));
    self
  }

  pub fn clan(&mut self, c: Clan) -> &mut Self {
    self.race = Some(Either::Right(c));
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

  pub fn send(&self) -> Result<Paginated<CharacterSearchItem>> {
    let text = self.scraper.text(self.as_url())?;
    lodestone_parser::parse_character_search(&text).map_err(Error::Parse)
  }

  pub fn as_url(&self) -> Url {
    let mut url = crate::LODESTONE_URL.join("character/").unwrap();

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

      match self.job {
        Some(Either::Left(j)) => { pairs.append_pair("classjob", &j.as_lodestone().to_string()); },
        Some(Either::Right(cjr)) => { pairs.append_pair("classjob", cjr.as_lodestone()); },
        _ => {},
      }

      match self.race {
        Some(Either::Left(r)) => { pairs.append_pair("race_tribe", &format!("race_{}", r.as_lodestone())); },
        Some(Either::Right(c)) => { pairs.append_pair("race_tribe", &format!("tribe_{}", c.as_lodestone())); },
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
