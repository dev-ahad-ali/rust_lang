use std::{future::Future, pin::Pin, time::Duration};

use ::trpl;
fn main() {
    trpl::run(async {
        // let (tx, mut rx) = trpl::channel();

        // let val = String::from("hi");
        // tx.send(val).unwrap();

        // let received = rx.recv().await.unwrap();
        // println!("Got: {received}");

        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx_fut1 = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received: {value}");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx_fut1), Box::pin(rx_fut), Box::pin(tx_fut)];

        trpl::join_all(futures).await;

        // other with using pin
        // let tx1_fut = pin!(async move {
        //     // --snip--
        // });
        // let rx_fut = pin!(async {
        //     // --snip--
        // });
        // let tx_fut = pin!(async move {
        //     // --snip--
        // });
        // let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

        let a = async { 1u32 };
        let b = async { "Hello" };
        let c = async { true };

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
    })
}
