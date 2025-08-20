use std::time::{Duration, SystemTime};
use std::env;
use std::fs::{DirBuilder, OpenOptions};
use std::io::Write;
use std::io;

pub fn single_start() {
    // ? zahrat cpu
    //
    // PROBLEM: cas se postupne prodluzuje i kdyz se pocita to same
    //      vetsi memory usage over time 
    //  IDEA : shadowing puts the value in a different adress
    //      let x = 1;   x_01
    //      let x = 4;   x_02
    //         totalne jina adresa
    //
    // X SOLUTION 
    //      no shadowing, muttable variables
    //      different log system ( without it it is like 1999x faster)
    
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

    let mut time_rec: [Duration; 5] = [Duration::new(0,0); 5];
    let mut run_count = 0;
    let mut sys_time: SystemTime;

    let start_num: u128 = u128::pow(10,11);
    let end_num: u128 = (u128::pow(10,11)) + 1_000_000;

    let mut a_matrix: [[u128; 10] ; 10];
    let mut b_matrix: [[u128; 10] ; 10];
    let mut result: [[u128; 10]; 10];

    let mut matrix_num: u128;

    let mut time_elapsed: Duration;


    println!("Started...Please wait");
    println!("The program will do math operations in a loop 5 times to get the average time");
    log_content = format!("{log_content}\n############## The program will start doing matrix multiplication from numbers 10^11 to 10^11 + 3000, 5 times to get an average time ##############");

    while run_count < 5 {
        sys_time = SystemTime::now();
        for x in start_num..end_num {
            println!("\n#######{x} out of {end_num}#######");
    
            a_matrix = [
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
            b_matrix = [
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
            result = [[0; 10]; 10];
    
            matrix_num = 0;
            for x in 0..a_matrix.len() {
                for k in 0..b_matrix.len() {
                    for y in 0..a_matrix[x].len() {
                        matrix_num += a_matrix[x][y] * b_matrix[y][k];
                    };
                    result[x][k] = matrix_num;
                    matrix_num = 0;
                }
            };
        };

        time_elapsed = sys_time.elapsed().expect("Time error");
        time_rec[run_count] = time_elapsed;


        println!("\n\n ##### loop time: {time_elapsed:?} #####\n\n");



        run_count += 1;
    };
    //average time
    let mut avg_time: Duration = Duration::new(0,0);
    for time in time_rec {
        avg_time += time;
    };
    avg_time = avg_time / (time_rec.len()) as u32;
    log_content = format!("{log_content}\n\n###### The average time is: {avg_time:?} ######");
    log_content = format!("{log_content}\nAll times: \n{time_rec:?}");
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
                println!("File saved");
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
