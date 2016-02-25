#![cfg(feature = "codegen")]
#![feature(rustc_private)]

extern crate syntax;
extern crate aster;
extern crate elastic_codegen;
extern crate walkdir;

use std::error::Error;
use std::fs;
use std::fs::{ File, OpenOptions };
use std::io::{ Seek, SeekFrom, Write };
use syntax::ast::*;
use syntax::ext::base::ExtCtxt;
use syntax::parse::token;
use syntax::codemap::DUMMY_SP;
use syntax::parse::token::intern;
use elastic_codegen::api::ast::*;
use elastic_codegen::api::parse;
use elastic_codegen::api::gen::rust::*;
use elastic_codegen::gen::rust::*;
use elastic_codegen::emit::*;
use elastic_codegen::emit::rust::*;
use walkdir::WalkDir;

use std::env;

macro_rules! get_ctxt {
	($cx:ident, $ps:ident, $fgc:ident) => {
		$cx = ExtCtxt::new(
			&$ps, vec![],
			syntax::ext::expand::ExpansionConfig::default("qquote".to_string()),
			&mut $fgc
		);
		$cx.bt_push(syntax::codemap::ExpnInfo {
			call_site: DUMMY_SP,
			callee: syntax::codemap::NameAndSpan {
				format: syntax::codemap::MacroBang(intern("")),
				allow_internal_unstable: false,
				span: None,
			}
		});
	};
}

//TODO: Move this into its own crate (not on crates.io though)
fn main() {
    //TODO: Do this properly with getopts
    let mut args = env::args();
    let _ = args.next().unwrap();
    let indir = args.next().unwrap();
    let outdir = args.next().unwrap();
    
    println!("spec: {}", indir);
    println!("output: {}", outdir);
    
    gen_from_source(&indir, &outdir).unwrap();
}

//TODO: Split this up into smaller chunks once the process is worked out
fn gen_from_source(source_dir: &str, dest_dir: &str) -> Result<(), String> {
    //Clear out the contents of the dest_dir
    println!("clearing destination dir...");
    let _ = fs::remove_dir_all(dest_dir).map_err(|e| e.description().to_string());
    let _ = fs::create_dir_all(dest_dir).map_err(|e| e.description().to_string());
    
    //Create an emitter
    let ps = syntax::parse::ParseSess::new();
	let mut fgc = vec![];
	let mut cx;
	get_ctxt!(cx, ps, fgc);
    let emitter = RustEmitter::new(cx);
    
    //Get the spec source
    println!("parsing source spec files...");
    let parsed = try!(parse::from_dir(source_dir).map_err(|e| e.description().to_string()));
    
    for endpoint in parsed {
        //1. Get the path for the generated source
        println!("building path for {}...", endpoint.get_name());
        let mut path = try!(endpoint.get_mod_path().map_err(|_| format!("Error parsing path for {}", endpoint.get_name())));    
        let (file, file_is_mod) = match path.len() {
            0 => ("mod".to_string(), true),
            1 => ("mod".to_string(), true),
            _ => (try!(path.pop().ok_or(format!("Error parsing path filename for {}", endpoint.get_name()))), false)
        };
        
        let dir_path = format!("{}/{}", dest_dir, path.join("/"));
        let file_path = format!("{}/{}.rs", dir_path, file);
        
        //Ensure the path exists
        try!(fs::create_dir_all(&dir_path).map_err(|e| e.description().to_string()));
        
        //2. Open the source file
        let (mut src_file, is_new) = match OpenOptions::new().write(true).append(true).open(&file_path) {
            Ok(f) => {
                println!("Opened file...");
                (f, false)
            },
            Err(_) => {
                println!("Creating file...");
                (try!(File::create(&file_path).map_err(|e| e.description().to_string())), true)
            }
        };
        
        //4. Emit file header
        println!("emitting source for {}...", endpoint.get_name());
        try!(emitter.emit_str(&"//Autogenerated\n", &mut src_file).map_err(|e| e.description().to_string()));
        try!(src_file.sync_all().map_err(|e| e.description().to_string()));
        
        //5. TODO: Generate and emit source functions
        
        //6. Emit mod header if file isn't mod
        if !file_is_mod {
            let mod_path = format!("{}/{}.rs", dir_path, "mod");
            let mut mod_file = match OpenOptions::new().write(true).append(true).open(&mod_path) {
                Ok(f) => f,
                Err(_) => File::create(&mod_path).unwrap()
            };
            
            try!(emitter.emit_str(&format!("pub mod {};\n", file), &mut mod_file).map_err(|e| e.description().to_string()));
            try!(mod_file.sync_all().map_err(|e| e.description().to_string()));
        }
    }
    
    File::create("src/genned/mod.rs").unwrap();
    
    let mut mod_paths = Vec::new();
    
    for entry in WalkDir::new(dest_dir).min_depth(1).max_open(1).into_iter().filter_map(|e| e.ok()) {
        let meta = entry.metadata().unwrap();
        if meta.is_dir() {
            if let Some(parent) = entry.path().parent() {
                let mut parent = parent;
                let name = entry.file_name();
                
                mod_paths.push((
                    (format!("{}/{}.rs", parent.to_str().unwrap().to_string(), "mod")), 
                    name.to_str().unwrap().to_string()
                ));
            }
        }
    }
    
    for (path, name) in mod_paths {
        let mut mod_file = match OpenOptions::new().write(true).append(true).open(&path) {
            Ok(f) => f,
            Err(_) => File::create(&path).unwrap()
        };
        
        try!(emitter.emit_str(&format!("pub mod {};\n", name), &mut mod_file).map_err(|e| e.description().to_string()));
        try!(mod_file.sync_all().map_err(|e| e.description().to_string()));
    }

    Ok(())
}