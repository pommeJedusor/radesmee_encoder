use std::env;
pub mod radesmee_encoding;

const RADESMEE_CODE_TO_TEXT: &str = "decrypt";
const TEXT_TO_RADESMEE_CODE: &str = "crypt";

fn show_help(program_name: &str) {
    println!(
        "some basic command examples:\n{} {} :radaesmee::radaesmee::radesmee::radesmee::radesmee::radaesmee::radesmee::radaesmee::radaesmee::radaesmee::radesmee::radesmee::radaesmee::radaesmee::radesmee::radesmee:\n{} {} :3\n\nyou can use the --help or -h to get this message",
        program_name, RADESMEE_CODE_TO_TEXT, program_name, TEXT_TO_RADESMEE_CODE,
    );
}

fn main() {
    let text_starting_index = 2;

    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];

    // if not sub command in second place nor help command
    if args.len() <= 1
        || (&args[1] != RADESMEE_CODE_TO_TEXT
            && &args[1] != TEXT_TO_RADESMEE_CODE
            && &args[1] != "-h"
            && &args[1] != "--help")
    {
        println!(
            "the comamnd must include one of those two sub commands {}, {}\n",
            RADESMEE_CODE_TO_TEXT, TEXT_TO_RADESMEE_CODE,
        );
        show_help(program_name);
        return;
    }

    let subcommand = &args[1];

    if subcommand == "-h" || subcommand == "--help" {
        show_help(program_name);
        return;
    }

    if args.len() < text_starting_index + 1 {
        println!("the comamnd must include something to decrypt/crypt\n");
        show_help(program_name);
        return;
    }

    // decrypt
    if subcommand == RADESMEE_CODE_TO_TEXT {
        let decrypted_message =
            match radesmee_encoding::radesmee_to_text(&args[text_starting_index..].join(" ")) {
                Ok(x) => x,
                Err(x) => x,
            };
        println!("{}", decrypted_message);
        // crypt
    } else {
        let crypted_message =
            radesmee_encoding::text_to_radesmee(&args[text_starting_index..].join(" "));
        println!("{}", crypted_message);
    }
}
