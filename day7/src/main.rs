use std::fs::File;
use std::io::{ BufReader, BufRead, Error};

#[derive(Clone)]
struct Dir {
    name: String,
    dirs: Vec<Dir>,
    sfile: Vec<Sfile>,
    last: Option<Box<Dir>>,
}

#[derive(Clone)]
struct Sfile {
    name: String,
    size: i32,
}


fn main() -> Result<(), Error> {
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut root:Dir = Dir {
        name: String::from("/"),
        dirs: Vec::new(),
        sfile: Vec::new(),
        last: None,
    };

    let mut currentDir:Dir = root.clone();

    let mut lastDir:Dir = Dir {
        name: String::from("/"),
        dirs: Vec::new(),
        sfile: Vec::new(),
        last: None,
    };

    let mut tempcount = 0;

    for line in buffered.lines() {
        if tempcount > 0 {
            let x = line?;
            let command = x.get(0..3).unwrap();
            match command {
            "$ c" => {
                if x.ends_with("..") { 
                    currentDir = lastDir.clone();
                } else {
                    let dirName = x.get(5..x.len());
                    lastDir = currentDir;
                    currentDir = Dir {
                        name: String::from(dirName.unwrap()),
                        dirs: Vec::new(),
                        sfile: Vec::new(),
                        last: Some(Box::new(lastDir.clone())),
                    }   
                }
            },
            "dir" => {
                let dirName = x.get(4..x.len()).unwrap();
                let tempDir = Dir {
                    name: String::from(dirName),
                    dirs: Vec::new(),
                    sfile: Vec::new(),
                    last: Some(Box::new(lastDir.clone())),
                };
                currentDir.dirs.insert(0, tempDir);
            }
            "$ l" => (),
            _ => {
                let fileSize = x.split_once(" ").unwrap().0.parse::<i32>().unwrap();
                let fileName = x.split_once(" ").unwrap().1.to_string();
                currentDir.sfile.insert(0, Sfile { name: fileName, size: fileSize });
            }
            };
        } else {
            tempcount += 1;
        }
        println!("{}", currentDir.name);
    }

    fn countFileSize(mut dir: Dir) -> i32 {
        let mut score = 0;

        println!("Now in dir {}", dir.name);

        if dir.sfile.len() != 0 {
            score += dir.sfile.pop().unwrap().size;
        }

        println!("Now in bob {}", dir.dirs.len());

        while !dir.dirs.is_empty() {
            score += countFileSize(dir.dirs.pop().unwrap());
        }
        
        println!("Now in bee {}", dir.name);

        if !dir.last.is_none() {
            score += countFileSize(*dir.last.unwrap());
        }

        return  score;
    }



    println!("{}", countFileSize(currentDir));

    Ok(())
}