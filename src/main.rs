use urlencoding::{encode, decode};

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() >= 3 {
    let url = args[2..].join(" ");

    match args[1].as_str() {
      "encode" => {
        let encoded = encode(url.as_str());
        println!("{}", encoded);
      }

      "decode" => {
        let decoded = decode(url.as_str()).expect("UTF-8");
        println!("{}", decoded);
      }

      _ => {
        eprintln!("Unknown operation {}", args[1].as_str());
        std::process::exit(1);
      }
    }
  } else {
    eprintln!("Usage: url-encode [encode | decode] (URL)");
    std::process::exit(1);
  }
}
