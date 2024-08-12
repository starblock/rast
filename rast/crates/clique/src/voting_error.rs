use ethereum_types::{Address, H64, H160, H256, U256};
use std::fmt;
use unexpected::{Mismatch, OutOfBounds};
// use std::error;

/// Voting errors.
#[derive(Debug)]
pub enum EngineError {
	/// Signature or author field does not belong to an authority.
	NotAuthorized(Address),
	/// The same author issued different votes at the same step.
	DoubleVote(Address),
	/// The received block is from an incorrect proposer.
	NotProposer(Mismatch<Address>),
	/// Message was not expected.
	UnexpectedMessage,
	/// Seal field has an unexpected size.
	BadSealFieldSize(OutOfBounds<usize>),
	/// Validation proof insufficient.
	InsufficientProof(String),
	/// Failed system call.
	FailedSystemCall(String),
	/// Malformed consensus message.
	MalformedMessage(String),
	/// Requires client ref, but none registered.
	RequiresClient,
	/// Invalid engine specification or implementation.
	InvalidEngine,
	/// Requires signer ref, but none registered.
	RequiresSigner,
	/// Missing Parent Epoch
	MissingParent,
	/// Checkpoint is missing
	CliqueMissingCheckpoint(H256),
	/// Missing vanity data
	CliqueMissingVanity,
	/// Missing signature
	CliqueMissingSignature,
	/// Missing signers
	CliqueCheckpointNoSigner,
	/// List of signers is invalid
	CliqueCheckpointInvalidSigners(usize),
	/// Wrong author on a checkpoint
	CliqueWrongAuthorCheckpoint(Mismatch<Address>),
	/// Wrong checkpoint authors recovered
	CliqueFaultyRecoveredSigners(Vec<String>),
	/// Invalid nonce (should contain vote)
	CliqueInvalidNonce(H64),
	/// The signer signed a block to recently
	CliqueTooRecentlySigned(Address),
	/// Custom
	Custom(String),
}

impl fmt::Display for EngineError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use self::EngineError::*;
		let msg = match *self {
			CliqueMissingCheckpoint(ref hash) => format!("Missing checkpoint block: {}", hash),
			CliqueMissingVanity => format!("Extra data is missing vanity data"),
			CliqueMissingSignature => format!("Extra data is missing signature"),
			CliqueCheckpointInvalidSigners(len) => format!("Checkpoint block list was of length: {} of checkpoint but
															it needs to be bigger than zero and a divisible by 20", len),
			CliqueCheckpointNoSigner => format!("Checkpoint block list of signers was empty"),
			CliqueInvalidNonce(ref mis) => format!("Unexpected nonce {} expected {} or {}", mis, 0_u64, u64::max_value()),
			CliqueWrongAuthorCheckpoint(ref oob) => format!("Unexpected checkpoint author: {}", oob),
			CliqueFaultyRecoveredSigners(ref mis) => format!("Faulty recovered signers {:?}", mis),
			CliqueTooRecentlySigned(ref address) => format!("The signer: {} has signed a block too recently", address),
			Custom(ref s) => s.clone(),
			DoubleVote(ref address) => format!("Author {} issued too many blocks.", address),
			NotProposer(ref mis) => format!("Author is not a current proposer: {}", mis),
			NotAuthorized(ref address) => format!("Signer {} is not authorized.", address),
			UnexpectedMessage => "This Engine should not be fed messages.".into(),
			BadSealFieldSize(ref oob) => format!("Seal field has an unexpected length: {}", oob),
			InsufficientProof(ref msg) => format!("Insufficient validation proof: {}", msg),
			FailedSystemCall(ref msg) => format!("Failed to make system call: {}", msg),
			MalformedMessage(ref msg) => format!("Received malformed consensus message: {}", msg),
			RequiresClient => format!("Call requires client but none registered"),
			RequiresSigner => format!("Call requires signer but none registered"),
			InvalidEngine => format!("Invalid engine specification or implementation"),
			MissingParent => format!("Parent Epoch is missing from database"),
		};

		f.write_fmt(format_args!("Engine error ({})", msg))
	}
}

impl error::Error for EngineError {
	fn description(&self) -> &str {
		"Engine error"
	}
}
