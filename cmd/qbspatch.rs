#![forbid(unsafe_code)]
use qbsdiff::Bspatch;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(
        qbspatch =>
        (version: "1.2.0")
        (about: "fast and memory saving bsdiff 4.x compatible patcher")
        (@arg BSIZE:
            -b +takes_value
            "buffer size")
        (@arg SOURCE:
            +required
            "source file")
        (@arg TARGET:
            +required
            "target file")
        (@arg PATCH:
            +required
            "patch file"))
    .get_matches();

    let bsize_expr = matches.value_of("BSIZE").unwrap_or("16384");
    let source_name = matches.value_of("SOURCE").unwrap();
    let target_name = matches.value_of("TARGET").unwrap();
    let patch_name = matches.value_of("PATCH").unwrap();

    match BspatchApp::new(bsize_expr, source_name, target_name, patch_name) {
        Ok(app) => {
            if let Err(e) = app.execute() {
                eprintln!("error: {}", e);
            }
        }
        Err(e) => eprintln!("error: {}", e),
    }
}

struct BspatchApp {
    source: Vec<u8>,
    target: Box<dyn Write>,
    patch: Vec<u8>,
    bsize: usize,
}

impl BspatchApp {
    pub fn new(
        bsize_expr: &str,
        source_name: &str,
        target_name: &str,
        patch_name: &str,
    ) -> io::Result<Self> {
        let bsize;
        match usize::from_str(bsize_expr) {
            Ok(n) => bsize = n,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
        }

        let mut source;
        if source_name == "-" {
            source = Vec::new();
            io::stdin().read_to_end(&mut source)?;
        } else {
            source = fs::read(source_name)?;
        }
        source.shrink_to_fit();

        let target: Box<dyn Write>;
        if target_name == "-" {
            target = Box::new(io::stdout());
        } else {
            target = Box::new(fs::File::create(target_name)?);
        }

        let mut patch = fs::read(patch_name)?;
        patch.shrink_to_fit();

        Ok(BspatchApp {
            source,
            target,
            patch,
            bsize,
        })
    }

    pub fn execute(self) -> io::Result<()> {
        Bspatch::new(&self.patch[..])?
            .buffer_size(self.bsize)
            .apply(&self.source[..], self.target)?;
        Ok(())
    }
}
