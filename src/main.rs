use menu::menu;
use mt::TuringMachine;
use tape::Tape;
use text_io::read;

use crate::factory::Factory;

mod factory;
mod menu;
mod mt;
mod tape;
mod transitions;
fn main() {
    let mut turing_machine: TuringMachine;
    let mut tape: Tape;
    menu();
    let mut option: u8 = read!();
    let mut path: String = String::from("mt_basica.txt");
    // create default mt
    turing_machine = create_mt(path);

    while option != 3 {
        match option {
            1 => {
                println!("Digite o nome do arquivo: ");
                path = read!();
                turing_machine = create_mt(path);
            }
            2 => {
                println!("Digite uma cadeia:");
                let word: String = read!();
                tape = Tape::new(
                    word,
                    turing_machine.white_symbol,
                    turing_machine.tape_alphabet.clone(),
                    turing_machine.initial_state.clone(),
                );

                println!("---------------------------");
                loop {
                    match turing_machine.get_transition(&tape.state, tape.get_current_symbol()) {
                        Some(transition) => {
                            let is_walkable = tape.move_on_tape(
                                transition.direction.clone(),
                                transition.symbol.clone(),
                            );

                            tape.state = transition.state.clone();

                            if !is_walkable {
                                is_mt_acceptable(&turing_machine, &tape);
                                break;
                            }
                        }
                        None => {
                            is_mt_acceptable(&turing_machine, &tape);
                            break;
                        }
                    }
                }
            }
            _ => {
                println!("Fim");
                break;
            }
        }
        println!("Digite uma opção novamente: ");
        option = read!();
    }
}

fn create_mt(path: String) -> TuringMachine {
    match Factory::new(path) {
        Ok(mt) => mt,
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn is_mt_acceptable(turing_machine: &TuringMachine, tape: &Tape) -> () {
    if turing_machine.is_acceptable(&tape.state) {
        println!("CADEIA ACEITA!")
    } else {
        println!("CADEIA REJEITADA!")
    }
}
