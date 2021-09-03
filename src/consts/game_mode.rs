﻿///////////////////////////////////////////////
//                                           //
//                     !                     //
//   This file is automatically generated!   //
//           Do not directly edit!           //
//                                           //
///////////////////////////////////////////////

use strum_macros::{ EnumString, Display, AsRefStr, IntoStaticStr };

/// League of Legends game mode, such as Classic,
/// ARAM, URF, One For All, Ascension, etc.
#[cfg_attr(feature = "nightly", non_exhaustive)]
#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash)]
#[derive(EnumString, Display, AsRefStr, IntoStaticStr)]
#[repr(u8)]
pub enum GameMode {
    /// ARAM games
    ARAM,
    /// All Random Summoner's Rift games
    ARSR,
    /// Ascension games
    ASCENSION,
    /// Blood Hunt Assassin games
    ASSASSINATE,
    /// Classic Summoner's Rift and Twisted Treeline games
    CLASSIC,
    /// Dark Star: Singularity games
    DARKSTAR,
    /// Doom Bot games
    DOOMBOTSTEEMO,
    /// Snowdown Showdown games
    FIRSTBLOOD,
    /// Nexus Blitz games, deprecated in patch 9.2 in favor of gameMode NEXUSBLITZ.
    GAMEMODEX,
    /// Legend of the Poro King games
    KINGPORO,
    /// Nexus Blitz games.
    NEXUSBLITZ,
    /// Dominion/Crystal Scar games
    ODIN,
    /// Odyssey: Extraction games
    ODYSSEY,
    /// One for All games
    ONEFORALL,
    /// PROJECT: Hunters games
    PROJECT,
    /// Nexus Siege games
    SIEGE,
    /// Star Guardian Invasion games
    STARGUARDIAN,
    /// Tutorial games
    TUTORIAL,
    /// Tutorial: Welcome to League.
    TUTORIAL_MODULE_1,
    /// Tutorial: Power Up.
    TUTORIAL_MODULE_2,
    /// Tutorial: Shop for Gear.
    TUTORIAL_MODULE_3,
    /// Ultimate Spellbook games.
    ULTBOOK,
    /// URF games
    URF,
}

serde_string!(GameMode);
