use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "Result<bool,std::io::Error>")]
struct Ping;

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

impl Handler<Ping> for MyActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Ping, _ctx: &mut Self::Context) -> Self::Result {
        println!("Ping received");
        Ok(true)
    }
}

#[actix_rt::main]
async fn main() {
    {
        let addr = MyActor.start();

        match addr.send(Ping).await {
            Ok(res) => println!("Got result: {}", &res.unwrap()),
            Err(err) => println!("Got error: {}", &err),
        }
    }

    // System::current().stop();
    actix_rt::time::sleep(std::time::Duration::from_millis(100)).await;
}
