#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_event, decl_module, decl_storage, decl_error, Parameter};
use frame_system::ensure_signed;

use sp_runtime::{
    DispatchResult, RuntimeDebug,
    traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize, Bounded, One, CheckedAdd}
};
use orml_traits::{MultiReservableCurrency, MultiCurrency};


pub trait Trait: frame_system::Trait {
    type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;
    type Currency: MultiReservableCurrency<Self::AccountId>;
    type OrderId: Parameter + AtLeast32BitUnsigned + Default + Copy + MaybeSerializeDeserialize + Bounded;
}

#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct Order<CurrencyId, Balance, AccountId> {
    pub base_currency_id: CurrencyId,
    #[codec(compact)]
    pub base_amount: Balance,
    pub target_currency_id: CurrencyId,
    #[codec(compact)]
    pub target_amount: Balance,
    pub owner: AccountId,
}

type BalanceOf<T> = <<T as Trait>::Currency as MultiCurrency<<T as frame_system::Trait>::AccountId>>::Balance;
type CurrencyIdOf<T> = <<T as Trait>::Currency as MultiCurrency<<T as frame_system::Trait>::AccountId>>::CurrencyId;
type OrderId<T> = Order<CurrencyIdOf<T>, BalanceOf<T>, <T as frame_system::Trait>::AccountId>; 

decl_storage! {
    trait Store for Module<T: Trait> as Exchange {
    }
}

decl_event! {
    pub enum Event {
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;
    }
}

impl<T: Trait> Module<T> {}
