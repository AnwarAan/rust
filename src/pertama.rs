use crate::kedua::say_hello as say_hello_dua;

pub fn say_hello() {
    println!("Hai Muchamad");

    say_hello_dua();
}

pub mod kedua {
    pub fn say_hello() {
        super::say_hello();
    }
}
