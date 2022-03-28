# Stateright Counter

An Actor that implements Increment and Read using [Stateright](https://github.com/stateright/stateright).

# Build

`cargo build`

# Run

Currently it runs and the `println!` and `log` messages are seen when it starts:
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
[2022-03-28T22:53:39Z INFO  stateright::actor::spawn] Actor started. id=127.0.0.1:3333, state=CounterState { id: Id(139637976796421), counter_value: 0 }, out=[]
```

Then issue an "Increment" using `nc`:
```
$ nc -u 127.0.0.1 3333
Increment

```

And we see the increment when using `tcpdump`:
```
$ sudo tcpdump -i lo -n udp port 3333 -X
tcpdump: verbose output suppressed, use -v[v]... for full protocol decode
listening on lo, link-type EN10MB (Ethernet), snapshot length 262144 bytes
15:42:02.543896 IP 127.0.0.1.45180 > 127.0.0.1.3333: UDP, length 10
	0x0000:  4500 0026 3985 4000 4011 0340 7f00 0001  E..&9.@.@..@....
	0x0010:  7f00 0001 b07c 0d05 0012 fe25 496e 6372  .....|.....%Incr
	0x0020:  656d 656e 740a                           ement.
```

But we see no `on_msg`, `println!` or `log` messages, the output doesn't change:
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
[2022-03-28T22:53:39Z INFO  stateright::actor::spawn] Actor started. id=127.0.0.1:3333, state=CounterState { id: Id(139637976796421), counter_value: 0 }, out=[]
```

**So it seems there is some other needed initialization.**

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
