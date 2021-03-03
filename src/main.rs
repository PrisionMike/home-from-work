use std::process::Command;

fn main() {
    let output = Command::new("wsl")
                          .args(&["ip","addr"])
                          .output();
    
    let output = match output {
        Ok(v) => v,
        Err(error) => panic!("Aditi is not feeling well, {:?}", error)
    };
                              
    println!("{}",String::from_utf8_lossy(&output.stdout))
}

