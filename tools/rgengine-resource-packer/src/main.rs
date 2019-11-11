extern crate rgengine;
extern crate base64;

use std::env;
use std::io::{ Write };
use std::process::exit;
use std::path::{ Path, PathBuf };
use base64::{ encode };
use rgengine::resource::{ PackedStorage };
use rgengine::util::{ director, load_file };

fn ask_y_n(question: &str) -> bool {
    let mut s = "".to_owned();
    while &s == "" || &s != "y" && &s != "n" {
        print(&format!("{} (y / n): ", question));
        std::io::stdin().read_line(&mut s).ok();
        s = s.trim().to_string();
    }
    &s == "y"
}

fn print(s: &str) {
    print!("{}", s);
    std::io::stdout().flush().unwrap();
}

fn generate_encrypt_key() -> String {
    let mut keys: Vec<u8> = Vec::new();
    for _i in 0..48 {
        keys.push(director::rand());
    }
    encode(&keys)
}

fn prepare(resource_path: String, encrypt_key: Option<String>) -> (PathBuf, PathBuf, String) {
    (
        prepare_resource_path(resource_path),
        prepare_output_path("resource.dat".to_owned()),
        prepare_encrypt_key(encrypt_key)
    )
}

fn prepare_encrypt_key(encrypt_key: Option<String>) -> String {
    if let Some(k) = encrypt_key {
        k
    } else {
        let key = generate_encrypt_key();
        println!("generate encrypt key: {}", key);
        key
    }
}

fn prepare_resource_path(resource_path: String) -> PathBuf {
    let path = Path::new(&resource_path);
    if !path.exists() {
        println!("{} not exists.", path.to_str().unwrap());
        exit(1);
    }
    path.to_path_buf()
}

fn prepare_output_path(output_path: String) -> PathBuf {
    let path = Path::new(&output_path);
    if path.exists() {
        if !ask_y_n(&format!("{} is already exists. Allow overwrite this?", path.to_str().unwrap())) {
            exit(1);
        }
        std::fs::remove_file(&path).unwrap();
    }
    path.to_path_buf()
}

fn packing(storage: &PackedStorage, path: PathBuf) {
    for p in find_resource_files(path.clone()) {
        let name = p.strip_prefix(&path).unwrap().to_str().unwrap();
        let body = load_file(&p).unwrap();
        storage.save(&name, &body).unwrap();
        println!("packed: {}", p.to_str().unwrap());
    }
}

fn find_resource_files(path: PathBuf) -> Vec<PathBuf> {
    if path.is_dir() {
        let mut r: Vec<PathBuf> = Vec::new();
        for entry in std::fs::read_dir(&path).unwrap() {
            let e = entry.unwrap().path().to_path_buf();
            let mut files = find_resource_files(e);
            r.append(&mut files);
        }
        r
    } else {
        vec!(path)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 2 {
        println!("usage: ./rgengine-resource-packer path/to/resource [encrypt_key]");
        exit(1);
    }
    let (resource, output, encrypt_key) = prepare(args.get(1).cloned().unwrap(), args.get(2).cloned());
    let storage = PackedStorage::new(output, Some(encrypt_key.clone()));
    packing(&storage, resource);
    println!("packing complete!");
    println!("Please set RESOURCE_ENCRYPT_KEY={}\nThis key is used for read resource.dat and read/write save data.", encrypt_key);
}
