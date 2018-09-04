# lodestone_scraper

A Lodestone client library.

```rust
use ffxiv_types::World;
use lodestone_scraper::LodestoneScraper;

fn main() {
  let ls = LodestoneScraper::default();

  let search = ls.character_search()
    .name("Duvivi Duvi")
    .world(World::Adamantoise)
    .send()
    .unwrap();
  let character = ls.character(search.results[0].id).unwrap();

  assert_eq!("Duvivi Duvi", character.name);
  assert_eq!(World::Adamantoise, character.world);
}
```
