pub mod config {
    pub trait ConfigType: Sized {
        fn doc_hint() -> String;
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
mod tests {
    use config_proc_macro::config_type;

    #[config_type]
    enum Bar {
        Foo,
        Bar,
        #[doc_hint = "foo_bar"]
        FooBar,
        FooFoo(i32),
    }
}
