mod cli;
use cli::CliError;
use std::env;
fn main() {
    // The first element is traditionally the path of the executable,
    // but it can be set to arbitrary text, and might not even exist.
    //  This means this property should not be relied upon for security purposes.
    let arg = env::args().skip(1).next();

    match arg {
        Some(a) => match cli::get_command(&a) {
            Ok(a) => cli::run(a),
            Err(CliError::ComandoNaoReconhecido) => {
                println!("Comando desconhecido");
            }
        },
        None => println!("help"),
    }
}
