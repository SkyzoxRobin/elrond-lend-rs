elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::library;
use super::storage;

use common_structs::{DebtPosition, IssueData, BORROW_TOKEN_PREFIX, LEND_TOKEN_PREFIX};

const LEND_TOKEN_NAME: &[u8] = b"IntBearing";
const DEBT_TOKEN_NAME: &[u8] = b"DebtBearing";

#[elrond_wasm::module]
pub trait UtilsModule: library::LibraryModule + storage::StorageModule {
    fn prepare_issue_data(&self, prefix: BoxedBytes, ticker: BoxedBytes) -> IssueData {
        let prefixed_ticker = [prefix.as_slice(), ticker.as_slice()].concat();
        let mut issue_data = IssueData {
            name: BoxedBytes::zeros(0),
            ticker: TokenIdentifier::from(BoxedBytes::from(prefixed_ticker)),
            is_empty_ticker: true,
        };

        if prefix == BoxedBytes::from(LEND_TOKEN_PREFIX) {
            let name = [LEND_TOKEN_NAME, ticker.as_slice()].concat();
            issue_data.name = BoxedBytes::from(name.as_slice());
            issue_data.is_empty_ticker = self.lend_token().is_empty();
        } else if prefix == BoxedBytes::from(BORROW_TOKEN_PREFIX) {
            let name = [DEBT_TOKEN_NAME, ticker.as_slice()].concat();
            issue_data.name = BoxedBytes::from(name.as_slice());
            issue_data.is_empty_ticker = self.borrow_token().is_empty();
        }

        issue_data
    }

    fn compute_health_factor(&self) -> u32 {
        1u32
    }

    #[view(getCapitalUtilisation)]
    fn get_capital_utilisation(&self) -> Self::BigUint {
        let reserve_amount = self.reserves(&self.pool_asset().get()).get();
        let borrowed_amount = self.total_borrow().get();

        self.compute_capital_utilisation(&borrowed_amount, &reserve_amount)
    }

    #[view(getDebtInterest)]
    fn get_debt_interest(&self, amount: &Self::BigUint, timestamp: u64) -> SCResult<Self::BigUint> {
        let time_diff = self.get_timestamp_diff(timestamp)?;
        let borrow_rate = self.get_borrow_rate();

        Ok(self.compute_debt(amount, &time_diff.into(), &borrow_rate))
    }

    #[view(getDepositRate)]
    fn get_deposit_rate(&self) -> Self::BigUint {
        let pool_params = self.pool_params().get();
        let capital_utilisation = self.get_capital_utilisation();
        let borrow_rate = self.get_borrow_rate();

        self.compute_deposit_rate(
            &capital_utilisation,
            &borrow_rate,
            &pool_params.reserve_factor,
        )
    }

    #[view(getBorrowRate)]
    fn get_borrow_rate(&self) -> Self::BigUint {
        let pool_params = self.pool_params().get();
        let capital_utilisation = self.get_capital_utilisation();

        self.compute_borrow_rate(
            &pool_params.r_base,
            &pool_params.r_slope1,
            &pool_params.r_slope2,
            &pool_params.u_optimal,
            &capital_utilisation,
        )
    }

    #[view(debtPosition)]
    fn view_debt_position(&self, position_id: BoxedBytes) -> Option<DebtPosition<Self::BigUint>> {
        self.debt_positions().get(&position_id)
    }

    #[view(getPositionInterest)]
    fn get_debt_position_interest(&self, position_id: BoxedBytes) -> SCResult<Self::BigUint> {
        let debt_position = self.debt_positions().get(&position_id).unwrap_or_default();
        self.get_debt_interest(&debt_position.size, debt_position.timestamp)
    }

    fn get_timestamp_diff(&self, timestamp: u64) -> SCResult<u64> {
        let current_time = self.blockchain().get_block_timestamp();
        require!(current_time >= timestamp, "Invalid timestamp");
        Ok(current_time - timestamp)
    }
}
