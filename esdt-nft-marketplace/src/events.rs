elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::auction::{Auction, AuctionType};

#[allow(clippy::too_many_arguments)]
#[elrond_wasm::module]
pub trait EventsModule {
    fn emit_auction_token_event(self, auction_id: u64, auction: Auction<Self::BigUint>) {
        self.auction_token_event(
            &auction.auctioned_token.token_type,
            auction.auctioned_token.nonce,
            auction_id,
            &auction.nr_auctioned_tokens,
            &auction.original_owner,
            &auction.min_bid,
            &auction.max_bid.unwrap_or_default(),
            auction.start_time,
            auction.deadline,
            auction.payment_token.token_type,
            auction.payment_token.nonce,
            auction.auction_type,
            auction.creator_royalties_percentage,
        )
    }

    fn emit_bid_event(self, auction_id: u64, auction: Auction<Self::BigUint>) {
        self.bid_event(
            &auction.auctioned_token.token_type,
            auction.auctioned_token.nonce,
            auction_id,
            &auction.nr_auctioned_tokens,
            &auction.current_winner,
            &auction.current_bid,
        );
    }

    fn emit_end_auction_event(self, auction_id: u64, auction: Auction<Self::BigUint>) {
        self.end_auction_event(
            &auction.auctioned_token.token_type,
            auction.auctioned_token.nonce,
            auction_id,
            &auction.nr_auctioned_tokens,
            &auction.current_winner,
            &auction.current_bid,
        );
    }

    fn emit_buy_sft_event(self, auction_id: u64, auction: Auction<Self::BigUint>) {
        self.buy_sft_event(
            &auction.auctioned_token.token_type,
            auction.auctioned_token.nonce,
            auction_id,
            &auction.current_winner,
            &auction.min_bid,
        );
    }

    fn emit_withdraw_event(self, auction_id: u64, auction: Auction<Self::BigUint>) {
        self.withdraw_event(
            &auction.auctioned_token.token_type,
            auction.auctioned_token.nonce,
            auction_id,
            &auction.nr_auctioned_tokens,
            &auction.original_owner,
        );
    }

    #[event("auction_token_event")]
    fn auction_token_event(
        &self,
        #[indexed] auction_token_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] auctioned_token_amount: &Self::BigUint,
        #[indexed] seller: &Address,
        #[indexed] min_bid: &Self::BigUint,
        #[indexed] max_bid: &Self::BigUint,
        #[indexed] start_time: u64,
        #[indexed] deadline: u64,
        #[indexed] accepted_payment_token: TokenIdentifier,
        #[indexed] accepted_payment_token_nonce: u64,
        #[indexed] auction_type: AuctionType,
        creator_royalties_percentage: Self::BigUint, // between 0 and 10,000
    );

    #[event("bid_event")]
    fn bid_event(
        &self,
        #[indexed] auction_token_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] nr_auctioned_tokens: &Self::BigUint,
        #[indexed] bidder: &Address,
        #[indexed] bid_amount: &Self::BigUint,
    );

    #[event("end_auction_event")]
    fn end_auction_event(
        &self,
        #[indexed] auction_token_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] nr_auctioned_tokens: &Self::BigUint,
        #[indexed] auction_winner: &Address,
        #[indexed] winning_bid_amount: &Self::BigUint,
    );

    #[event("buy_sft_event")]
    fn buy_sft_event(
        &self,
        #[indexed] auction_token_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] buyer: &Address,
        #[indexed] bid_sft_amount: &Self::BigUint,
    );

    #[event("withdraw_event")]
    fn withdraw_event(
        &self,
        #[indexed] auction_token_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] nr_auctioned_tokens: &Self::BigUint,
        #[indexed] seller: &Address,
    );
}
