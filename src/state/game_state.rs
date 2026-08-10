use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameStat {
  #[default]
  Loading,
  Playing,
  Paused,
}