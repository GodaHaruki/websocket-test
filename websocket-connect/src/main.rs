use ws::{connect, CloseCode, Sender};

fn main() {
    connect("ws://127.0.0.1:8080", |out| {
        let mut is_task = 0;
        for i in 0..100 {
            println!("{}", i); // 出力が499で止まる
            out.send("hello");
            is_task += 1;
        }

        move |msg: ws::Message| {
            println!("returned: {}", msg);
            out.close(CloseCode::Normal);
            Ok(())
        }
    });
}
