use std::{fmt, io::Cursor};

use bytes::Bytes;
use reqwest::blocking::Client;
use rodio::Decoder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Dataset {
    source: Source,
    count: usize,
    symbols: Vec<IPASymbol>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Source {
    file: String,
    pages: usize,
    note: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct IPASymbol {
    section: String,
    section_label: String,
    page: usize,
    name: String,
    ipa_name: Option<String>,
    ipa_number: Option<String>,
    unicode_name: Option<String>,
    unicode_range: Option<String>,
    hex_value: String,
    codepoints: Vec<String>,
    symbol: String,
    display_symbol: String,
    tipa_code: Option<String>,
    afii_code: Option<String>,
    audio: Option<Audio>,
    phonetics: Phonetics,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Audio {
    source: String,
    mapping_source: String,
    commons_file: String,
    page_url: String,
    fetch_url: String,
    metadata_url: String,
}

impl Audio {
    fn fetch(&self, client: &Client) -> reqwest::Result<Bytes> {
        client
            .get(&self.fetch_url)
            .send()?
            .error_for_status()?
            .bytes()
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Phonetics {
    class: String,
    consonant: Option<Consonant>,
    vowel: Option<Vowel>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Consonant {
    airstream: AirStream,
    voicing: Option<Voicing>,
    place: Option<Place>,
    manner: Manner,
    lateral: bool,
    secondary_articulations: Vec<SecondaryArticulation>,
    chart: ConsonantChart,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum AirStream {
    PulmonicEgressive,
    GlottalicEgressive,
    GlottalicIngressive,
    LingualIngressive,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Voicing {
    Voiced,
    Voiceless,
}

impl Voicing {
    fn as_str(&self) -> &'static str {
        match self {
            Voicing::Voiced => "voiced",
            Voicing::Voiceless => "voiceless",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Place {
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    DentalAlveolar,
    Postalveolar,
    Retroflex,
    AlveoloPalatal,
    Palatoalveolar,
    PostalveolarVelar,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Epiglottal,
    Glottal,
    LabialVelar,
    LabialPalatal,
}

impl Place {
    fn as_str(&self) -> &'static str {
        match self {
            Place::Bilabial => "bilabial",
            Place::Labiodental => "labiodental",
            Place::Dental => "dental",
            Place::Alveolar => "alveolar",
            Place::DentalAlveolar => "dental alveolar",
            Place::Postalveolar => "postalveolar",
            Place::Retroflex => "retroflex",
            Place::AlveoloPalatal => "alveolo palatal",
            Place::Palatoalveolar => "palatoalveolar",
            Place::PostalveolarVelar => "postalveolar velar",
            Place::Palatal => "palatal",
            Place::Velar => "velar",
            Place::Uvular => "uvular",
            Place::Pharyngeal => "pharyngeal",
            Place::Epiglottal => "epiglottal",
            Place::Glottal => "glottal",
            Place::LabialVelar => "labial velar",
            Place::LabialPalatal => "labial palatal",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Manner {
    Plosive,
    Nasal,
    Trill,
    Tap,
    Flap,
    Fricative,
    Approximant,
    LateralFricative,
    LateralApproximant,
    LateralFlap,
    Click,
    Implosive,
    Ejective,
    Affricate,
    FricativeOrApproximant,
}

impl fmt::Display for Manner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Manner::Plosive => "plosive",
            Manner::Nasal => "nasal",
            Manner::Trill => "trill",
            Manner::Tap => "tap",
            Manner::Flap => "flap",
            Manner::Fricative => "fricative",
            Manner::Approximant => "approximant",
            Manner::LateralFricative => "lateral fricative",
            Manner::LateralApproximant => "lateral approximant",
            Manner::LateralFlap => "lateral flap",
            Manner::Click => "click",
            Manner::Implosive => "implosive",
            Manner::Ejective => "ejective",
            Manner::Affricate => "affricate",
            Manner::FricativeOrApproximant => "fricative or approximant",
        };

        write!(f, "{value}")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SecondaryArticulation {
    Labialized,
    Palatalized,
    Velarized,
    Pharyngealized,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ConsonantChart {
    chart_id: Option<String>,
    row: Option<Manner>,
    column: Option<Place>,
    pair_position: Option<ConsonantPairPosition>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum ConsonantPairPosition {
    Voiced,
    Voiceless,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Vowel {
    height: Height,
    backness: Backness,
    roundness: Option<Roundness>,
    rhotic: bool,
    chart: VowelChart,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct VowelChart {
    chart_id: Option<String>,
    row: Option<String>,
    column: Option<String>,
    pair_position: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Height {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

impl fmt::Display for Height {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Height::Close => "close",
            Height::NearClose => "near-close",
            Height::CloseMid => "close-mid",
            Height::Mid => "mid",
            Height::OpenMid => "open-mid",
            Height::NearOpen => "near-open",
            Height::Open => "open",
        };

        fmt.write_str(value)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Backness {
    Front,
    NearFront,
    Central,
    NearBack,
    Back,
}

impl fmt::Display for Backness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Backness::Front => "front",
            Backness::NearFront => "near-front",
            Backness::Central => "central",
            Backness::NearBack => "near-back",
            Backness::Back => "back",
        };

        f.write_str(value)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Roundness {
    Rounded,
    Unrounded,
}

impl fmt::Display for Roundness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Roundness {
    fn as_str(&self) -> &'static str {
        match self {
            Roundness::Rounded => "rounded",
            Roundness::Unrounded => "unrounded",
        }
    }
}

fn play(b: Bytes) -> Result<(), Box<dyn std::error::Error>> {
    let mut sink = rodio::DeviceSinkBuilder::open_default_sink()?;
    sink.log_on_drop(false);
    let player = rodio::Player::connect_new(&sink.mixer());
    let source = Decoder::try_from(Cursor::new(b))?;
    player.append(source);
    player.sleep_until_end();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw = std::fs::read_to_string("src/data/ipa_symbols.json")?;
    let dataset: Dataset = serde_json::from_str(&raw)?;

    println!(
        "{} IPA entries from {}\n",
        dataset.count, dataset.source.file
    );

    for s in dataset.symbols.iter() {
        match s.phonetics.class.as_str() {
            "consonant" => {
                if let Some(c) = &s.phonetics.consonant {
                    if let Some(audio) = &s.audio {
                        println!(
                            "[Consonant] {} = {} / {} / {} ({})",
                            s.display_symbol,
                            c.voicing.as_ref().map(Voicing::as_str).unwrap_or("x"),
                            c.place.as_ref().map(Place::as_str).unwrap_or("x"),
                            c.manner,
                            audio.fetch_url
                        );
                    } else {
                        println!(
                            "[Consonant] {} = {} / {} / {}",
                            s.display_symbol,
                            c.voicing.as_ref().map(Voicing::as_str).unwrap_or("x"),
                            c.place.as_ref().map(Place::as_str).unwrap_or("x"),
                            c.manner,
                        );
                    }
                }
            }
            "vowel" => {
                if let Some(v) = &s.phonetics.vowel {
                    if let Some(audio) = &s.audio {
                        println!(
                            "[vowel] {} = {} / {} / {} ({})",
                            s.display_symbol,
                            v.height,
                            v.backness,
                            v.roundness.as_ref().map(Roundness::as_str).unwrap_or("x"),
                            audio.fetch_url
                        );
                    } else {
                        println!(
                            "[vowel] {} = {} / {} / {}",
                            s.display_symbol,
                            v.height,
                            v.backness,
                            v.roundness.as_ref().map(Roundness::as_str).unwrap_or("x"),
                        );
                    }
                }
            }
            other => {
                println!("[{}] {}", other, s.display_symbol)
            }
        }
    }

    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    let client = Client::builder().user_agent(APP_USER_AGENT).build()?;

    if let Some(symbol) = dataset.symbols.iter().find(|s| s.symbol == "p") {
        if let Some(audio) = &symbol.audio {
            let b = audio.fetch(&client)?;
            play(b)?;
        }
    }

    Ok(())
}
