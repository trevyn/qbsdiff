use qbsdiff::Bsdiff;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(
        qbsdiff =>
        (version: "0.1.0")
        (about: "fast and memory saving bsdiff 4.x compatible delta compressor")
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

    match BsdiffApp::new(bsize_expr, source_name, target_name, patch_name) {
        Ok(app) => {
            if let Err(e) = app.execute() {
                eprintln!("error: {}", e);
            }
        }
        Err(e) => eprintln!("error: {}", e),
    }
}

struct BsdiffApp {
    source: Vec<u8>,
    target: Vec<u8>,
    patch: Box<dyn Write>,
    bsize: usize,
}

impl BsdiffApp {
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

        if source_name == "-" && target_name == "-" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "source and target could not be stdin at the same time",
            ));
        }

        let mut source = Vec::new();
        if source_name == "-" {
            io::stdin().read_to_end(&mut source)?;
        } else {
            File::open(source_name)?.read_to_end(&mut source)?;
        }

        let mut target = Vec::new();
        if target_name == "-" {
            io::stdin().read_to_end(&mut target)?;
        } else {
            File::open(target_name)?.read_to_end(&mut target)?;
        }

        let patch: Box<dyn Write>;
        if patch_name == "-" {
            patch = Box::new(io::stdout());
        } else {
            patch = Box::new(File::create(patch_name)?);
        }

        Ok(BsdiffApp {
            source,
            target,
            patch,
            bsize,
        })
    }

    pub fn execute(self) -> io::Result<()> {
        Bsdiff::new(&self.source[..])
            .buffer_size(self.bsize)
            .compare(&self.target[..], self.patch)?;
        Ok(())
    }
}
