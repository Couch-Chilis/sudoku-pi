use super::{Cell, Game, Notes, Sudoku};
use anyhow::{anyhow, Context};
use serde::de::{self, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fmt, fs};

impl Game {
    /// Loads an existing game from disk, or returns `Self::default()` if no
    /// game could be loaded.
    pub fn load() -> Self {
        fs::read(get_sudoku_path())
            .context("Can't read file")
            .and_then(|json| Self::from_json(&json))
            .map_err(|err| println!("Can't restore Sudoku game: {err}"))
            .unwrap_or_default()
    }

    /// Saves the game to disk.
    ///
    /// This is called automatically on drop.
    fn save(&self) {
        let path = get_sudoku_path();
        if self.current.is_solved() {
            if path.exists() {
                fs::remove_file(path)
                    .unwrap_or_else(|err| println!("Can't clean up Sudoku game: {err}"));
            }
        } else {
            self.to_json()
                .and_then(|json| fs::write(path, json).context("Can't write to file"))
                .unwrap_or_else(|err| println!("Can't save Sudoku game: {err}"));
        }
    }

    /// Serializes the game to JSON, omitting its solution.
    fn to_json(&self) -> Result<Vec<u8>, anyhow::Error> {
        serde_json::to_vec(&SerializedGame::from(self)).map_err(anyhow::Error::from)
    }

    /// Parses the game from JSON, verifying there is only a single solution.
    fn from_json(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        serde_json::from_slice(bytes)
            .map_err(anyhow::Error::from)
            .and_then(|serialized_game| {
                let SerializedGame {
                    start,
                    current,
                    notes,
                } = serialized_game;
                match start.find_unique_solution() {
                    Some(solution) => Ok(Game {
                        start,
                        current,
                        solution,
                        notes,
                    }),
                    None => Err(anyhow!("Saved game didn't have a unique solution")),
                }
            })
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.save()
    }
}

/// The serialized game state, without its solution.
#[derive(Serialize, Deserialize)]
pub struct SerializedGame {
    pub start: Sudoku,
    pub current: Sudoku,
    pub notes: Notes,
}

impl From<&Game> for SerializedGame {
    fn from(game: &Game) -> Self {
        Self {
            start: game.start.clone(),
            current: game.current.clone(),
            notes: game.notes.clone(),
        }
    }
}

impl<'de> Deserialize<'de> for Sudoku {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(SudokuVisitor)
    }
}

struct SudokuVisitor;

impl<'de> Visitor<'de> for SudokuVisitor {
    type Value = Sudoku;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "an array with 81 optional integers from 1 through 9"
        )
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let vec: Vec<Cell> = (0..81)
            .map(|_| {
                let cell: Cell = seq.next_element()?.unwrap_or_default();
                if let Some(n) = cell {
                    if n.get() > 9 {
                        return Err(de::Error::invalid_value(
                            de::Unexpected::Unsigned(n.get().into()),
                            &self,
                        ));
                    }
                }
                Ok(cell)
            })
            .collect::<Result<_, _>>()?;
        if vec.len() != 81 {
            return Err(de::Error::invalid_length(81, &self));
        }

        Ok(Sudoku {
            cells: vec.try_into().unwrap_or([None; 81]),
        })
    }
}

impl Serialize for Sudoku {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(81))?;
        for cell in self.cells {
            seq.serialize_element(&cell)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Notes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(NotesVisitor)
    }
}

struct NotesVisitor;

impl<'de> Visitor<'de> for NotesVisitor {
    type Value = Notes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an array with 81 integers")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let vec: Vec<u16> = (0..81)
            .map(|_| Ok(seq.next_element()?.unwrap_or_default()))
            .collect::<Result<_, _>>()?;

        Ok(Notes {
            cells: vec.try_into().unwrap_or([0; 81]),
        })
    }
}

impl Serialize for Notes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(81))?;
        for cell in self.cells {
            seq.serialize_element(&cell)?;
        }
        seq.end()
    }
}

fn get_sudoku_path() -> PathBuf {
    #[allow(deprecated)]
    std::env::home_dir()
        .map(|path| path.join(".sudoku.json"))
        .unwrap_or(PathBuf::from("/tmp/sudoku.json"))
}