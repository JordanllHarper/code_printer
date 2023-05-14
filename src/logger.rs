pub mod logger_mod{
    pub fn log(message : &str){
        println!("CODE_PRINTER: {message}");
    }
    pub fn log_scan(path : &str){
        println!("CODE PRINTER: SCANNING => {path}")
    }
    pub fn log_found_include(){
        log("FOUND INCLUDE FILE");
    }
}