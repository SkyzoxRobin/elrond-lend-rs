#![no_std]

use core::array::FixedSizeArray;

use elrond_wasm::{contract_call, require, sc_error};
mod liquidity_pool_proxy;
use liquidity_pool_proxy::*;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct InterestMetadata<BigUint: BigUintApi> {
	pub timestamp: u64
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct DebtMetadata<BigUint: BigUintApi> {
    pub timestamp: u64,
    pub collateral_amount: BigUint,
    pub collateral_identifier: TokenIdentifier,
    pub colletareal_timestamp: u64
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct RepayPostion<BigUint: BigUintApi> {
    pub identifier: TokenIdentifier,
    pub amount: BigUint,
    pub nonce: u64,
    pub collateral_identifier: TokenIdentifier,
    pub collateral_amount: BigUint,
    pub collateral_timestamp: u64
}


#[elrond_wasm_derive::contract(LendingPoolImpl)]
pub trait LendingPool {
    #[init]
    fn init(&self) {}

    #[storage_set("debug")]
    fn set_debug(&self, value: u8);

    #[view]
    #[storage_get("debug")]
    fn get_debug(&self) -> u8;

    #[payable("*")]
    #[endpoint]
    fn deposit(
        &self,
        #[var_args] caller: OptionalArg<Address>,
        #[payment_token] asset: TokenIdentifier,
        #[payment] amount: BigUint,
    ) -> SCResult<()> {
        
        let initial_caller = caller.into_option().unwrap_or(self.get_caller());

        require!(amount > 0, "amount must be greater than 0");
        require!(!initial_caller.is_zero(), "invalid address provided");
        require!(self.pools_map().contains_key(&asset.clone()), "asset not supported");

        let pool_address = self
            .pools_map()
            .get(&asset.clone())
            .unwrap_or(Address::zero());

        require!(!pool_address.is_zero(), "invalid liquidity pool address");
        
        let mut args = ArgBuffer::new();
        args.push_argument_bytes(asset.as_esdt_identifier());
        args.push_argument_bytes(amount.to_bytes_be().as_slice());
        args.push_argument_bytes(b"deposit_asset");
        args.push_argument_bytes(initial_caller.as_bytes());

        self.send().execute_on_dest_context_raw(
            self.get_gas_left(), 
            &pool_address,
            &BigUint::zero(), 
            ESDT_TRANSFER_STRING, 
            &args
        );

        
        Ok(())
    }

    #[payable("*")]
    #[endpoint]
    fn withdraw(
        &self,
        asset_to_withdraw: TokenIdentifier,
        #[var_args] initial_caller: OptionalArg<Address>,
        #[paymnet_token] lend_token: TokenIdentifier,
        #[payment] amount: BigUint
    ) -> SCResult<()> {
        let caller = initial_caller.into_option().unwrap_or(self.get_caller());

        require!(amount > 0, "amount must be greater than 0");
        require!(!caller.is_zero(), "invalid address provided");
        require!(self.pools_map().contains_key(&asset_to_withdraw.clone()), "asset not supported");


        let asset_token_pool_address = self
            .pools_map()
            .get(&asset_to_withdraw.clone())
            .unwrap_or(Address::zero());

        require!(!asset_token_pool_address.is_zero(), "invalid liquidity pool address");

        let mut args = ArgBuffer::new();
        args.push_argument_bytes(lend_token.as_esdt_identifier());
        args.push_argument_bytes(amount.to_bytes_be().as_slice());
        args.push_argument_bytes(b"withdraw");
        args.push_argument_bytes(caller.as_bytes());

        self.send().execute_on_dest_context_raw(
            self.get_gas_left(), 
            &asset_token_pool_address,
            &BigUint::zero(), 
            ESDT_TRANSFER_STRING, 
            &args
        );

        Ok(())
    }

    #[payable("*")]
    #[endpoint(lockBTokens)]
    fn lock_b_tokens(
        &self,
        asset_to_repay: TokenIdentifier,
        #[var_args] caller: OptionalArg<Address>,
        #[payment_token] borrow_token: TokenIdentifier,
        #[payment] amount: BigUint
    ) -> SCResult<()>{
        let initial_caller = caller.into_option().unwrap_or(self.get_caller());

        require!(amount > 0, "amount must be greater than 0");
        require!(!initial_caller.is_zero(), "invalid address provided");

        asset_address = self.get_pool_address(asset_to_repay);

        require!(self.pools_map().contains_key(&asset_to_repay.clone()), "asset not supported");


        nft_nonce = self.call_value().esdt_token_nonce();

        let mut args = ArgBuffer::new();
        args.push_argument_bytes(borrow_token.as_esdt_identifier());
        args.push_argument_bytes(amount.to_bytes_be().as_slice());
        args.push_argument_bytes(b"lockBTokens");
        args.push_argument_bytes(caller.as_bytes());


        self.send().execute_on_dest_context_raw(
            self.get_gas_left(), 
            &asset_address,
            &BigUint::zero(), 
            ESDT_TRANSFER_STRING, 
            &args
        );
        


        Ok(())
    }

    #[payable("*")]
    #[endpoint]
    fn repay(
        &self,
        repay_unique_id : H256,
        #[var_args] caller: OptionalArg<Address>,
        #[payment_token] asset: TokenIdentifier,
        #[payment] amount: BigUint
    ) -> SCResult<()> {

        let caller = initial_caller.into_option().unwrap_or(self.get_caller());

        require!(amount > 0, "amount must be greater than 0");
        require!(!caller.is_zero(), "invalid address provided");
        require!(self.pools_map().contains_key(&asset.clone()), "asset not supported");

        asset_address = self.get_pool_address(asset);

        let results: RepayPostion<BigUint>= contract_call!(self, asset_address, LiquidtyPool)
                                    .with_token_transfer(asset, amount)
                                    .repay(self.get_caller(), repay_unique_id, asset,amount)
                                    .execute_on_dest_context(self.get_gas_left(), self.send());
        

        let collateral_token_address = self.pools_map()
                                                .get(&results.collateral_identifier.clone())
                                                .unwrap_or(BigUint::zero());

        require!(collateral_token_address != BigUint::zero(), "asset is not supported");

        let mut args_mint_lend = ArgBuffer::new();
        args_mint_lend.push_argument_bytes(results.collateral_identifier.as_esdt_identifier());
        args_mint_lend.push_argument_bytes(results.collateral_amount.to_bytes_be().as_slice());
        args_mint_lend.push_argument_bytes(b"mintLTokens");
        args_mint_lend.push_argument_bytes(caller.as_bytes());
        args_mint_lend.push_argument_bytes(results.collateral_timestamp.as_ne_bytes().as_slice());

        self.send().execute_on_dest_context_raw(
            self.get_gas_left(), 
            &collateral_token_address,
            &BigUint::zero(), 
            ESDT_TRANSFER_STRING, 
            &args_mint_lend
        );

        Ok()
    }

    #[payable("*")]
    #[endpoint]
    fn borrow(
        &self,
        asset_to_put_as_collateral: TokenIdentifier,
        asset_to_borrow: TokenIdentifier,
        #[var_args] caller: OptionalArg<Address>,
        #[payment_token] asset_collateral_lend_token: TokenIdentifier,
        #[payment] amount: BigUint,
    ) -> SCResult<()> {
        
        let initial_caller = caller.into_option().unwrap_or(self.get_caller());

        require!(amount > 0, "amount must be greater than 0");
        require!(!initial_caller.is_zero(), "invalid address provided");
        require!(self.pools_map().contains_key(&asset_to_put_as_collateral.clone()), "asset not supported");
        require!(self.pools_map().contains_key(&asset_to_borrow.clone()), "asset not supported");

        let collateral_token_pool_address = self
            .pools_map()
            .get(&asset_to_put_as_collateral.clone())
            .unwrap_or(Address::zero());

        let borrow_token_pool_address = self
            .pools_map()
            .get(&asset_to_borrow.clone())
            .unwrap_or(Address::zero());


        require!(!collateral_token_pool_address.is_zero(), "invalid liquidity pool address");
        require!(!borrow_token_pool_address.is_zero(), "invalid liquidity pool address");


        let mut args_add_collateral = ArgBuffer::new();
        args_add_collateral.push_argument_bytes(asset_collateral_lend_token.as_esdt_identifier());
        args_add_collateral.push_argument_bytes(amount.to_bytes_be().as_slice());
        args_add_collateral.push_argument_bytes(b"addCollateral");
        


        self.send().execute_on_dest_context_raw(
            self.get_gas_left(), 
            &collateral_token_pool_address,
            &BigUint::zero(), 
            ESDT_TRANSFER_STRING, 
            &args_add_collateral
        );
        
        let mut args_borrow = ArgBuffer::new();
        args_borrow.push_argument_bytes(amount.to_bytes_be().as_slice());
        args_borrow.push_argument_bytes(b"borrow");
        args_borrow.push_argument_bytes(initial_caller.as_bytes());


        self.send().execute_on_dest_context_raw(
            self.get_gas_left(), 
            &collateral_token_pool_address,
            &BigUint::zero(), 
            ESDT_TRANSFER_STRING, 
            &args_borrow
        );

        Ok(())
    }

    #[callback]
    fn deposit_callback(&self, #[call_result] result: AsyncCallResult<()>) {
        match result {
            AsyncCallResult::Ok(_) => {}
            AsyncCallResult::Err(_) => {}
        }
    }

    #[endpoint(setPoolAddress)]
    fn set_pool_address(
        &self, 
        base_asset: TokenIdentifier, 
        pool_address: Address
    ) -> SCResult<()> {

        require!(
            !self.pools_map().contains_key(&base_asset.clone()),
            "asset already supported"
        );

        self.pools_map().insert(base_asset, pool_address);

        Ok(())
    }

    #[view(getPoolAddress)]
    fn get_pool_address(&self, base_asset: TokenIdentifier) -> SCResult<Address> {
        Ok(self.pools_map().get(&base_asset).unwrap_or(Address::zero()))
    }

    #[storage_mapper("pools_map")]
    fn pools_map(&self) -> MapMapper<Self::Storage, TokenIdentifier, Address>;
}
