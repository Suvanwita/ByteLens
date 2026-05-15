use goblin::Object;
use clap::Parser;

#[derive(Parser)]
pub struct Dummy;

pub fn analyze_binary(bytes: &[u8], args: &crate::Args) {
    match Object::parse(bytes) {
        Ok(Object::Elf(elf)) => {
            println!("Binary Type: ELF");
            println!("Architecture: {:?}", elf.header.e_machine);

            if args.sections {
                println!("\n=== SECTIONS ===");

                for section in &elf.section_headers {
                    let name = elf.shdr_strtab.get_at(section.sh_name)
                        .unwrap_or("UNKNOWN");

                    println!(
                        "{} | addr: 0x{:x} | size: {}",
                        name,
                        section.sh_addr,
                        section.sh_size
                    );
                }
            }

            if args.imports {
                println!("\n=== IMPORTS ===");

                for lib in elf.libraries {
                    println!("{lib}");
                }
            }
        }

        Ok(Object::PE(pe)) => {
            println!("Binary Type: PE");
            println!("Entry Point: 0x{:x}", pe.entry);

            if args.sections {
                println!("\n=== SECTIONS ===");

                for section in &pe.sections {
                    let name = String::from_utf8_lossy(&section.name);

                    println!(
                        "{} | virtual_size: {}",
                        name.trim_matches(char::from(0)),
                        section.virtual_size
                    );
                }
            }

            if args.imports {
                println!("\n=== IMPORTS ===");

                for import in pe.imports {
                    println!("{}", import.name);
                }
            }
        }

        Ok(Object::Mach(_)) => {
            println!("Binary Type: Mach-O");
        }

        _ => {
            println!("Unknown binary format");
        }
    }
}