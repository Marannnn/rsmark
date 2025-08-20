use std::time::{Duration, SystemTime};
use std::env;
use std::fs::{DirBuilder, OpenOptions};
use std::io::Write;
use std::io;

pub fn single_start() {
    //logovat do souboru misto vypisovat do konzole
        // X    vytvorit slozku
        //Zeptat se na log -> 
            // zeptat se na jmeno
        //ulozit log do slozky
    // ? zahrat cpu
    //
    // PROBLEM: cas se postupne prodluzuje i kdyz se pocita to same
    
    let mut log_content = String::new();

    let directory = match env::var("HOME") {
        Ok(unix_dir) => {
            log_content = format!("{log_content}Found HOME dir");

            let unix_dir = format!("{unix_dir}/.local/share/rsmark");
            unix_dir
        },
        Err(_) => {
            let win_dir = match env::var("APP_DATA") {
                Ok(dir) => {
                    log_content = format!("{log_content}Found APP_DATA dir");

                    let dir = format!("{dir}/rsmark");
                    dir
                },
                Err(_) => {
                    panic!("Did not find directory, please check if you have one of the following: $HOME (on Unix and Unix like systems) or $APP_DATA (Windows)");
                },
            };
            win_dir
        },
    };

    DirBuilder::new()
        .recursive(true)
        .create(&directory).unwrap();

    let mut time_rec: [Duration; 10] = [Duration::new(0,0); 10];
    let mut run_count = 0;

    let start_num: u128 = u128::pow(10,11);
    let end_num: u128 = (u128::pow(10,11)) + 5000;

    println!("Started...Please wait");
    println!("The program will do math operations in a loop 5 times to get the average time");
    log_content = format!("{log_content}\n############## The program will start doing matrix multiplication from numbers 10^11 to 10^11 + 5000, 5 times to get an average time ##############");

    while run_count < 5 {
        let sys_time = SystemTime::now();
        for x in start_num..end_num {
            log_content = format!("{log_content}\n ########{} out of 10#######\n", run_count + 1);
            log_content = format!("{log_content}\n#### Currenty at number: {x} out of 100_100_000####");
            println!("\n#######{x} out of 1_000_0000#######");
    
            let a_matrix: [[u128; 10] ; 10] = [
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
            ];
            log_content = format!("{log_content}\n");
            for line in &a_matrix {
                log_content = format!("{log_content}{line:?}\n");
            };
            log_content = format!("{log_content}\n");
            let b_matrix: [[u128; 10] ; 10] = [
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
                [x,x,x,x,x,x,x,x,x,x],
            ];
            log_content = format!("{log_content}\n");
            for line in &b_matrix {
                log_content = format!("{log_content}{line:?}\n");
            };
            log_content = format!("{log_content}\n");
    
            let mut result: [[u128; 10]; 10] = [[0; 10]; 10];
    
            let mut num: u128 = 0;
            for x in 0..a_matrix.len() {
                for k in 0..b_matrix.len() {
                    for y in 0..a_matrix[x].len() {
                        num += a_matrix[x][y] * b_matrix[y][k];
                    };
                    result[x][k] = num;
                    num = 0;
                }
            };
            log_content = format!("{log_content}\n");
            log_content = format!(" {log_content} ## ## ## ## ## ## ## ## ## ##\n");
            for line in &result {
                log_content = format!("{log_content}{line:?}\n");
            };
            log_content = format!(" {log_content} ## ## ## ## ## ## ## ## ## ##");
            log_content = format!("{log_content}\n");
        };

        let time = sys_time.elapsed().expect("Time error");
        log_content = format!("{log_content}\n\n ##### loop time: {time:?} #####\n\n");
        time_rec[run_count] = time;


        println!("\n\n ##### loop time: {time:?} #####\n\n");



        run_count += 1;
    };
    //average time
    let mut avg_time: Duration = Duration::new(0,0);
    for time in time_rec {
        avg_time += time;
    };
    let avg_time = avg_time / (time_rec.len()) as u32;
    log_content = format!("{log_content}\n###### The average time is: {avg_time:?} ######");
    println!("###### The average time is: {avg_time:?} ######");

    loop {
        println!("Save log? y | n");
        let mut option = String::new();
        if let Err(err) = io::stdin().read_line(&mut option) {
            println!("{err}: Please try again");
            continue;
        };
        let option = option.trim();
        match option {
            "y" => {
                println!("Name of the log file:");
                let mut title = String::new();
                if let Err(err) = io::stdin().read_line(&mut title) {
                    println!("{err}");
                    continue;
                };
                let log_path= format!("{directory}/{title}.txt");

                let mut log_file = match OpenOptions::new().write(true).create(true).open(&log_path) {
                    Ok(file) => file,
                    Err(err) => {
                        println!("{err}: Couldn't create/open the file");
                        continue;
                    },
                };
                let log_content = log_content.as_bytes();
                let _ = log_file.write_all(log_content);
                break
            },
            "n" => break,
            _ => {
                println!("Please select y- yes | n-no");
                continue;
            },
        };
    };
}
