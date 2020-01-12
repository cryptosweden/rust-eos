use crate::{ParseNameError, ParseAssetError};
//use keys;
use hex;

#[derive(Debug)]
pub enum Error {
    ParseNameErr(ParseNameError),
    ParseAssetErr(ParseAssetError),
    //SignErr(keys::error::Error),
    FromHexError(hex::FromHexError),
}
