use std::any::Any;

use who_cares::WhoCares;

#[test]
fn test_function_can_return_who_cares() -> WhoCares<()> {
    let answer = "answer=42".strip_prefix("answer=")?;
    let value = answer.parse::<u8>()?;

    assert_eq!(value, 42);
    WhoCares(())
}

#[test]
fn who_cares_module_path_is_available() -> WhoCares<()> {
    let answer = "answer=42".strip_prefix("answer=")?;
    let value = answer.parse::<u8>()?;

    assert_eq!(value, 42);
    WhoCares(())
}

#[test]
fn result_err_panics_with_debug_value() {
    let panic = std::panic::catch_unwind(|| -> WhoCares<()> {
        let value: Result<(), _> = Err("你好");
        value?;
        WhoCares(())
    })
    .expect_err("Err should panic");

    let message = panic_message(panic.as_ref());
    assert_eq!(message, "called WhoCares `?` on an `Err` value: \"你好\"");
}

#[test]
fn option_none_panics() {
    let panic = std::panic::catch_unwind(|| -> WhoCares<()> {
        let _answer = "status=ready".strip_prefix("answer=")?;
        WhoCares(())
    })
    .expect_err("None should panic");

    let message = panic_message(panic.as_ref());
    assert_eq!(message, "called WhoCares `?` on a `None` value");
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
