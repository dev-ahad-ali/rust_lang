use std::time::Duration;

fn main() {
    trpl::run(async {
        let slow = async {
            println!("Slow started");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("Slow ended");
        };
        let fast = async {
            println!("Fast started");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("Fast ended");
        };

        trpl::race(slow, fast).await;
    });
}
