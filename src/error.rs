use candid::CandidType;
use serde::Deserialize;
use thiserror::Error;

#[derive(CandidType, Deserialize, Debug, Error)]
pub enum Error {
    #[error("The topic (`{0}`) already Subscribed")]
    RepeatSubscription(String),

    #[error("The chain(`{0}`) already exists")]
    ChainAlreadyExisting(String),
    #[error("The token(`{0}`) already exists")]
    TokenAlreadyExisting(String),

    #[error("not supported proposal")]
    NotSupportedProposal,
    #[error("proposal error: (`{0}`)")]
    ProposalError(String),

    #[error("generate directive error for : (`{0}`)")]
    GenerateDirectiveError(String),

    #[error("the message is malformed and cannot be decoded error")]
    MalformedMessageBytes,
    #[error("unauthorized")]
    Unauthorized,
    #[error("The `{0}` is deactive")]
    DeactiveChain(String),
    #[error("The ticket id (`{0}`) already exists!")]
    AlreadyExistingTicketId(String),
    #[error("Not fount the ticket id (`{0}`) !")]
    NotFoundTicketId(String),
    #[error("The resubmit ticket id must exist!")]
    ResubmitTicketIdMustExist,
    #[error("The resubmit ticket must same as the old ticket!")]
    ResubmitTicketMustSame,
    #[error("The resumit ticket sent too often")]
    ResubmitTicketSentTooOften,
    #[error("not found chain: (`{0}`)")]
    NotFoundChain(String),
    #[error("not found token: (`{0}`)")]
    NotFoundToken(String),
    #[error("not found account(`{0}`) token(`{1}`) on the chain(`{2}`")]
    NotFoundAccountToken(String, String, String),
    #[error("Not found this token(`{0}`) on chain(`{1}`) ")]
    NotFoundChainToken(String, String),
    #[error("Insufficient token (`{0}`) on chain (`{1}`) !)")]
    NotSufficientTokens(String, String),
    #[error("The ticket amount(`{0}`) parse error: `{1}`")]
    TicketAmountParseError(String, String),
    #[error("ecdsa_public_key failed : (`{0}`)")]
    EcdsaPublicKeyError(String),
    #[error("sign_with_ecdsa failed: (`{0}`)")]
    SighWithEcdsaError(String),
    #[error("address(`{0}`) parse error for target chain: `{1}`")]
    InvalidAddress(String, String),

    #[error("custom error: (`{0}`)")]
    CustomError(String),
}
