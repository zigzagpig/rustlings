// errors4.rs
// 执行 `rustlings hint errors4` 或在观察模式下使用 `hint` 子命令来获取提示。
fn main() {
    println!("Hello {}!", "world");
}
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // 嗯。。为什么这只返回了一个 Ok 值？
        match value {
            v if v > 0 => Ok(PositiveNonzeroInteger(v as u64)),
            0 => Err(CreationError::Zero),
            _ => Err(CreationError::Negative),
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
