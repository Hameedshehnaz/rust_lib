pub fn percentile_rank(mut data: Vec<u64>) {
    let data_len = data.len();
    let percentil = 100;
    data.sort();
    println!("data :{:?}", data);
    let mut rank = (percentil as f32/100.0 * data_len as f32) as usize;
    let mut max_indx = data_len as usize - 1 as usize;  
    let score: u64  = data[max_indx];
    let mut position = 1;
   
   
    println!("rank {:?}, score: {}, position: {:?}, percentile: {:?}", rank, score, position, percentil);
   
    while max_indx!= 0{
        let percentile = (rank as f32 /data_len as f32  * 100.0) as u8;
        rank = rank -1;
        if data[max_indx] == data[max_indx -1]{
            println!("rank {:?}, score: {}, position: {:?}, percentile: {:?}", rank, data[max_indx -1], position, percentile);
        } else {
            position = position + 1;
            println!("rank {:?}, score: {}, position: {:?}, percentile: {:?}", rank, data[max_indx -1], position, percentile);
        } 
        max_indx -= 1 ;
   }
    
}
