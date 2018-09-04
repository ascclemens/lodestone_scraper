use crate::models::ClassJobRole;

use ffxiv_types::{DataCenter, Race, Clan};

use lodestone_parser::models::{
  GrandCompany,
  character::Job,
};

crate trait AsLodestone {
  type Representation;

  fn as_lodestone(&self) -> Self::Representation;
}

impl AsLodestone for DataCenter {
  type Representation = String;

  fn as_lodestone(&self) -> Self::Representation {
    format!("_dc_{}", self.as_str())
  }
}

impl AsLodestone for ClassJobRole {
  type Representation = &'static str;

  fn as_lodestone(&self) -> Self::Representation {
    match *self {
      ClassJobRole::ClassTank => "_class_TANK",
      ClassJobRole::ClassHealer => "_class_HEALER",
      ClassJobRole::ClassDps => "_class_DPS",
      ClassJobRole::JobTank => "_job_TANK",
      ClassJobRole::JobHealer => "_job_HEALER",
      ClassJobRole::JobDps => "_job_DPS",
      ClassJobRole::ClassDiscipleOfTheHand => "_class_CRAFTER",
      ClassJobRole::ClassDiscipleOfTheLand => "_class_GATHERER",
    }
  }
}

impl AsLodestone for Job {
  type Representation = u8;

  fn as_lodestone(&self) -> Self::Representation {
    match *self {
      Job::Gladiator => 1,
      Job::Pugilist => 2,
      Job::Marauder => 3,
      Job::Lancer => 4,
      Job::Archer => 5,
      Job::Conjurer => 6,
      Job::Thaumaturge => 7,
      Job::Carpenter => 8,
      Job::Blacksmith => 9,
      Job::Armorer => 10,
      Job::Goldsmith => 11,
      Job::Leatherworker => 12,
      Job::Weaver => 13,
      Job::Alchemist => 14,
      Job::Culinarian => 15,
      Job::Miner => 16,
      Job::Botanist => 17,
      Job::Fisher => 18,
      Job::Paladin => 19,
      Job::Monk => 20,
      Job::Warrior => 21,
      Job::Dragoon => 22,
      Job::Bard => 23,
      Job::WhiteMage => 24,
      Job::BlackMage => 25,
      Job::Arcanist => 26,
      Job::Summoner => 27,
      Job::Scholar => 28,
      Job::Rogue => 29,
      Job::Ninja => 30,
      Job::Machinist => 31,
      Job::DarkKnight => 32,
      Job::Astrologian => 33,
      Job::Samurai => 34,
      Job::RedMage => 35,
    }
  }
}

impl AsLodestone for GrandCompany {
  type Representation = u8;

  fn as_lodestone(&self) -> Self::Representation {
    match *self {
      GrandCompany::Maelstrom => 1,
      GrandCompany::TwinAdders => 2,
      GrandCompany::Flames => 3,
    }
  }
}

impl AsLodestone for Race {
  type Representation = u8;

  fn as_lodestone(&self) -> Self::Representation {
    match *self {
      Race::Hyur => 1,
      Race::Elezen => 2,
      Race::Lalafell => 3,
      Race::Miqote => 4,
      Race::Roegadyn => 5,
      Race::AuRa => 6,
    }
  }
}

impl AsLodestone for Clan {
  type Representation = u8;

  fn as_lodestone(&self) -> Self::Representation {
    match *self {
      Clan::Midlander => 1,
      Clan::Highlander => 2,
      Clan::Wildwood => 3,
      Clan::Duskwight => 4,
      Clan::Plainsfolk => 5,
      Clan::Dunesfolk => 6,
      Clan::SeekerOfTheSun => 7,
      Clan::KeeperOfTheMoon => 8,
      Clan::SeaWolf => 9,
      Clan::Hellsguard => 10,
      Clan::Raen => 11,
      Clan::Xaela => 12,
    }
  }
}
