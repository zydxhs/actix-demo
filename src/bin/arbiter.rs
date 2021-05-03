use std::usize;

use actix::prelude::*;

struct SumActor {}

impl Actor for SumActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Value(usize, usize);

impl Handler<Value> for SumActor {
    type Result = usize;

    fn handle(&mut self, msg: Value, _ctx: &mut Self::Context) -> Self::Result {
        msg.0 + msg.1
    }
}

struct DisplayActor {}

impl Actor for DisplayActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct Display(usize);

impl Handler<Display> for DisplayActor {
    type Result = ();

    fn handle(&mut self, msg: Display, _ctx: &mut Self::Context) -> Self::Result {
        println!("Got {:?}", msg.0);
    }
}

#[actix_rt::main]
async fn main() {
    let system = System::new();

    let execution = async {
        let sum_addr = SumActor {}.start();
        let dis_addr = DisplayActor {}.start();

        let sum_result = sum_addr.send(Value(6, 7)).await;
        match sum_result {
            Ok(res) => dis_addr.send(Display(res)).await.is_ok(),
            Err(e) => {
                eprintln!("Encountered mailbox error: {:?}", e);
                true
            }
        };
    };

    let arb = Arbiter::new();
    arb.spawn(execution);
    System::current().stop();
    system.run().unwrap();
}
