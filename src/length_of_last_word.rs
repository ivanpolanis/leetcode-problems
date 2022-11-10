impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut started = false;
        let mut count = 0; 
        for c in s.chars().rev() {
            if c != ' ' {
                started = true;
                cont +=1;
            }
            if c == ' ' && started {
                break;
            }
        }
        return cont;
    }
}
