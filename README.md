# Stateright Counter

An Actor that implements Increment and Read using [Stateright](https://github.com/stateright/stateright).

# Build

`cargo build`

# Run

If you want to see the UDP packets you can start `tcpdump`:
```
$ sudo tcpdump -i lo -n udp port 3333 -X
tcpdump: verbose output suppressed, use -v[v]... for full protocol decode
listening on lo, link-type EN10MB (Ethernet), snapshot length 262144 bytes
```

Next run with `spawn` command, and we see the `println!` and `log` messages when it starts:
```
wink@3900x:~/prgs/rust/myrepos/sr-counter (main)
$ cargo run spawn
   Compiling sr-counter v0.2.0 (/home/wink/prgs/rust/myrepos/sr-counter)
    Finished dev [unoptimized + debuginfo] target(s) in 2.31s
     Running `target/debug/sr-counter spawn`
A CounterActor supports two messges, Increment and Read.
You can monitor and interact using tcpdump and netcat.
Use `tcpdump -D` if you see error `lo0: No such device exists`.
Examples:
$ sudo tcpdump -i lo0 -s 0 -nnX
$ nc -u 127.0.0.1 3333
"Increment"
"Read"

on_start:+ self: CounterActor, id: 127.0.0.1:3333 _o: []
on_start:- ret_value: CounterState { id: Id(139637976796421), counter_value: 0 }
[2022-03-29T16:36:39Z INFO  stateright::actor::spawn] Actor started. id=127.0.0.1:3333, state=CounterState { id: Id(139637976796421), counter_value: 0 }, out=[]
```

Start `nc`, and you see a blank line:
```
$ nc -u 127.0.0.1 3333

```

`nc` is now waiting for JSON formatted messages. In this case
the commands are just simple word strings, but they must be
surrounded by quotes. So issue a "Read" (Note; you can also
start with "Increment" if you'd like). The CounterActor
will respond with a "ReadOk" message with the "value":
```
$ nc -u 127.0.0.1 3333
"Read"
{"ReadOk":{"value":0}}
```
Next issue the "Increment" (Note; you may press return to begin
the command on a new line, if you wish):
```
"Increment"
```
And follow that up with another "Read", to see the new value:
```
"Read"
{"ReadOk":{"value":1}}
```

The full output of the `cargo run spawn` is:
```
wink@3900x:~/prgs/rust/myrepos/sr-counter (main)
$ cargo run spawn
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/sr-counter spawn`
A Counter supports two messges Increment and Read
You can monitor and interact using tcpdump and netcat. Examples:
$ sudo tcpdump -i lo0 -s 0 -nnX
$ nc -u 127.0.0.1 3333
"Increment"
"Read"

on_start:+ self: CounterActor, id: 127.0.0.1:3333 _o: []
on_start:- ret_value: CounterState { id: Id(139637976796421), counter_value: 0 }
[2022-03-29T03:46:09Z INFO  stateright::actor::spawn] Actor started. id=127.0.0.1:3333, state=CounterState { id: Id(139637976796421), counter_value: 0 }, out=[]
[2022-03-29T03:46:18Z INFO  stateright::actor::spawn] Received message. id=127.0.0.1:3333, src=127.0.0.1:60529, msg=Read
on_msg:+ self: CounterActor, state: CounterState { id: Id(139637976796421), counter_value: 0 }, src: 127.0.0.1:60529, msg: Read, o: []
on_msg:- self: CounterActor, state: CounterState { id: Id(139637976796421), counter_value: 0 }, src: 127.0.0.1:60529, msg: Read, o: [Send(Id(139637976853617), ReadOk { value: 0 })]
[2022-03-29T03:46:41Z INFO  stateright::actor::spawn] Received message. id=127.0.0.1:3333, src=127.0.0.1:60529, msg=Increment
on_msg:+ self: CounterActor, state: CounterState { id: Id(139637976796421), counter_value: 0 }, src: 127.0.0.1:60529, msg: Increment, o: []
on_msg:- self: CounterActor, state: CounterState { id: Id(139637976796421), counter_value: 1 }, src: 127.0.0.1:60529, msg: Increment, o: []
[2022-03-29T03:46:47Z INFO  stateright::actor::spawn] Received message. id=127.0.0.1:3333, src=127.0.0.1:60529, msg=Read
on_msg:+ self: CounterActor, state: CounterState { id: Id(139637976796421), counter_value: 1 }, src: 127.0.0.1:60529, msg: Read, o: []
on_msg:- self: CounterActor, state: CounterState { id: Id(139637976796421), counter_value: 1 }, src: 127.0.0.1:60529, msg: Read, o: [Send(Id(139637976853617), ReadOk { value: 1 })]
```

And the `tcpdump` output is:
```
$ sudo tcpdump -i lo -n udp port 3333 -X
tcpdump: verbose output suppressed, use -v[v]... for full protocol decode
listening on lo, link-type EN10MB (Ethernet), snapshot length 262144 bytes
20:45:51.478684 IP 127.0.0.1.49375 > 127.0.0.1.3333: UDP, length 7
	0x0000:  4500 0023 e865 4000 4011 5462 7f00 0001  E..#.e@.@.Tb....
	0x0010:  7f00 0001 c0df 0d05 000f fe22 2252 6561  ...........""Rea
	0x0020:  6422 0a                                  d".
20:45:51.478896 IP 127.0.0.1.3333 > 127.0.0.1.49375: UDP, length 22
	0x0000:  4500 0032 5971 4000 4011 e347 7f00 0001  E..2Yq@.@..G....
	0x0010:  7f00 0001 0d05 c0df 001e fe31 7b22 5265  ...........1{"Re
	0x0020:  6164 4f6b 223a 7b22 7661 6c75 6522 3a30  adOk":{"value":0
	0x0030:  7d7d                                     }}
20:46:18.118694 IP 127.0.0.1.60529 > 127.0.0.1.3333: UDP, length 7
	0x0000:  4500 0023 531c 4000 4011 e9ab 7f00 0001  E..#S.@.@.......
	0x0010:  7f00 0001 ec71 0d05 000f fe22 2252 6561  .....q.....""Rea
	0x0020:  6422 0a                                  d".
20:46:18.118903 IP 127.0.0.1.3333 > 127.0.0.1.60529: UDP, length 22
	0x0000:  4500 0032 5d2f 4000 4011 df89 7f00 0001  E..2]/@.@.......
	0x0010:  7f00 0001 0d05 ec71 001e fe31 7b22 5265  .......q...1{"Re
	0x0020:  6164 4f6b 223a 7b22 7661 6c75 6522 3a30  adOk":{"value":0
	0x0030:  7d7d                                     }}
