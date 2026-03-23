use std::io;

use io::BufWriter;
use io::Write;

use stop_words::LANGUAGE;

pub const fn lang2str(l: &LANGUAGE) -> &'static str {
    l.as_str()
}

pub static STR2LANG: phf::Map<&'static str, LANGUAGE> = phf::phf_map! {
    "Afrikaans" => LANGUAGE::Afrikaans,
    "Arabic" => LANGUAGE::Arabic,
    "Armenian" => LANGUAGE::Armenian,
    "Basque" => LANGUAGE::Basque,
    "Bengali" => LANGUAGE::Bengali,
    "Breton" => LANGUAGE::Breton,
    "Bulgarian" => LANGUAGE::Bulgarian,
    "Catalan" => LANGUAGE::Catalan,
    "Chinese" => LANGUAGE::Chinese,
    "Croatian" => LANGUAGE::Croatian,
    "Czech" => LANGUAGE::Czech,
    "Danish" => LANGUAGE::Danish,
    "Dutch" => LANGUAGE::Dutch,
    "English" => LANGUAGE::English,
    "Esperanto" => LANGUAGE::Esperanto,
    "Estonian" => LANGUAGE::Estonian,
    "Finnish" => LANGUAGE::Finnish,
    "French" => LANGUAGE::French,
    "Galician" => LANGUAGE::Galician,
    "German" => LANGUAGE::German,
    "Greek" => LANGUAGE::Greek,
    "Gujarati" => LANGUAGE::Gujarati,
    "Hausa" => LANGUAGE::Hausa,
    "Hebrew" => LANGUAGE::Hebrew,
    "Hindi" => LANGUAGE::Hindi,
    "Hungarian" => LANGUAGE::Hungarian,
    "Indonesian" => LANGUAGE::Indonesian,
    "Irish" => LANGUAGE::Irish,
    "Italian" => LANGUAGE::Italian,
    "Japanese" => LANGUAGE::Japanese,
    "Korean" => LANGUAGE::Korean,
    "Kurdish" => LANGUAGE::Kurdish,
    "Latin" => LANGUAGE::Latin,
    "Latvian" => LANGUAGE::Latvian,
    "Lithuanian" => LANGUAGE::Lithuanian,
    "Malay" => LANGUAGE::Malay,
    "Marathi" => LANGUAGE::Marathi,
    "Norwegian" => LANGUAGE::Norwegian,
    "Persian" => LANGUAGE::Persian,
    "Polish" => LANGUAGE::Polish,
    "Portuguese" => LANGUAGE::Portuguese,
    "Romanian" => LANGUAGE::Romanian,
    "Russian" => LANGUAGE::Russian,
    "Slovak" => LANGUAGE::Slovak,
    "Slovenian" => LANGUAGE::Slovenian,
    "Somali" => LANGUAGE::Somali,
    "Sotho" => LANGUAGE::Sotho,
    "Spanish" => LANGUAGE::Spanish,
    "Swahili" => LANGUAGE::Swahili,
    "Swedish" => LANGUAGE::Swedish,
    "Tagalog" => LANGUAGE::Tagalog,
    "Thai" => LANGUAGE::Thai,
    "Turkish" => LANGUAGE::Turkish,
    "Ukrainian" => LANGUAGE::Ukrainian,
    "Urdu" => LANGUAGE::Urdu,
    "Vietnamese" => LANGUAGE::Vietnamese,
    "Yoruba" => LANGUAGE::Yoruba,
    "Zulu" => LANGUAGE::Zulu,
};

pub fn str2lang(s: &str) -> Option<&'static LANGUAGE> {
    STR2LANG.get(s)
}

pub fn lang2words(lang: &LANGUAGE) -> &'static [&'static str] {
    stop_words::get(lang)
}

pub fn str2lang2words(lang: &str) -> &'static [&'static str] {
    str2lang(lang).map(lang2words).unwrap_or_default()
}

pub fn words2writer<W>(words: &[&str], mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    for word in words {
        writeln!(&mut wtr, "{word}")?;
    }
    wtr.flush()
}

pub fn keys2writer<W>(s2l: &phf::Map<&'static str, LANGUAGE>, mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    let keys = s2l.keys();
    for key in keys {
        writeln!(&mut wtr, "{key}")?;
    }
    wtr.flush()
}

pub fn keys2stdout() -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    keys2writer(&STR2LANG, BufWriter::new(&mut ol))?;
    ol.flush()
}

pub fn str2lang2words2stdout(lang: &str) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();

    let words: &[&str] = str2lang2words(lang);

    words2writer(words, BufWriter::new(&mut ol))?;

    ol.flush()
}

pub fn lang2words2stdout(lang: &LANGUAGE) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();

    let words: &[&str] = lang2words(lang);

    words2writer(words, BufWriter::new(&mut ol))?;

    ol.flush()
}
