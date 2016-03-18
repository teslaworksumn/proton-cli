#![feature(plugin)]
#![plugin(stainless)]

extern crate proton_cli;

describe! initialize_project {
    before_each {
        println!("Before");
    }
    
    after_each {
        println!("After");
    }

    failing "works" {
        println!("Hello");
        panic!("Woah");
    }

}
