// .chunks // 1, 2, 3, 4, 5, 6, 7, 8, 9
// .windows // 1, 2, 3

fn main() {
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    // Minecraft의 Chunk와 비슷하다.
    // Chunk size가 0일 경우 Panic 발생
    for chunk in num_vec.chunks(3) {
        println!("Chunk is : {chunk:?}");
    }

    // window 하나가 출력된 후 첫 번째 window index에 1만큼 증가하여 다음 window를 출력한다.
    // Chunk size가 0일 경우 Panic 발생
    for window in num_vec.windows(3) {
        println!("Window is : {window:?}");
    }
}