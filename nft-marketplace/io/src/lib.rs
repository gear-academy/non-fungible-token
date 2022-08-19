#![no_std]
use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
pub use nft_io::DelegatedApproveMessage;
use primitive_types::U256;
use scale_info::TypeInfo;

pub type ContractId = ActorId;
pub type TokenId = U256;
pub type Price = u128;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitMarket {
    pub admin_id: ActorId,
    pub treasury_id: ActorId,
    pub treasury_fee: u16,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone, PartialEq, Eq)]
pub struct Auction {
    pub bid_period: u64,
    pub started_at: u64,
    pub ended_at: u64,
    pub current_price: Price,
    pub current_winner: ActorId,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct Bid {
    pub id: ActorId,
    pub price: Price,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone, Default, PartialEq, Eq)]
pub struct Item {
    pub owner_id: ActorId,
    pub ft_contract_id: Option<ContractId>,
    pub price: Option<u128>,
    pub auction: Option<Auction>,
    pub offers: BTreeMap<(Option<ContractId>, Price), ActorId>,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct DelegatedApprove {
    pub message: DelegatedApproveMessage,
    pub signature: [u8; 64],
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MarketAction {
    /// Adds NFT contract addresses that can be listed on marketplace.
    ///
    /// # Requirements:
    /// Only admin can add approved NFT accounts.
    ///
    /// # Arguments:
    /// * `nft_contract_id`: the NFT contract address
    ///
    /// On success replies [`MarketEvent::NftContractAdded`].
    AddNftContract(ContractId),

    /// Adds the contract addresses of fungible tokens with which users can pay for NFTs.
    ///
    /// # Requirements:
    /// Only admin can add approved fungible-token accounts.
    ///
    /// # Arguments:
    /// * `ft_contract_id`: the FT contract address
    ///
    /// On success replies [`MarketEvent::FtContractAdded`].
    AddFTContract(ContractId),

    /// Adds data on market item.
    /// If the item of that NFT does not exist on the marketplace then it will be listed.
    /// If the item exists then that action is used to change the price or suspend the sale.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be the NFT owner
    /// * `nft_program_id` in `delegated_approve` must be in the list of `approved_nft_contracts`
    /// * if item already exists, then it cannot be changed if there is an active auction
    ///
    /// Arguments:
    /// * `delegated_approve`: the approve message from user to give rights to marketplace
    /// * `price`: the NFT price (if it is `None` then the item is not on the sale)
    ///
    /// On success replies [`MarketEvent::MarketDataAdded`].
    AddMarketData {
        delegated_approve: DelegatedApprove,
        ft_contract_id: Option<ActorId>,
        price: Option<u128>,
    },

    /// Sells the NFT.
    ///
    /// # Requirements:
    /// * The NFT item must exists and be on sale.
    /// * If the NFT is sold for a native Gear value, then a buyer must attach value equals to the price.
    /// * If the NFT is sold for fungible tokens then a buyer must have enough tokens in the fungible token contract.
    /// * There must be no an opened auction on the item.
    ///
    /// Arguments:
    /// * `nft_contract_id`: NFT contract address
    /// * `token_id`: the token ID
    ///
    /// On success replies [`MarketEvent::ItemSold`].
    BuyItem {
        nft_contract_id: ContractId,
        token_id: TokenId,
    },

    /// Creates an auction for selected item.
    /// If the NFT item doesn't exist on the marketplace then it will be listed
    ///
    /// Requirements:
    /// * Only the item owner can start auction.
    /// * `nft_contract_id` in the `delegated_approve` must be in the list of `approved_nft_contracts`
    /// *  There must be no active auction.
    ///
    /// Arguments:
    /// * `delegated_approve`: the approve message from user to give rights to marketplace
    /// * `ft_contract_id`: the fungible token contract address that can be used for trading
    /// * `min_price`: the starting price
    /// * `bid_period`: the time interval. If the auction ends before `exec::blocktimestamp() + bid_period`
    /// then the auction end time is delayed for `bid_period`.
    ///
    /// On success replies [`MarketEvent::AuctionCreated`].
    CreateAuction {
        delegated_approve: DelegatedApprove,
        ft_contract_id: Option<ContractId>,
        min_price: Price,
        bid_period: u64,
        duration: u64,
    },

    /// Adds a bid to an ongoing auction.
    ///
    /// # Requirements:
    /// * The item must extsts.
    /// * The auction must exists on the item.
    /// * If the NFT is sold for a native Gear value, then a buyer must attach value equals to the price indicated in the arguments.
    /// * If the NFT is sold for fungible tokens then a buyer must have   enough tokens in the fungible token contract.
    /// * `price` must be greater then the current offered price for that item.
    ///
    /// # Arguments
    /// * `nft_contract_id`: the NFT contract address.
    /// * `token_id`: the NFT id.
    /// * `price`: the offered price.
    ///  
    /// On success replies [`MarketEvent::BidAdded`].   
    AddBid {
        nft_contract_id: ContractId,
        token_id: TokenId,
        price: Price,
    },

    /// Settles the auction.
    ///
    /// Requirements:
    /// * The auction must be over.
    ///
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    /// * `token_id`: the NFT id
    ///   
    /// On successful auction replies [`MarketEvent::AuctionSettled`].
    /// If no bids were made replies [`MarketEvent::AuctionCancelled`].
    SettleAuction {
        nft_contract_id: ContractId,
        token_id: TokenId,
    },

    /// Adds a price offer to the item.
    ///
    /// Requirements:
    /// * NFT item must exists and be listed on the marketplace.
    /// * There must be no an ongoing auction on the item.
    /// * If a user makes an offer in native Gear value, then he must attach value equals to the price indicated in the arguments.
    /// * If a user makes an offer in fungible tokens then he must have  enough tokens in the fungible token contract.
    /// * The price can not be equal to 0.
    /// * There must be no identical offers on the item.
    ///
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    /// * `ft_contract_id`: the FT contract address
    /// * `token_id`: the NFT id
    /// * `price`: the offer price
    ///     
    /// On success replies [`MarketEvent::OfferAdded`].
    AddOffer {
        nft_contract_id: ContractId,
        ft_contract_id: Option<ContractId>,
        token_id: TokenId,
        price: Price,
    },

    /// Withdraws tokens.
    ///
    /// Requirements:
    /// * NFT item must exists and be listed on the marketplace.
    /// * Only the offer creator can withdraw his tokens.
    /// * The offer with indicated hash must exist.
    ///
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    /// * `token_id`: the NFT id
    /// * `offer_hash`: the offer hash that includes the offer price and the address of fungible token contract.
    ///
    /// On success replies [`MarketEvent::TokensWithdrawn`].
    Withdraw {
        nft_contract_id: ContractId,
        token_id: TokenId,
        ft_contract_id: Option<ContractId>,
        price: Price,
    },

    /// Accepts an offer.
    ///
    /// Requirements:
    /// * NFT item must exists and be listed on the marketplace.
    /// * Only owner can accept offer.
    /// * There must be no ongoing auction.
    /// * The offer with indicated hash must exist.
    ///
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    /// * `token_id`: the NFT id
    /// * `offer_hash`: the offer hash that includes the offer price and the address of fungible token contract.
    ///      
    /// On success replies [`MarketEvent::OfferAccepted`].
    AcceptOffer {
        nft_contract_id: ContractId,
        token_id: TokenId,
        ft_contract_id: Option<ContractId>,
        price: Price,
    },

    Item {
        nft_contract_id: ContractId,
        token_id: TokenId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MarketEvent {
    NftContractAdded(ContractId),
    FtContractAdded(ContractId),
    MarketDataAdded {
        nft_contract_id: ContractId,
        ft_contract_id: Option<ContractId>,
        token_id: TokenId,
        price: Option<u128>,
    },
    ItemSold {
        owner: ActorId,
        nft_contract_id: ContractId,
        token_id: TokenId,
    },
    BidAdded {
        nft_contract_id: ContractId,
        token_id: TokenId,
        price: Price,
    },
    AuctionCreated {
        nft_contract_id: ContractId,
        token_id: TokenId,
        price: Price,
    },
    AuctionSettled {
        nft_contract_id: ContractId,
        winner: ActorId,
        token_id: TokenId,
        price: Price,
    },
    AuctionCancelled {
        nft_contract_id: ContractId,
        token_id: TokenId,
    },
    NFTListed {
        nft_contract_id: ContractId,
        owner: ActorId,
        token_id: TokenId,
        price: Option<Price>,
    },
    ItemInfo(Item),
    OfferAdded {
        nft_contract_id: ContractId,
        ft_contract_id: Option<ContractId>,
        token_id: TokenId,
        price: Price,
    },
    OfferAccepted {
        nft_contract_id: ContractId,
        token_id: TokenId,
        new_owner: ActorId,
        price: Price,
    },
    TokensWithdrawn {
        nft_contract_id: ContractId,
        token_id: TokenId,
        ft_contract_id: Option<ContractId>,
        price: Price,
    },
}
