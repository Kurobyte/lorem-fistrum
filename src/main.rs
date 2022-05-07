use rand::{Rng, distributions::Uniform};
use clap::{Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    linies: Option<u32>,
    #[clap(short, long, default_value = "fistrum")]
    tipus: String,
}

fn genera_random(init: u32, end: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let range: Uniform<u32>;

    if end >= init { range = Uniform::new(init, end); }
    else { range = Uniform::new(init - end, end + init); }

    rng.sample(range)
}

fn genera_linea(llista: &Vec<&str>, final_linea: bool) -> String {
    let num_paraules = genera_random(4, 14);
    let mut p_anterior: u32 = 0;
    let mut paraula: u32 = genera_random(0, llista.len() as u32);
    let mut frase: String = llista.get(paraula as usize).unwrap().to_string();

    for _ in 0..num_paraules {
        paraula = genera_random(0, llista.len() as u32);
        if p_anterior == paraula { paraula = genera_random(0, llista.len() as u32); }
        // println!("{:}", paraula);
        // println!("{:?}", llista.get(paraula as usize).unwrap());
        frase = format!("{:} {:}", frase, llista.get(paraula as usize).unwrap());
        p_anterior = paraula;
    }

    if final_linea { frase = format!("{:}.", frase); }

    frase
}

fn genera_parraf(llista: &Vec<&str>, lin: u32) -> String {
    let linies = if lin == 0 { 1 } else { lin };
    let mut parraf = "".to_string();
    // println!("{:?}", linies);

    for l in 0..linies {
        let nova = genera_linea(&llista, l == (linies -1));
        // println!("{:?}", nova);
        if l == 0 { parraf = nova }
        else { parraf = format!("{:} {:}", parraf, nova) }
    }

    parraf
}

fn main() {
    let args = Cli::parse();
    let chiquito: Vec<&str> = vec![
        "fistro","torpedo","pecador","sexuarl","por la gloria de mi madre","diodeno","condemor","jarl","ese que llega","pupita","la caidita","te voy a borrar el cerito","al ataquerl",
        "a wan","a peich","a gramenawer","no puedor","hasta luego Lucas","mamaar","apetecan","caballo blanco caballo negroorl","ese pedazo de","benemeritaar","te va a hasé pupitaa",
        "de la pradera", "ese hombree", "quietooor", "qué dise usteer", "no te digo trigo por no llamarte Rodrigor", "está la cosa muy malar", "tiene musho peligro","ahorarr","diodenoo",
        "amatomaa","me cago en tus muelas","llevame al sircoo", "papaar papaar", "se calle ustée", "va usté muy cargadoo", "hijo míoorr", "un número nuevooorl", "le voy a jasé",
        "te da cuen", "aguaaa", "a can demor e narrr", "relájese usterl"
    ];
    let ipsum: Vec<&str> = vec![
        "sit amet", "consectetur", "adipisicing", "elit", "sed", "eiusmod", "tempor", "incididunt", "ut", "labore", "et", "dolore", "magna", "aliqua", "enim", "ad", "minim", "veniam",
        "quis", "nostrud", "exercitation", "ullamco", "laboris", "nisi", "ut", "aliquip", "ex", "commodo", "consequat", "duis", "aute", "irure", "dolor", "reprehenderit", "voluptate",
        "velit", "esse", "cillum","occaecat", "qui", "officia"
    ];

    if args.tipus == "fistrum" {
        println!("Lorem fistrum {:}", genera_parraf(&chiquito, args.linies.unwrap_or_default()));
    } else {
        println!("Lorem ipsum {:}", genera_parraf(&ipsum, args.linies.unwrap_or_default()));
    }
}
