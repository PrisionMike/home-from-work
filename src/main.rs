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

    println!("And the result is:\n{}", &closerip);

    let result = portproxy(&closerip).expect("error implementing portproxy");

    let ppres = String::from_utf8_lossy(&result.stdout);

    println!("{}",ppres);

    let sshout = Command::new("wsl").args(&["sudo","/etc/init.d/ssh","start"]).output().expect("Error starting SSH on WSL");

    println!("{}",String::from_utf8_lossy(&sshout.stdout));
}

fn portproxy(a: &str) -> Result< std::process::Output, std::io::Error > {
    // let commstring1 = "netsh interface portproxy
                    //   add v4tov4 listenport=2396 connectaddress=";
    // let commstring2 = " connectport= 2396 listenaddress=0.0.0.0";

    // let commstring = format!("{}{}{}",commstring1,a,commstring2);

    Command::new("netsh")
            .args(&["interface","portproxy","add","v4tov4","listenport=3999",
                    "connectaddress=",a,"connectport=2396","listenaddress=0.0.0.0"])
            .output()
}

