// starts at 50
// L**, R**
// password is how many times it hits 0 exactly through any sequence of rotations

// parse data, loop through if it hits zero add one to secret code
// L -> negative nums
// R -> positive nums 

fn secret_decoder(nums:&[isize]) -> isize {
    let mut password:isize = 0;
    let mut state:isize = 50; 
    
    for &n in nums {
        state += n;
        if state == 0 {
            password += 1;
        }
    }
    password

}


fn main() {
    println!("Hello, world!");
}
