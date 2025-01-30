// This file acts like a connection to other entrypoints of subdirectories containing code
// what's special about lib.rs?
// You don't have to make the connection in main.rs anything you see here can be accessible in main only using "use" keyword no need for "mod"
// mod => to connect or to declare a module for usage to let the compiler search for it
// use => find something within the declared module in your scope.
// for main.rs your scope contains all the crates you install, std libs, and all that..
// your root is called crate:: you can use it in "use" keyword

// however since you used lib.rs to see things for you. You can no longer use crate:: in main. But you can still use it in other subfiles.
// for main instead of crate:: use [project_name]::

pub mod utils;

pub fn ma(){
    println!("ma");
}