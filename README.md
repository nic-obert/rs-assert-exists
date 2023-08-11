# assert_exists

Useful macro to assert that a certain symbol exists in the source code.  
This is useful to strictly couple otherwise loosely coupled functions and symbols that depend on each other.

## Usage

```rust
fn prepare_for_my_variant() {
    assert_exists!(MyEnum::MyVariant);
    // ...
}
```

## License

Licensed under MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>)
