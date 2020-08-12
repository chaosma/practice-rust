use thiserror::Error;

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("invalid format of Simple command parameter")]
    SimpleCmdError,
    #[error("invalid format of Coins command parameter")]
    CoinsCmdError,
}
