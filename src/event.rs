use crate::api_types::AttemptResult;
use std::fmt::Write;

pub enum Event {
        E333,
        E222,
        E444,
        E555,
        E666,
        E777,
        E333Blindfolded,
        E333FewestMoves,
        E333OneHanded,
        EClock,
        EMegaminx,
        EPyraminx,
        ESkewb,
        ESquare1,
        E444Blindfolded,
        E555Blindfolded,
        EMultiNew,
        E333Feet,
        EMagic,
        EMasterMagic,
        EMultiOld,
}

impl Event {
        pub fn new(id: &str) -> Option<Event> {
                use Event::*;
                Some(match id {
                        "333" => E333,
                        "222" => E222,
                        "444" => E444,
                        "555" => E555,
                        "666" => E666,
                        "777" => E777,
                        "333bf" => E333Blindfolded,
                        "333fm" => E333FewestMoves,
                        "333oh" => E333OneHanded,
                        "clock" => EClock,
                        "minx" => EMegaminx,
                        "pyram" => EPyraminx,
                        "skewb" => ESkewb,
                        "sq1" => ESquare1,
                        "444bf" => E444Blindfolded,
                        "555bf" => E555Blindfolded,
                        "333mbf" => EMultiNew,
                        "magic" => EMagic,
                        "mmagic" => EMasterMagic,
                        "333mbo" => EMultiOld,
                        _ => None?,
                })
        }

        pub fn id(&self) -> &'static str {
                use Event::*;
                match self {
                        E333 => "333",
                        E222 => "222",
                        E444 => "444",
                        E555 => "555",
                        E666 => "666",
                        E777 => "777",
                        E333Blindfolded => "333bf",
                        E333FewestMoves => "333fm",
                        E333OneHanded => "333oh",
                        EClock => "clock",
                        EMegaminx => "minx",
                        EPyraminx => "pyram",
                        ESkewb => "skewb",
                        ESquare1 => "sq1",
                        E444Blindfolded => "444bf",
                        E555Blindfolded => "555bf",
                        EMultiNew => "333mbf",
                        E333Feet => "333ft",
                        EMagic => "magic",
                        EMasterMagic => "mmagic",
                        EMultiOld => "333mbo",
                }
        }

        pub fn name(&self) -> &'static str {
                use Event::*;
                match self {
                        E333 => "3x3x3 Cube",
                        E222 => "2x2x2 Cube",
                        E444 => "4x4x4 Cube",
                        E555 => "5x5x5 Cube",
                        E666 => "6x6x6 Cube",
                        E777 => "7x7x7 Cube",
                        E333Blindfolded => "3x3x3 Blindfolded",
                        E333FewestMoves => "3x3x3 Fewest Moves",
                        E333OneHanded => "3x3x3 One-Handed",
                        EClock => "Clock",
                        EMegaminx => "Megaminx",
                        EPyraminx => "Pyraminx",
                        ESkewb => "Skewb",
                        ESquare1 => "Square-1",
                        E444Blindfolded => "4x4x4 Blindfolded",
                        E555Blindfolded => "5x5x5 Blindfolded",
                        EMultiNew => "3x3x3 Multi-Blind",
                        E333Feet => "3x3x3 With Feet",
                        EMagic => "Magic",
                        EMasterMagic => "Master Magic",
                        EMultiOld => "3x3x3 Multi-Blind Old Style",
                }
        }

        pub fn format(&self) -> Format {
                match self {
                        Event::EMultiNew => Format::MultiNew,
                        Event::EMultiOld => Format::MultiOld,
                        Event::E333FewestMoves => Format::Number,
                        _ => Format::Time,
                }
        }
}

pub enum Format {
        Time,
        Number,
        MultiNew,
        MultiOld,
}

impl Format {
        pub fn print_result(&self, result: AttemptResult) -> Option<String> {
                match result {
                        AttemptResult::Skipped => None,
                        AttemptResult::DNF => Some("DNF".to_owned()),
                        AttemptResult::DNS => Some("DNS".to_owned()),
                        AttemptResult::Ok(v) => Some(match self {
                                Format::Time => {
                                        let centiseconds = v % 100;
                                        let seconds = (v / 100) % 60;
                                        let minutes = (v / (100 * 60)) % 60;
                                        let hours = v / (100 * 60 * 60);
                                        let mut time = String::new();
                                        if hours > 0 {
                                                write!(&mut time, "{hours}:");
                                        }
                                        if !time.is_empty() {
                                                write!(&mut time, "{minutes:02}:");
                                        }
                                        if minutes > 0 && time.is_empty() {
                                                write!(&mut time, "{minutes}:");
                                        }
                                        if !time.is_empty() {
                                                write!(&mut time, "{seconds:02}.");
                                        }
                                        if time.is_empty() {
                                                write!(&mut time, "{seconds}.");
                                        }
                                        write!(&mut time, "{centiseconds:02}");
                                        time
                                },
                                Format::Number => todo!(),
                                Format::MultiNew => todo!(),
                                Format::MultiOld => todo!(),
                        }),
                }
        }
}
