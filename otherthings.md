```
let mut vec = Vec::with_capacity(2);

nitc::nit_commit("Kat21".to_string(), "Added some basic files.".to_string(), vec);
configure::deser_config();



if let Some(ref mut stdout) = child.stdout {
    for line in BufReader::new(stdout).lines() {
        let line = line.unwrap();
        println!("[stdout] {}", line);
    }
}

if let Some(ref mut stderr) = child.stderr {
    for line in BufReader::new(stderr).lines() {
        let line = line.unwrap();
        println!("[stderr] {}", line);
    }
}
```