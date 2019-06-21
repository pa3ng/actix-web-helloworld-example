# Notes/Opinions:

* Rust's 99th-percentile in Kubernetes (Minikube) is sub-millisecond at 20,000 requests per second and whereas all others start falling apart at 20,000, rust is the sole application that'll take 50,0000 requests per second: ***short of resorting to writing your own low-level code, you will not find a more performant language***
* Rust uses little memory
    * 1000 tps < 9 MB
    * 5000 tps < 10.5 MB
    * 10000 tps < 20 MB
    * 20000 tps in 150 MB
    * 1000 concurrent connections < 27 MB
    * 5000 concurrent connections < 132 MB
* Rust does not use garbage collection, is nonetheless type-safe. If it compiles, it's also thread-safe
* Rust does not have `null` (see [Option](https://doc.rust-lang.org/std/option/index.html)), Rust (like Go) does not do exceptions (see [Result](https://doc.rust-lang.org/std/result/)), Rust is [lazy](https://doc.rust-lang.org/book/ch13-02-iterators.html), Rust has [closures (aka lambda expressions)](https://doc.rust-lang.org/rust-by-example/fn/closures.html), Rust does [ranges](https://doc.rust-lang.org/std/ops/struct.Range.html) [slices](https://doc.rust-lang.org/book/ch04-03-slices.html), Rust has a powerful [match](https://doc.rust-lang.org/book/ch06-02-match.html) operator, Rust has [hygenic macros](https://doc.rust-lang.org/book/ch19-06-macros.html) (see [clap](https://docs.rs/clap) for a practical example): Rust is as expressive as all the other modern languages you've become accustomed to
* _"If it compiles, it runs"_ generally applies to Rust: Rust moves most bugs to compilation rather than runtime. This, admittedly, will be initially very frustrating
* [rustup](https://rustup.rs/) and [cargo](https://doc.rust-lang.org/cargo/) are awesome, as is the Rust ecosystem as a whole ([crates.io](https://crates.io/), [docs.rs](https://docs.rs/), [docs](https://doc.rust-lang.org/), [code formatting](https://github.com/rust-lang/rustfmt), [linting](https://github.com/rust-lang/rust-clippy), and more)
* `cargo test` is fast: like JavaScript, you can expect the code/test cycle to be quick
* Rust has decent [IDE support](https://areweideyet.com/)
* Probably the only thing holding Rust adoption back currently is the upcoming `async`/`await` [support](https://areweasyncyet.rs/), which is coming soon
* Rust is ["the most loved languague"](https://insights.stackoverflow.com/survey/2019#most-loved-dreaded-and-wanted) according to Stack Overflow’s annual Developer Survey.

## Bottom Line?

Rust is the next big language. It's fast, it's good. If you're ready for the next big thing, this is it.

## A bonus example -- _asynchronous_ HTTP requests!!!:

`async`/`await` has not yet landed in Rust (see [areweasyncyet](https://areweasyncyet.rs/)), but here's an example using the current `future` syntax.

```rust
use hyper::rt::{self, Future, Stream};
use hyper::Client;

fn main() {
    rt::run(rt::lazy(|| {
        let client = Client::new();

        let get0 = client
            .get("http://127.0.0.1:8080/hello?name=world".parse().unwrap())
            .and_then(|res| res.into_body().concat2());

        let get1 = client
            .get(format!("http://127.0.0.1:8080/hello?name={}", ".".repeat(10000)).parse().unwrap())
            .and_then(|res| res.into_body().concat2());

        get0.join(get1).map(|(got0, got1)| {
            println!("GET (0): {:?}", got0);
            println!("GET (1): {:?}", got1);
        })
        .map_err(|err| println!("Error: {}", err))
    }));
}
```

For a slightly more complex example that is used to hit items' buckets (at a sustained 50 concurrent requests), see [populate_cache](https://git.target.com/redsky/item-identifier-lookup/blob/master/src/main.rs#L377-L420) and associated calling [code](https://git.target.com/redsky/item-identifier-lookup/blob/master/src/main.rs#L424-L445). Note that a single thread manages the requests.
