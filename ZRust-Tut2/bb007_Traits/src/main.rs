
struct RustDev{
    awesome:bool,
}

struct JavaDev{
    awesome:bool,
}

struct PythonDev{
    awesome:bool,
}

trait Developer{
    fn new(awesome:bool) -> Self ;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("Hello World!")
    }
}
impl Developer for RustDev{
    fn new(awesome: bool) -> Self {
       RustDev{awesome:awesome}
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("println!(\"Hello World!\");")
    }
}

impl Developer for PythonDev{
    fn new(awesome: bool) -> Self {
        PythonDev{awesome:awesome}
    }

    fn language(&self) -> &str {
       "Python 3"
    }

    fn say_hello(&self) {
        println!("print(\"Hello World\")");
    }
}


fn main() {
    // let r = RustDev{awesome:true};
    let r = RustDev::new(true);
    let p = PythonDev::new(true);

    println!("\n-------------------------\n{}\n-------------------------", r.language());
    r.say_hello();
    println!("\n-------------------------\n{}\n-------------------------", p.language());
    p.say_hello();
}
