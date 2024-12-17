use crate::{
	Capture, Context, CreateScheme, ExitError, ExitFatal, ExitReason, Machine, Opcode, Stack,
};
use alloc::vec::Vec;
use primitive_types::{H160, H256, U256};

/// Transfer from source to target, with given value.
#[derive(Clone, Debug)]
pub struct Transfer {
	/// Source address.
	pub source: H160,
	/// Target address.
	pub target: H160,
	/// Transfer value.
	pub value: U256,
}

/// EVM context handler.
#[auto_impl::auto_impl(&mut, Box)]
pub trait Handler {
	/// Type of `CREATE` interrupt.
	type CreateInterrupt;
	/// Feedback value for `CREATE` interrupt.
	type CreateFeedback;
	/// Type of `CALL` interrupt.
	type CallInterrupt;
	/// Feedback value of `CALL` interrupt.
	type CallFeedback;
	/// The runtime error.
	type RuntimeError: From<ExitFatal> + Into<ExitReason>;

	/// Get balance of address.
	fn balance(&self, address: H160) -> Result<U256, Self::RuntimeError>;
	/// Get code size of address.
	fn code_size(&self, address: H160) -> Result<U256, Self::RuntimeError>;
	/// Get code hash of address.
	fn code_hash(&self, address: H160) -> Result<H256, Self::RuntimeError>;
	/// Get code of address.
	fn code(&self, address: H160) -> Result<Vec<u8>, Self::RuntimeError>;
	/// Get storage value of address at index.
	fn storage(&self, address: H160, index: H256) -> Result<H256, Self::RuntimeError>;
	/// Get original storage value of address at index.
	fn original_storage(&self, address: H160, index: H256) -> Result<H256, Self::RuntimeError>;

	/// Get the gas left value.
	fn gas_left(&self) -> Result<U256, Self::RuntimeError>;
	/// Get the gas price value.
	fn gas_price(&self) -> Result<U256, Self::RuntimeError>;
	/// Get execution origin.
	fn origin(&self) -> Result<H160, Self::RuntimeError>;
	/// Get environmental block hash.
	fn block_hash(&self, number: U256) -> Result<H256, Self::RuntimeError>;
	/// Get environmental block number.
	fn block_number(&self) -> Result<U256, Self::RuntimeError>;
	/// Get environmental coinbase.
	fn block_coinbase(&self) -> Result<H160, Self::RuntimeError>;
	/// Get environmental block timestamp.
	fn block_timestamp(&self) -> Result<U256, Self::RuntimeError>;
	/// Get environmental block difficulty.
	fn block_difficulty(&self) -> Result<U256, Self::RuntimeError>;
	/// Get environmental gas limit.
	fn block_gas_limit(&self) -> Result<U256, Self::RuntimeError>;
	/// Environmental block base fee.
	fn block_base_fee_per_gas(&self) -> Result<U256, Self::RuntimeError>;
	/// Get environmental chain ID.
	fn chain_id(&self) -> Result<U256, Self::RuntimeError>;

	/// Check whether an address exists.
	fn exists(&self, address: H160) -> Result<bool, Self::RuntimeError>;
	/// Check whether an address has already been deleted.
	fn deleted(&self, address: H160) -> Result<bool, Self::RuntimeError>;
	/// Checks if the address or (address, index) pair has been previously accessed
	/// (or set in `accessed_addresses` / `accessed_storage_keys` via an access list
	/// transaction).
	/// References:
	/// * <https://eips.ethereum.org/EIPS/eip-2929>
	/// * <https://eips.ethereum.org/EIPS/eip-2930>
	fn is_cold(&self, address: H160, index: Option<H256>) -> Result<bool, Self::RuntimeError>;

	/// Set storage value of address at index.
	fn set_storage(
		&mut self,
		address: H160,
		index: H256,
		value: H256,
	) -> Result<(), Self::RuntimeError>;
	/// Create a log owned by address with given topics and data.
	fn log(
		&mut self,
		address: H160,
		topics: Vec<H256>,
		data: Vec<u8>,
	) -> Result<(), Self::RuntimeError>;
	/// Mark an address to be deleted, with funds transferred to target.
	fn mark_delete(&mut self, address: H160, target: H160) -> Result<(), Self::RuntimeError>;
	/// Invoke a create operation.
	fn create(
		&mut self,
		caller: H160,
		scheme: CreateScheme,
		value: U256,
		init_code: Vec<u8>,
		target_gas: Option<u64>,
	) -> Result<
		Capture<(ExitReason, Option<H160>, Vec<u8>), Self::CreateInterrupt>,
		Self::RuntimeError,
	>;
	/// Feed in create feedback.
	fn create_feedback(
		&mut self,
		_feedback: Self::CreateFeedback,
	) -> Result<(), Self::RuntimeError> {
		Ok(())
	}
	/// Invoke a call operation.
	fn call(
		&mut self,
		code_address: H160,
		transfer: Option<Transfer>,
		input: Vec<u8>,
		target_gas: Option<u64>,
		is_static: bool,
		context: Context,
	) -> Result<Capture<(ExitReason, Vec<u8>), Self::CallInterrupt>, Self::RuntimeError>;
	/// Feed in call feedback.
	fn call_feedback(&mut self, _feedback: Self::CallFeedback) -> Result<(), Self::RuntimeError> {
		Ok(())
	}

	/// Pre-validation step for the runtime.
	fn pre_validate(
		&mut self,
		context: &Context,
		opcode: Opcode,
		stack: &Stack,
	) -> Result<(), Self::RuntimeError>;
	/// Handle other unknown external opcodes.
	fn other(&mut self, opcode: Opcode, _stack: &mut Machine) -> Result<(), Self::RuntimeError> {
		Err(ExitFatal::CallErrorAsFatal(ExitError::InvalidCode(opcode)).into())
	}
}
