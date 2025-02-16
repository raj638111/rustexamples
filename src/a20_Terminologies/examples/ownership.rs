
fn main() {
    return_a_string();
}

/*
error[E0106]: missing lifetime specifier
 --> src/a20_Terminologies/examples/ownership.rs:6:25
  |
6 | fn return_a_string() -> &String {
  |                         ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
  |
6 | fn return_a_string() -> &'static String {
  |                          +++++++
help: instead, you are more likely to want to return an owned value
  |
6 - fn return_a_string() -> &String {
6 + fn return_a_string() -> String {
  |
 */
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}