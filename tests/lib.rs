#![feature(plugin)]
#![plugin(stainless)]

extern crate proton_cli;
extern crate tempdir;

use proton_cli::{Error, initialize_project};

describe! initialize_project {
    before_each {
        println!("Before");
    }
    
    after_each {
        println!("After");
    }

    it "works with no directory" {
    }

    it "works with an empty directory" {
    }

    it "fails with a non-empty directory" {
    }

}
