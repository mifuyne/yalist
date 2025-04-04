# Generating widgets with for loop
Useful for listing all the checklist items on load

```rust
for idx in 0..=3 {
    let mut but = Button::default()
        .with_label(&format!("test {}", idx))
        .with_size(list_view.width(), 22);
    list_view.add(&but);
    list_view.fixed(&but, 50);
}
```