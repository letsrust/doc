use std::{error::Error, process::Command};

use anyhow::Result;

fn main() -> Result<()> {
    test_fn_params();
    Ok(())
}

// 函数中使用 trait object：作为参数或者返回值

type BoxedError = Box<dyn Error + Send + Sync>;

trait Executor {
    fn run(&self) -> Result<Option<i32>, BoxedError>;
}

struct Shell<'a, 'b> {
    cmd: &'a str,
    args: &'b [&'a str],
}

impl<'a, 'b> Shell<'a, 'b> {
    fn new(cmd: &'a str, args: &'b [&'a str]) -> Self {
        Shell { cmd, args }
    }
}

impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>, BoxedError> {
        let output = Command::new(self.cmd).args(self.args).output()?;
        Ok(output.status.code())
    }
}

// 范型参数
fn execute_generic(cmd: &impl Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

// 使用 trait object: &dyn T
fn execute_trait_object(cmd: &dyn Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

// 使用 trait object: Box<dyn T>
fn execute_trait_object_box(cmd: Box<dyn Executor>) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

fn test_fn_params() {
    let cmd = Shell::new("ls", &[]);
    let result = cmd.run().unwrap();
    assert_eq!(result, Some(0));

    let result = execute_generic(&cmd).unwrap();
    assert_eq!(result, Some(0));

    let result = execute_trait_object(&cmd).unwrap();
    assert_eq!(result, Some(0));

    let result = execute_trait_object_box(Box::new(cmd)).unwrap();
    assert_eq!(result, Some(0));
}

// 返回值使用 trait object
// pub trait CryptoResolver {
// Provide an implementation of the Random trait or None if none available.
//     fn resolve_rng(&self) -> Option<Box<dyn Random>>;

// Provide an implementation of the Dh trait for the given DHChoice or None if unavailable.
//     fn resolve_dh(&self, choice: &DHChoice) -> Option<Box<dyn Dh>>;

// Provide an implementation of the Hash trait for the given HashChoice or None if unavailable.
//     fn resolve_hash(&self, choice: &HashChoice) -> Option<Box<dyn Hash>>;

// Provide an implementation of the Cipher trait for the given CipherChoice or None if unavailable.
//     fn resolve_cipher(&self, choice: &CipherChoice) -> Option<Box<dyn Cipher>>;

// Provide an implementation of the Kem trait for the given KemChoice or None if unavailable
//     #[cfg(feature = "hfs")]
//     fn resolve_kem(&self, _choice: &KemChoice) -> Option<Box<dyn Kem>> {
//         None
//     }
// }
