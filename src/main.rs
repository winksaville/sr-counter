use serde::{Deserialize, Serialize};
use stateright::actor::{Actor, Id, Out};
use std::borrow::Cow;
use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
enum CounterMsg {
    Increment,
    Read,
    ReadOk { value: u8 },
}
use CounterMsg::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct CounterState {
    id: Id,
    counter_value: u8,
}

#[derive(Debug, Clone)]
struct CounterActor;

impl Actor for CounterActor {
    type Msg = CounterMsg;
    type State = CounterState;

    fn on_start(&self, id: Id, _o: &mut Out<Self>) -> Self::State {
        println!("on_start:+ self: {self:?}, id: {id} _o: {_o:?}");
        let ret_value = CounterState {
            id,
            counter_value: 0,
        };

        println!("on_start:- ret_value: {ret_value:?}");
        ret_value
    }

    fn on_msg(
        &self,
        id: Id,
        state: &mut Cow<Self::State>,
        src: Id,
        msg: Self::Msg,
        o: &mut Out<Self>,
    ) {
        println!("on_msg:+ self: {self:?}, state: {state:?}, src: {src}, msg: {msg:?}, o: {o:?}");
        assert_eq!(state.id, id);
        match msg {
            Increment => {
                let mut sm = state.to_mut();
                sm.counter_value += 1
            }
            Read => o.send(
                src,
                ReadOk {
                    value: state.counter_value,
                },
            ),
            ReadOk { value } => println!("ReadOk value: {value}"),
        }
        println!("on_msg:- self: {self:?}, state: {state:?}, src: {src}, msg: {msg:?}, o: {o:?}");
    }
}

fn main() -> Result<(), pico_args::Error> {
    use stateright::actor::spawn;

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info")); // `RUST_LOG=${LEVEL}` env variable to override

    let mut args = pico_args::Arguments::from_env();
    match args.subcommand()?.as_deref() {
        Some("spawn") => {
            let port = 3333;
            let address = Ipv4Addr::LOCALHOST;

            println!("A Counter supports two messges Increment and Read");
            println!("You can monitor and interact using tcpdump and netcat. Examples:");
            println!("$ sudo tcpdump -i lo0 -s 0 -nnX");
            println!("$ nc -u {address} {port}");
            println!("{}", serde_json::to_string(&CounterMsg::Increment).unwrap());
            println!("{}", serde_json::to_string(&CounterMsg::Read).unwrap());
            println!();

            spawn(
                serde_json::to_vec,
                |bytes| serde_json::from_slice(bytes),
                vec![(SocketAddrV4::new(address, port), CounterActor)],
            )
            .unwrap();
        }
        _ => {
            println!("USAGE:");
            println!("  ./counter spawn");
        }
    }

    Ok(())
}
