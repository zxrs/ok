pub use anyhow::anyhow;

#[macro_export]
macro_rules! ok {
    ($val:expr) => {
        match $val {
            Some(v) => Ok(v),
            None => Err($crate::anyhow!(
                "[{}:{}] {} is None.",
                file!(),
                line!(),
                stringify!($val)
            ))
        }
    };
}

#[macro_export]
macro_rules! err {
    ($val:expr) => {
        match $val {
            Some(v) => Err($crate::anyhow!(
                "[{}:{}] {} is Some({:?}).",
                file!(),
                line!(),
                stringify!($val),
                v
            )),
            None => Ok(())
        }
    };
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn ok() {
        let a: Result<i32> = ok!(Some(1));
        assert!(a.is_ok());

        let b = err!(Option::<i32>::None);
        assert!(b.is_ok());
    }

    #[test]
    fn err() {
        let a: Result<i32> = ok!(None);
        assert!(a.is_err());

        let b = err!(Some(1));
        assert!(b.is_err());
    }
}
