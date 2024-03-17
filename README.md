# Interview tasks

Feel free to use/share the tasks in [TASKS.md](/TASKS.md) as you wish.

These tasks are intended to be simple interview tasks to test applicants.

## Some information about the tasks

### Estimated time

The tasks take about an hour or two to implement.

### Programming language

The tasks were made with [rust](https://www.rust-lang.org/) in mind, but other languages works fine as well.

## My implementation

I implemented the tasks because I wanted to see if they covered my intended areas and weren't too complicated.

### Running my implementation

#### Client/Server

Start the server

```sh
cargo run -p server
```

or the async server

```sh
cargo run -p server-async
```

Run a client

```sh
cargo run -p client -- 127.0.0.1:3000
```

Change the port to 3001 or 3002 to test the other server connections.

#### Converter

Run the converter

```sh
cargo run -p converter
```
