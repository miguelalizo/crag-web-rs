# Notes

## Learning a bunch of things at once can be slow, but such is life.
- There were a lot of refactors, for example removing code duplication and outsourcing to a function instead that during the learning process feels like there are just bigger fish to fry
- Similarly, with this being my first serious Rust project and first time trying to use TCP protocol, there are times I have been stuck and not knowing if it is a lack of knowledge of TCP or the code is not doing what I intend, or both. This makes progress very slow
    - example: reading from the TCPStream took a while to figure out how to get it to not block
- Writing rust at first is a much slower experience than writing Python - also the project i am tackling requires knowledge of networking as opposed to data manipulation. This is more technical

I am determined to become a great software developer. I know I must be bad before I am okay and be okay before I am great. This will not stop me.

Code review:
- This is fundamentally what I am missing. There is so much I just picked up from you. I am fully aware that I need to be surrounded by experience and this is what I am seeking in my next role. I am extremely coachable and want it badly, but havent had the coaches.
    - Maybe you have a project we can colaborate on?
- Thanks for not being nice. Nothing is ever learned from only positive feedback. Getting called out forces you to rethink and figure out what to do better next time.
    - With that said, it did feel nice for you to say some of my ideas were solid! 


## General takeaways and quality of life improvements as a Rust developer

- Cargo clippy + cargo fmt (on save) + Problems tab on VSCode are must haves
- I had been reluctant to use copilot until I saw your videos. My thoughts were I need to learn to write code myself and copilot would hinder my learning, or provide a false sense of progress.
    - I changed my mind watching it complete boiler plate code as its main purpose. Not getting in the way of learning in that sense, it just helps you type less. It feels like the rust-analyzer extension but better.
- I need to break away from the habit of writing examples as tests. It is just as much work to put them in a test module to be run automatically
- cargo watch is amazing - glad you showed me this
    - `cargo install cargo-watch && cargo install cargo-nextest`
- cargo-nextest is also really great specially coupled with cqrgo watch
    `cargo watch -x 'nextest r'`
- Top level cargo workspace is really helpful for projects


## Builder pattern

Creational patterns are amazing, I have used the builder pattern in Python before and very very similar
- Outsource mutable side of the build process to ServerBuilder then return a Server that is ready to use really cleaned up the creation


```rust
pub struct ServerBuilder {
    handlers: HashMap<request::Request, handler::Handler>,
}

impl ServerBuilder {
    pub fn finalize(
        self,
        addr: impl ToSocketAddrs,
        pool_size: usize,
    ) -> Result<Server, ServerError> {
        let socket_addr = match addr.to_socket_addrs() {
            Ok(addr_iter) => addr_iter,
            Err(_) => panic!("could not resolve socket address"),
        }
        .next()
        .ok_or(ServerError::BadSocketaddr)?;

        let tcp_listener = TcpListener::bind(socket_addr).map_err(ServerError::ServerCreation)?;

        let pool = threadpool::ThreadPool::build(pool_size).map_err(ServerError::PoolSizeError)?;

        let server = Server {
            tcp_listener,
            pool,
            handlers: self.handlers,
        };

        Ok(server)
    }

    pub fn register_handler(mut self, r: request::Request, handler: handler::Handler) -> Self {
        self.handlers.insert(r, handler);
        self
    }

    pub fn register_error_handler(self, handler: handler::Handler) -> Self {
        let request = request::Request::UNIDENTIFIED;
        self.register_handler(request, handler)
    }
}

impl Server {
    pub fn build() -> ServerBuilder {
        ServerBuilder {
            handlers: HashMap::new(),
        }
    }
    pub fn run(&self) {
        /// snipped
    }
}
```

Questions/thoughts:
- Doesnt the `finalize` method violate the single responsibility principle?
    - It is responsible for both validation of the socketaddr and instantiating/returning the Server

## trait bound syntax: taking in an impl ToSocketAddr is a GAME CHANGER
- really cleans up the code on the user side
- it also makes way more sense to do the socket addr validation work in the library as opposed to in the user side
- lastly it shows the difference of trait bound syntax vs generics. They are different things

- Different ways to write trait bound syntax: both do the same thing
```rust
impl ServerBuilder {
    pub fn finalize<T: ToSocketAddrs>(
        self,
        addr: T,
        pool_size: usize,
    ) -> Result<Server, ServerError> {
```

```rust
impl ServerBuilder {
    pub fn finalize(
        self,
        addr: impl ToSocketAddrs,
        pool_size: usize,
    ) -> Result<Server, ServerError> {
```

Questions/thoughts:
- How common is it to see a ToConcreteType blanket implementation?
- **The challenge here is knowing that ToSocketaddr exists**

## Parse request
I've spent days on this function trying to understand how to work with an open Tcp server/client connection

One thing I learned here is that the Read trait has the ability to shoot us in the foot. See commented section

```rust
        // clipped rest of function

        // Find the Content-Length header
        let content_length = request
            .iter()
            // .lines()
            .find(|line| line.starts_with("Content-Length:"))
            .and_then(|line| {
                line.trim()
                    .split(':')
                    .nth(1)
                    .and_then(|value| value.trim().parse::<usize>().ok())
            })
            .unwrap_or(0);

        // Parse the request body based on Content-Length
        // Read to end blocks until the client closes the connection
        // which it will not until the server sends a response
        // thus it will block until client times out
        let mut body_buffer = vec![0; content_length];
        buffer.read_exact(&mut body_buffer)?;

        // Add body to request
        req.add_body(String::from_utf8(body_buffer.clone()).unwrap_or_default());
    };
```

## type definition in handlers.rs

The Handler is a 
```rust
Box<dyn Fn(request::Request) -> response::Response + Send + Sync + 'static>
```

Is there a better way to reuse that definition here?

```rust
    pub fn register_handler(
        mut self,
        r: request::Request,
        handler: impl Fn(request::Request) -> response::Response + Send + Sync + 'static,
    ) -> Self {
        self.handlers.insert(r, Box::new(handler));
        self
    }
```

## impl Trait vs <>

Help me understand
[this section of Rust by example](https://doc.rust-lang.org/reference/types/impl-trait.html#:~:text=That%20is%2C%20impl%20Trait%20in,Trait%20are%20not%20exactly%20equivalent) in this context:

```rust
    pub fn register_handler(
        mut self,
        r: request::Request,
        handler: impl Fn(request::Request) -> response::Response + Send + Sync + 'static,
    ) -> Self {
        self.handlers.insert(r, Box::new(handler));
        self
    }
```
versus
```rust
    pub fn register_handler
    <
        T: Fn(request::Request) -> response::Response + Send + Sync + 'static,
    >(
        mut self,
        r: request::Request,
        handler: T,
    ) -> Self {
        self.handlers.insert(r, Box::new(handler));
        self
    }
```