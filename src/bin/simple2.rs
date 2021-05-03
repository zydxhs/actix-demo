use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "Responses")]
enum Messages {
    Ping,
    Pong,
}

enum Responses {
    GotPing,
    GotPong,
}

impl<A, M> MessageResponse<A, M> for Responses
where
    A: Actor,
    M: Message<Result = Responses>,
{
    fn handle(
        self,
        _: &mut <A as actix::Actor>::Context,
        tx: Option<OneshotSender<<M as actix::Message>::Result>>,
    ) {
        if let Some(tx) = tx {
            match tx.send(self) {
                Ok(a) => a,
                _ => unreachable!(),
            };
        }
    }
}

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }
    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

impl Handler<Messages> for MyActor {
    type Result = Responses;

    fn handle(&mut self, msg: Messages, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Messages::Ping => Responses::GotPing,
            Messages::Pong => Responses::GotPong,
        }
    }
}

#[actix_rt::main]
async fn main() {
    {
        let addr = MyActor.start();
        let ping_fut = addr.send(Messages::Ping).await;
        let pong_fut = addr.send(Messages::Pong).await;

        match ping_fut {
            Ok(res) => match res {
                Responses::GotPing => println!("Ping received"),
                Responses::GotPong => println!("Pong received"),
            },
            Err(e) => println!("Actor is probably dead: {}", e),
        }

        match pong_fut {
            Ok(res) => match res {
                Responses::GotPing => println!("Ping received"),
                Responses::GotPong => println!("Pong received"),
            },
            Err(e) => println!("Actor is probably dead: {}", e),
        }
    }

    // 另一个方案：去掉作用域、sleep，使用 System::current().stop();
    actix_rt::time::sleep(std::time::Duration::from_millis(100)).await;
}
