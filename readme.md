workarounds about formatting [sycamore](https://github.com/sycamore-rs/sycamore)'s `view! {}` and `cloned!()` as rustfmt cannot handle this.

note:
* usage: `sycamore-fmt "path/to/file.rs"`
* very poor performance, mainly caused by hundreds of `rustfmt` calls
* no configurable options, currently
* unstable, might mess up your code
* this repo contains a copy of [sycamore-macro](https://github.com/sycamore-rs/sycamore/tree/master/packages/sycamore-macro) crate because of `` error: `proc-macro` crate types currently cannot export any items other than functions tagged with `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]` ``