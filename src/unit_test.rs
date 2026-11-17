use super::*;

#[test]
fn read_file(){
    match fs::File::create_new("test.txt"){
        Ok(_)=>{ assert!(data::read_file("test.txt").is_ok())}
        Err(e)=>{ println!("{}",e.to_string());}
    }
    if let Err(e)=fs::remove_file("test.txt"){
        println!("{}",e.to_string());
    }
}