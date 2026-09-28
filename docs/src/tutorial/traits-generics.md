# Traits & Generics

```lp
trait Display {
    fn fmt(&self) -> String;
}

impl Display for User {
    fn fmt(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }
}

fn print_all<T: Display>(items: Vec<T>) {
    for item in items { print(item.fmt()) }
}
```