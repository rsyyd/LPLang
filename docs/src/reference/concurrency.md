# Concurrency

## Structured Concurrency
Nurseries ensure all tasks complete or cancel together:

```lp
nursery! {
    spawn task1();
    spawn task2();
}
```

## Channels
```lp
let (tx, rx) = channel::<String>();
tx.send("hello")?;
let msg = rx.recv()?;
```

## Async/Await
```lp
async fn fetch(url: String) -> Result<Response> { ... }
let response = fetch(url).await?;
```