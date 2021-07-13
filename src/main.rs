use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        std::process::exit(1);
    }
    let filename = &args[1];

    let mut file = File::open(filename).expect("Pas de fichier");
    let mut source = String::new();
    file.read_to_string(&mut source).expect("Erreur");

    let op = convert_to_op(source);

    let program = convert_to_instruction(op);
    interpreteur(&mut new_empty_memory(), &program);
}

#[derive(Debug, Clone)]
enum Instruction {
    Plus,
    Moins,
    Gauche,
    Droite,
    Afficher,
    Lire,
    Boucle(Vec<Instruction>),
}

#[derive(Debug)]
struct Memoire {
    donnee: Vec<i32>,
    case_courante: usize,
}

#[derive(Clone, Debug)]
enum OperCharac {
    Plus,
    Moins,
    ChevronDroit,
    ChevronGauche,
    Point,
    Virgule,
    CrochetOuvert,
    CrochetFerme,
}

fn new_memory(donnee: Vec<i32>) -> Memoire {
    Memoire {
        donnee: donnee,
        case_courante: 0,
    }
}

fn new_empty_memory() -> Memoire {
    new_memory(vec![0; 1024])
}

fn interpreteur(memoire: &mut Memoire, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        match instruction {
            Instruction::Plus => memoire.donnee[memoire.case_courante] += 1,
            Instruction::Moins => memoire.donnee[memoire.case_courante] -= 1,
            Instruction::Droite => {
                if memoire.case_courante + 1 == memoire.donnee.len() {
                    memoire.donnee.push(0);
                } else {
                    memoire.case_courante += 1;
                }
            }
            Instruction::Gauche => {
                if memoire.case_courante == 0 {
                    memoire.case_courante = memoire.donnee.len() - 1;
                } else {
                    memoire.case_courante -= 1;
                }
            }
            Instruction::Afficher => print!(
                "{}",
                char::from_u32(memoire.donnee[memoire.case_courante] as u32).unwrap_or('?')
            ),
            Instruction::Lire => {
                let mut buf = [0, 1];
                let _ = std::io::stdin().read(&mut buf);
                memoire.donnee[memoire.case_courante] = buf[0] as i32
            }
            Instruction::Boucle(instruc_boucle) => {
                while memoire.donnee[memoire.case_courante] != 0 {
                    interpreteur(memoire, instruc_boucle)
                }
            }
        }
    }
}

fn convert_to_op(source: String) -> Vec<OperCharac> {
    let mut operations = Vec::new();

    for symbol in source.chars() {
        let op = match symbol {
            '+' => Some(OperCharac::Plus),
            '-' => Some(OperCharac::Moins),
            '>' => Some(OperCharac::ChevronDroit),
            '<' => Some(OperCharac::ChevronGauche),
            '.' => Some(OperCharac::Point),
            ',' => Some(OperCharac::Virgule),
            '[' => Some(OperCharac::CrochetOuvert),
            ']' => Some(OperCharac::CrochetFerme),
            _ => None,
        };

        // Non-opcode characters are simply comments
        match op {
            Some(op) => operations.push(op),
            None => (),
        }
    }

    operations
}

fn convert_to_instruction(opcodes: Vec<OperCharac>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, op) in opcodes.iter().enumerate() {
        if loop_stack == 0 {
            let instr = match op {
                OperCharac::Plus => Some(Instruction::Plus),
                OperCharac::Moins => Some(Instruction::Moins),
                OperCharac::ChevronDroit => Some(Instruction::Droite),
                OperCharac::ChevronGauche => Some(Instruction::Gauche),
                OperCharac::Point => Some(Instruction::Afficher),
                OperCharac::Virgule => Some(Instruction::Lire),

                OperCharac::CrochetOuvert => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                }

                OperCharac::CrochetFerme => panic!("la loop n'a pas commencé #{}", i),
            };

            match instr {
                Some(instr) => program.push(instr),
                None => (),
            }
        } else {
            match op {
                OperCharac::CrochetOuvert => {
                    loop_stack += 1;
                }
                OperCharac::CrochetFerme => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Boucle(convert_to_instruction(
                            opcodes[loop_start + 1..i].to_vec(),
                        )));
                    }
                }
                _ => (),
            }
        }
    }
    program
}
