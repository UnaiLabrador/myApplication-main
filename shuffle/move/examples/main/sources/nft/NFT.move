module Sender::NFT {
    use Std::GUID::{Self, GUID};
    use Std::Signer;
    use Std::Vector;
    use Std::Option::{Self, Option};
    use Std::Errors;

    /// Errors
    const EID_NOT_FOUND: u64 = 0;
    const EID_EXISTS: u64 = 1;
    const ENFT_COLLECTION_NOT_PUBLISHED: u64 = 2;

    /// A non-fungible token of a specific `NFTType`, created by `id.addr`.
    /// Anyone can create a `NFT`. The access control policy for creating an `NFT<Type>` should be defined in the
    /// logic for creating `Type`. For example, if only Michelangelo should be able to  create `NFT<MikePainting>`,
    /// the `MikePainting` type should only be creatable by Michelangelo's address.
    struct NFT<NFTType: store + drop> has key, store {
        /// A globally unique identifier, which includes the address of the NFT
        /// creator (who may or may not be the same as the content creator). Immutable.
        id: GUID,
        /// A struct to enable type-specific fields that will be different for each Token.
        /// For example, `NFT<Painting>` with
        /// `struct Painting { name: vector<u84, painter: vector<u8>, year: u64, ... }`,
        /// Or, `NFT<DigitalPirateInGameItem> { item_type: u8, item_power: u8, ... }`. Mutable.
        type: NFTType,
        /// pointer to where the content and metadata is stored. Could be a DiemID domain, IPFS, Dropbox url, etc. Immutable.
        content_uri: vector<u8>,
    }

    struct NFTCollection<NFTType: store + drop> has key {
        nfts: vector<NFT<NFTType>>,
    }

    //    struct MintEvent has copy, drop, store {
    //        id: GUID::ID,
    //        creator: address,
    //        content_uri: vector<u8>,
    //    }

    //    struct TransferEvent has copy, drop, store {
    //        id: GUID::ID,
    //        from: address,
    //        to: address,
    //    }

    //    struct Admin has key {
    //        mint_events: Event::EventHandle<MintEvent>,
    //    }

    public fun initialize<NFTType: store + drop>(account: &signer) {
        if (!exists<NFTCollection<NFTType>>(Signer::address_of(account))) {
            move_to(account, NFTCollection { nfts: Vector::empty<NFT<NFTType>>() });
        };
    }

    public(script) fun initialize_nft_collection<NFTType: store + drop>(account: signer) {
        //        if (!exists<NFTCollection<NFTType>>(Signer::address_of(&account))) {
        //            move_to(&account, NFTCollection { nfts: Vector::empty<NFT<NFTType>>() });
        //        };
        initialize<NFTType>(&account);
    }

    /// Create a `NFT<Type>` that wraps `token`
    public fun create<NFTType: store + drop>(
        account: &signer, type: NFTType, content_uri: vector<u8>
    ): NFT<NFTType> {
        let token_id = GUID::create(account);
        NFT { id: token_id, type, content_uri }
    }

    /// Publish the non-fungible token `nft` under `account`.
    public fun add<NFTType: store + drop>(account: address, nft: NFT<NFTType>) acquires NFTCollection {
        assert!(exists<NFTCollection<NFTType>>(account), Errors::not_published(ENFT_COLLECTION_NOT_PUBLISHED));
        assert!(!has_token<NFTType>(account, &GUID::id(&nft.id)), Errors::already_published(EID_EXISTS));
        let nft_collection = &mut borrow_global_mut<NFTCollection<NFTType>>(account).nfts;
        Vector::push_back(
            nft_collection,
            nft,
        );
    }

    /// Returns whether the owner has a token with given id.
    fun has_token<NFTType: store + drop>(owner: address, token_id: &GUID::ID): bool acquires NFTCollection {
        Option::is_some(&index_of_token(&borrow_global<NFTCollection<NFTType>>(owner).nfts, token_id))
    }

    /// Remove the `NFT<Type>` under `account`
    fun remove<NFTType: store + drop>(owner: address, id: &GUID::ID): NFT<NFTType> acquires NFTCollection {
        let nft_collection = &mut borrow_global_mut<NFTCollection<NFTType>>(owner).nfts;
        let nft_index = index_of_token<NFTType>(nft_collection, id);
        assert!(Option::is_some(&nft_index), Errors::limit_exceeded(EID_NOT_FOUND));
        Vector::remove(nft_collection, Option::extract(&mut nft_index))
    }

    /// Finds the index of token with the given id in the nft_collection.
    fun index_of_token<NFTType: store + drop>(nft_collection: &vector<NFT<NFTType>>, id: &GUID::ID): Option<u64> {
        let i = 0;
        let len = Vector::length(nft_collection);
        while (i < len) {
            if (GUID::id(id<NFTType>(Vector::borrow(nft_collection, i))) == *id) {
                return Option::some(i)
            };
            i = i + 1;
        };
        Option::none()
    }

    /// Return the globally unique identifier of `nft`
    public fun id<NFTType: store + drop>(nft: &NFT<NFTType>): &GUID {
        &nft.id
    }

    /// Return the creator of this NFT
    public fun creator<NFTType: store + drop>(nft: &NFT<NFTType>): address {
        GUID::creator_address(id<NFTType>(nft))
    }

    /// View the underlying token of a NFT
    public fun type<NFTType: store + drop>(nft: &NFT<NFTType>): &NFTType {
        &nft.type
    }

    /// Transfer the non-fungible token `nft` under `account`.
    public(script) fun transfer<NFTType: store + drop>(
        account: signer,
        to: address,
        creator: address,
        creation_num: u64
    ) acquires NFTCollection {
        let owner_address = Signer::address_of(&account);

        // Remove NFT from `owner`'s collection
        let id = GUID::create_id(creator, creation_num);
        let nft = remove<NFTType>(owner_address, &id);

        // Add NFT to `to`'s collection
        add<NFTType>(to, nft);

        // TODO: add event emission
    }
}
