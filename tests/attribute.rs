use std::any::Any;

use who_cares::who_cares;

#[test]
#[who_cares]
fn attribute_macro_allows_plain_test_body() {
    let answer = "answer=42".strip_prefix("answer=")?;
    let value: u8 = answer.parse()?;

    assert_eq!(value, 42);
}

#[test]
fn attribute_macro_result_err_keeps_debug_message() {
    let panic = std::panic::catch_unwind(|| {
        attribute_macro_result_err_panics();
    })
    .expect_err("Err should panic");

    let message = panic_message(panic.as_ref());
    assert_eq!(message, "called WhoCares `?` on an `Err` value: \"你好\"");
}

#[test]
fn attribute_macro_option_none_keeps_panic_message() {
    let panic = std::panic::catch_unwind(|| {
        attribute_macro_option_none_panics();
    })
    .expect_err("None should panic");

    let message = panic_message(panic.as_ref());
    assert_eq!(message, "called WhoCares `?` on a `None` value");
}

#[who_cares]
fn attribute_macro_result_err_panics() {
    let value: Result<(), _> = Err("你好");
    value?;
}

#[who_cares]
fn attribute_macro_option_none_panics() {
    let _answer = "status=ready".strip_prefix("answer=")?;
}

fn panic_message(panic: &(dyn Any + Send)) -> &str {
    if let Some(message) = panic.downcast_ref::<&str>() {
        message
    } else {
        panic
            .downcast_ref::<String>()
            .expect("panic payload should be a string")
    }
}
