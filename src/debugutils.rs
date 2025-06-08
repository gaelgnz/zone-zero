pub fn log(frame_counter: i128, each_n_frames: i32, message: &str) {
    if frame_counter % each_n_frames as i128 == 0 {
        println!("[{}] {}", frame_counter, message);
    }
    
}