20:46:41.190680 IP 127.0.0.1.60529 > 127.0.0.1.3333: UDP, length 12
	0x0000:  4500 0028 531d 4000 4011 e9a5 7f00 0001  E..(S.@.@.......
	0x0010:  7f00 0001 ec71 0d05 0014 fe27 2249 6e63  .....q.....'"Inc
	0x0020:  7265 6d65 6e74 220a                      rement".
20:46:47.374738 IP 127.0.0.1.60529 > 127.0.0.1.3333: UDP, length 7
	0x0000:  4500 0023 531e 4000 4011 e9a9 7f00 0001  E..#S.@.@.......
	0x0010:  7f00 0001 ec71 0d05 000f fe22 2252 6561  .....q.....""Rea
	0x0020:  6422 0a                                  d".
20:46:47.374947 IP 127.0.0.1.3333 > 127.0.0.1.60529: UDP, length 22
	0x0000:  4500 0032 64e2 4000 4011 d7d6 7f00 0001  E..2d.@.@.......
	0x0010:  7f00 0001 0d05 ec71 001e fe31 7b22 5265  .......q...1{"Re
	0x0020:  6164 4f6b 223a 7b22 7661 6c75 6522 3a31  adOk":{"value":1
	0x0030:  7d7d                                     }}
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
