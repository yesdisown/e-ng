use clap::Parser;
use e_ng::macros::GeneratorMode;
use e_ng::{ObfuscateConfig, obfuscate};
use std::fs;
use std::io::{self, IsTerminal, Read, Write};
use std::path::PathBuf;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

/// Builds the colored ASCII banner with a red `E` block.
fn banner() -> String {
    format!(
        "\
{RED}======={RESET}
{RED}======={RESET}
{RED}======={RESET}
{RED}======={RESET}
{RED}==   =={RESET}           == =    ===  {RESET}
{RED}=  =  ={RESET} ========  =====  == == {RESET}
{RED}=     ={RESET}           == ==   ==== {RESET}
{RED}=  ===={RESET}           == ==     == {RESET}
{RED}=  =  ={RESET}           == ==  == == {RESET}
{RED}==   =={RESET}           == ==   ===  {RESET}
{RED}======={RESET}                        "
    )
}

/// Prints the ASCII banner to stderr.
fn print_banner() {
    eprintln!("{}\n", banner());
}

#[derive(Parser, Debug)]
#[command(
    name = "e-ng",
    version,
    about = "The successor to BrutBurger's Project E. A C++ obfuscator.",
    before_help = banner()
)]
struct Args {
    /// Input C++ file (reads from stdin if omitted).
    input: Option<PathBuf>,

    /// Output C++ file (writes to stdout if omitted).
    output: Option<PathBuf>,

    /// Set of characters to use ("ae" by default).
    #[arg(
        short,
        long,
        alias = "chars",
        default_value = "ae",
        conflicts_with = "phraseset"
    )]
    charset: String,

    /// Path to file containing the phrases to use, one phrase per line.
    #[arg(short, long, alias = "phrases", conflicts_with = "charset")]
    phraseset: Option<PathBuf>,

    /// Do not shuffle the generated `#define` lines.
    #[arg(long, default_value_t = false)]
    no_shuffle: bool,

    /// Print status info to stderr.
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn load_phrases(path: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().map(str::to_string).collect())
}

fn read_input(input: &Option<PathBuf>) -> io::Result<String> {
    match input {
        Some(path) => fs::read_to_string(path),
        None => {
            let stdin = io::stdin();
            if stdin.is_terminal() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "No input file and nothing piped to stdin.",
                ));
            }
            let mut buf = String::new();
            stdin.lock().read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}

fn write_output(output: &Option<PathBuf>, content: &str) -> io::Result<()> {
    match output {
        Some(path) => fs::write(path, content),
        None => {
            let mut stdout = io::stdout().lock();
            stdout.write_all(content.as_bytes())?;
            stdout.flush()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Always show the banner on run, including when `-v` / `--verbose` is passed.
    print_banner();

    let source = read_input(&args.input)?;

    let mode = match &args.phraseset {
        Some(path) => {
            let phrases = load_phrases(path)?;
            GeneratorMode::from_phrases(phrases)?
        }
        None => GeneratorMode::from_charset(&args.charset)?,
    };

    let config = ObfuscateConfig {
        mode,
        shuffle: !args.no_shuffle,
    };

    let result = obfuscate(&source, &config);
    write_output(&args.output, &result)?;

    if args.verbose {
        let src = args
            .input
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "stdin".to_string());
        let dst = args
            .output
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "stdout".to_string());
        eprintln!("Obfuscated {src} -> {dst}");
    }

    Ok(())
}
