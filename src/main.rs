use std::io;
use std::process::ExitCode;

use clap::Parser;
use stop_words::LANGUAGE;

use rs_stop_words_print::keys2stdout;
use rs_stop_words_print::lang2words2stdout;
use rs_stop_words_print::str2lang;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Print supported languages
    #[arg(long)]
    print_supported_languages: bool,

    /// Language to print stop words for
    #[arg(long)]
    language: Option<String>,
}

enum Task {
    ListLanguages,
    ListStopWords(&'static LANGUAGE),
}

impl Task {
    fn list_languages(&self) -> Result<(), io::Error> {
        keys2stdout()
    }

    fn list_stop_words(lang: &LANGUAGE) -> Result<(), io::Error> {
        lang2words2stdout(lang)
    }

    fn do_task(&self) -> Result<(), io::Error> {
        match self {
            Self::ListLanguages => self.list_languages(),
            Self::ListStopWords(lng) => Self::list_stop_words(lng),
        }
    }
}

fn lng2task(lng: &str) -> Result<Task, io::Error> {
    let olang: Option<_> = str2lang(lng);
    let lang: &LANGUAGE =
        olang.ok_or_else(|| io::Error::other(format!("invalid language: {lng}")))?;
    Ok(Task::ListStopWords(lang))
}

fn sub() -> Result<(), io::Error> {
    let args = Args::parse();

    if args.print_supported_languages {
        return Task::ListLanguages.do_task();
    }

    if let Some(lng) = args.language {
        return lng2task(&lng)?.do_task();
    }

    use clap::CommandFactory;
    Args::command().print_help()?;
    println!();
    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
