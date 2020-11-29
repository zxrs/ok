#[macro_export]
macro_rules! ok {
    ($val:expr) => {
        match $val {
            Some(v) => Ok(v),
            None => Err(anyhow!(
                "[{}/{}] {} is none.",
                file!(),
                line!(),
                stringify!($val)
            )),
        }
    };
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    #[test]
    fn ok() {
        let a: Result<i32> = ok!(Some(1));
        assert!(a.is_ok());
    }

    #[test]
    fn err() {
        let a: Result<i32> = ok!(None);
        assert!(a.is_err());
    }
}
