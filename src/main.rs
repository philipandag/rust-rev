#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::{fs, io::{self, stdin, stdout, BufRead, BufReader, Read, Write}};

fn reverse(text: &str) -> String
{
    text.chars().rev().collect()
}

fn print_rev_line(stream: &mut impl io::Write, text: &String, length: usize)
{
    let buf = text.replace("\n", "");
    let reversed = reverse(&buf);
    writeln!(stream, "{length}: {reversed}").expect("Blad  writeln");
}

fn rev(out_stream: &mut impl io::Write, in_stream: &mut impl io::Read)
{
    let mut input = String::new();
    let mut reader = BufReader::new(in_stream);
    let mut len = 1;
    while len > 0
    {
        input.clear();
        len = reader.read_line(&mut input).expect("Nie mozna odczytac pliku wejsciowego");
        input = input.replace("\n", "");
        if input.len() > 1
        {
            print_rev_line(out_stream, &input, input.len());
        }
    }
}

fn main() 
{
    let args: Vec<String> = std::env::args().collect();
    let len = args.len();
    match len
    {
        1 => { // live
            rev(&mut stdout(), &mut stdin());
        },
        2 => { // filename to stdout
            let mut file = fs::File::open(&args[1]).expect(format!("Nie mozna otworzyc pliku {}", args[1]).as_str());
            rev(&mut file, &mut stdin());
        },
        3 => {
            let mut infile = fs::File::options().read(true).open(&args[1]).expect(format!("Nie mozna otworzyc pliku {}", args[1]).as_str());
            let mut outfile = fs::File::options().write(true).open(&args[2]).expect(format!("Nie mozna otworzyc pliku {}", args[2]).as_str());
            rev(&mut outfile, &mut infile);
        },
        _ => {
            println!("usage:\n\rrev [in_file] [out_file]");
        },
    }
}



#[cfg(test)]
mod test {
    use std::{fs, io::Write};
    use super::*;

    #[test]
    fn reverses_oneline() {

        //initialize 
        let inpath = "rev_test_reverses_oneline_infile.txt";
        let outpath = "rev_test_reverses_oneline_outfile.txt";
        let mut infile = fs::File::options().write(true).create(true).open(inpath).expect("Cant create in file");
        let mut outfile = fs::File::options().write(true).create(true).open(outpath).expect("Cant create out file");

        let test_string = "Ala ma kota!, Kot Ma Ale!".to_string();
        let correct = "25: !elA aM toK ,!atok am alA\n";

        infile.write(test_string.as_bytes()).unwrap();
        let mut infile = fs::File::options().read(true).open(inpath).expect("Cant open in file");

        // run
        rev(&mut outfile, &mut infile);

        //test
        let mut outfile = fs::File::options().read(true).open(outpath).expect("Cant open out file");
        let mut read_string = String::new();
        let len = outfile.read_to_string(&mut read_string).expect("Nie mozna odczytac z pliku wyjsciowego");
        println!("Odczytano {len} znakow");
        assert_eq!(read_string, correct);

        //clean up
        let _ =  fs::remove_file(inpath);
        let _ = fs::remove_file(outpath);

    }

    #[test]
    fn reverses_multiline() {
        
        //initialize
        let inpath = "rev_test_reverses_multiline_infile.txt";
        let outpath = "rev_test_reverses_multiline_outfile.txt";
        let mut infile = fs::File::options().write(true).create(true).open(inpath).expect("Cant create in file");
        let mut outfile = fs::File::options().write(true).create(true).open(outpath).expect("Cant create out file");

        let test_string = "Ala ma kota!\n Kot Ma\n Ale!".to_string();
        let correct = "12: !atok am alA\n7: aM toK \n5: !elA \n".to_string();

        infile.write(test_string.as_bytes()).unwrap();
        let mut infile = fs::File::options().read(true).open(inpath).expect("Cant open in file");

        //run
        rev(&mut outfile, &mut infile);

        //test
        let mut outfile = fs::File::options().read(true).open(outpath).expect("Cant open out file");

        let mut read_string = String::new();
        let len = outfile.read_to_string(&mut read_string).expect("Nie mozna odczytac z pliku wyjsciowego");
        println!("Odczytano {len} znakow");
        assert_eq!(read_string, correct);

        //clean up
        let _ =  fs::remove_file(inpath);
        let _ = fs::remove_file(outpath);
    }
}
