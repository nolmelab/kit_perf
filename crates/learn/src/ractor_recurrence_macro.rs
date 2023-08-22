//! 매크로 작은 책의 실용적인 매크로 소개 장이다.  따라가면서 하나씩 해보도록 한다. 
//! 

#[cfg(test)]
mod tests {
    #[test]
    fn read_it() {
        // let fib = recurrence![a[n] = 0, 1, ..., a[n-1] + a[n-2]];
        // for e in fib.take(10) { println!("{}", e) }

        // 위와 같은 recurrence! 매크로를 만드는 과정이다. 

        macro_rules! recurrence {
            ( a[n] = $($inits:expr),+ , ... , $recur:expr ) => { 1 };
        }

        let fib = recurrence![a[n] = 0, 1, ..., a[n-1] + a[n-2]];
    }
}
