use std::time::{Duration, SystemTime};

pub fn single_start() {
    //spustit 10x na 5_000_000
        //udelat average cas
    //logovat do souboru misto vypisovat do konzole
    let mut time_rec: [Duration; 10] = [Duration::new(0,0); 10];
    let mut run_count = 0;
    while run_count < 10 {
        let sys_time = SystemTime::now();
        for x in 0..5_000{
            println!("#######{} out of 10#######\n", run_count + 1);
            println!("#######{x} out of 10000000#######");
    
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
            println!("\n");
            for line in &a_matrix {
                println!("{line:?}");
            };
            println!("\n");
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
            println!("\n");
            for line in &b_matrix {
                println!("{line:?}");
            };
            println!("\n");
    
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
            println!("\n");
            println!("## ## ## ## ## ## ## ## ## ##");
            for line in &result {
                println!("{line:?}\n");
            };
            println!("## ## ## ## ## ## ## ## ## ##");
            println!("\n");
        };

        let time = sys_time.elapsed().expect("Time error");
        println!("{time:?}");
        time_rec[run_count] = time;

        run_count += 1;
    };
    //average time
    let mut avg_time: Duration = Duration::new(0,0);
    for time in time_rec {
        avg_time += time;
    };
    let avg_time = avg_time / (time_rec.len()) as u32;
    println!("###### {avg_time:?} ######");
}
