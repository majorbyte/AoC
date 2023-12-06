pub fn task() {
    print!("\nsum:{}",  get_sum(vec![ 7,9,15,40,30,200]));
}
fn get_sum(input: Vec<i64>) -> i64{
    
    let mut n = 0;
    let mut total = 1;
    while n < input.len(){
        let mut i = (input[n] / 2).abs();
        let mut c = 0;
        while i > 0{
            if i * (input[n]-i) <= input[n+1] {
                break
            }
            c += 1;

            i -= 1;
        }
        if input[n] % 2 == 0 {
            total = total * (c *2 -1);
        } else {
            total = total * (c *2);
        }

        n += 2;
    }
    return total;

}

