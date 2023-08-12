// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#[cfg(feature = "ink")]
use obce::ink::ink_prelude::vec::Vec;
#[cfg(feature = "substrate")]
use obce::substrate::sp_std::vec::Vec;
#[cfg(feature = "substrate")]
use obce::substrate::{
    frame_support::traits::PalletInfoAccess,
    CriticalError,
    SupportCriticalError,
};
#[cfg(feature = "substrate")]
use pallet_dao_assets::Error as DaoAssetError;

/// The origin of the call. The smart contract can execute methods on behalf of the `caller` or itself.
#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[cfg_attr(
    feature = "ink",
    derive(ink_storage::traits::SpreadLayout, ink_storage::traits::PackedLayout,)
)]
#[cfg_attr(all(feature = "ink", feature = "std"), derive(ink_storage::traits::StorageLayout))]
pub enum Origin {
    Caller,
    Address,
}

impl Default for Origin {
    fn default() -> Self {
        Self::Address
    }
}

#[cfg(feature = "ink")]
impl ink_storage::traits::SpreadAllocate for Origin {
    fn allocate_spread(_ptr: &mut ink_primitives::KeyPtr) -> Self {
        Self::Address
    }
}

/// The trait describes types used in the chain extension definition. Substrate and ink! side can
/// have its types, so the trait is agnostic.
pub trait Environment {
    type AccountId;
    type AssetId;
    type Balance;
}

#[obce::definition(id = "pallet-dao-assets-chain-extension@v0.1")]
pub trait PalletDaoAssets<T: Environment> {
    fn transfer(
        &mut self,
        origin: Origin,
        id: T::AssetId,
        target: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), Error<T>>;

    fn transfer_keep_alive(
        &mut self,
        origin: Origin,
        id: T::AssetId,
        target: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), Error<T>>;

    fn approve_transfer(
        &mut self,
        origin: Origin,
        id: T::AssetId,
        delegate: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), Error<T>>;

    fn cancel_approval(&mut self, origin: Origin, id: T::AssetId, delegate: T::AccountId) -> Result<(), Error<T>>;

    fn transfer_approved(
        &mut self,
        origin: Origin,
        id: T::AssetId,
        owner: T::AccountId,
        destination: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), Error<T>>;
}

/// The common errors that can be emitted by the `pallet-asset`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error<T> {
    // Errors of chain extension
    /// Only the admin can execute methods on behalf of the `caller`.
    ContractIsNotAdmin,

    // Asset pallet errors
    /// Account balance must be greater than or equal to the transfer amount.
    BalanceLow,
    /// The account to alter does not exist.
    NoAccount,
    /// The signing account has no permission to do the operation.
    NoPermission,
    /// The given asset ID is unknown.
    Unknown,
    /// The asset ID is already taken.
    InUse,
    /// Invalid witness data given.
    BadWitness,
    /// Minimum balance should be non-zero.
    MinBalanceZero,
    /// Invalid metadata given.
    BadMetadata,
    /// No approval exists that would allow the transfer.
    Unapproved,
    /// The source account would not survive the transfer and it needs to stay alive.
    WouldDie,
    /// The asset-account already exists.
    AlreadyExists,
    /// The operation would result in funds being burned.
    WouldBurn,
    /// The asset is not live, and likely being destroyed.
    AssetNotLive,
    /// The asset status is not the expected status.
    IncorrectStatus,
    /// Unknown internal asset pallet error.
    DaoAssetPalletInternal,

    // Substrate errors
    #[cfg(feature = "substrate")]
    /// Critical errors which stop the execution of the chain extension on the substrate level.
    Critical(CriticalError),
    #[doc(hidden)]
    #[codec(skip)]
    /// It is a dummy variant to support unused generics.
    __Ignore(core::marker::PhantomData<T>),
}

#[cfg(feature = "substrate")]
impl<T: pallet_dao_assets::Config> From<CriticalError> for Error<T> {
    fn from(dispatch: CriticalError) -> Self {
        let dao_asset_module = <pallet_dao_assets::Pallet<T> as PalletInfoAccess>::index() as u8;

        // If error from the `pallet_assets` module, map it into ink! error
        if let CriticalError::Module(module) = dispatch {
            if module.index == asset_module {
                let mut input = module.error.as_slice();
                if let Ok(asset_error) = <DaoAssetError<T> as scale::Decode>::decode(&mut input) {
                    return asset_error.into()
                }
            }
        }

        Error::Critical(dispatch)
    }
}

#[cfg(feature = "substrate")]
impl<T> From<DaoAssetError<T>> for Error<T> {
    fn from(asset: DaoAssetError<T>) -> Self {
        match asset {
            DaoAssetError::<T>::BalanceLow => Error::<T>::BalanceLow,
            DaoAssetError::<T>::NoAccount => Error::<T>::NoAccount,
            DaoAssetError::<T>::NoPermission => Error::<T>::NoPermission,
            DaoAssetError::<T>::Unknown => Error::<T>::Unknown,
            DaoAssetError::<T>::InUse => Error::<T>::InUse,
            DaoAssetError::<T>::BadWitness => Error::<T>::BadWitness,
            DaoAssetError::<T>::MinBalanceZero => Error::<T>::MinBalanceZero,
            DaoAssetError::<T>::BadMetadata => Error::<T>::BadMetadata,
            DaoAssetError::<T>::Unapproved => Error::<T>::Unapproved,
            DaoAssetError::<T>::WouldDie => Error::<T>::WouldDie,
            DaoAssetError::<T>::AlreadyExists => Error::<T>::AlreadyExists,
            DaoAssetError::<T>::WouldBurn => Error::<T>::WouldBurn,
            DaoAssetError::<T>::AssetNotLive => Error::<T>::AssetNotLive,
            DaoAssetError::<T>::IncorrectStatus => Error::<T>::IncorrectStatus,
            _ => Error::<T>::DaoAssetPalletInternal,
        }
    }
}

#[cfg(feature = "substrate")]
impl<T> SupportCriticalError for Error<T> {
    fn try_to_critical(self) -> Result<CriticalError, Self> {
        match self {
            Error::<T>::Critical(error) => Ok(error),
            _ => Err(self),
        }
    }
}
