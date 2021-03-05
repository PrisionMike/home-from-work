use std::process::Command;

fn main() {
    let output = Command::new("wsl")
                          .args(&["ip","addr"])
                          .output();
    
    let output = match output {
        Ok(v) => v,
        Err(error) => panic!("Aditi is not feeling well, {:?}", error)
    };
    
    let fulloutput = &output.stdout;
    
    // println!("{}",String::from_utf8_lossy(fullstring))
    // println!("{}",fullstring)

    let fullstringoutput = String::from_utf8_lossy(fulloutput);

    let mut closerip = "";

    for line in fullstringoutput.lines() {
        if line.contains("eth0") {
            println!("grepped line:\t{}", line);

            // let loc = line.rfind("inet ");
            // match loc {
            //     Some(v) => {
            //         let t: Vec<str> = line[(v+5)..].rsplit("/").collect();
            //         closerip = t[1];                   
            //     },
                
            //     None => continue,
            // };

            if let Some(v) = line.rfind("inet ") {
                println!("{}", &line[(v+5)..]);
                
                if let Some(t) = line.rfind("/") {
                    
                    closerip = &line[(v+5)..t];
                    println!("further cut: {}",closerip);
                }
            }
        }
    }

    println!("And the result is:\n{}", closerip);


}

