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

use super::traits::{
    Environment as DaoAssetsEnvironment,
    PalletDaoAssets,
};
use crate::traits::{
    Error,
    Origin,
};
use obce::substrate::{
    frame_support::traits::fungibles::{
        approvals,
        Inspect,
        InspectMetadata,
    },
    frame_system::{
        Config as SysConfig,
        RawOrigin,
    },
    pallet_contracts::{
        chain_extension::Ext,
        Config as ContractConfig,
    },
    sp_core::crypto::UncheckedFrom,
    sp_runtime::traits::StaticLookup,
    sp_std::vec::Vec,
    ExtensionContext,
};
use pallet_dao_assets::Config as DaoAssetConfig;

#[derive(Default)]
pub struct DaoAssetsExtension;

impl<T: SysConfig + DaoAssetConfig + ContractConfig> AssetsEnvironment for T {
    type AccountId = <T as SysConfig>::AccountId;
    type AssetId = <T as DaoAssetConfig>::AssetId;
    type Balance = <T as DaoAssetConfig>::Balance;
}

#[obce::implementation]
impl<'a, 'b, E, T> PalletDaoAssets<T> for ExtensionContext<'a, 'b, E, T, DaoAssetsExtension>
where
    T: SysConfig + DaoAssetConfig + ContractConfig,
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
    E: Ext<T = T>,
    <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
{
    fn transfer(
        &mut self,
        origin: Origin,
        id: T::AssetId,
        target: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), Error<T>> {
        Ok(pallet_dao_assets::Pallet::<T>::transfer(
            self.select_origin(origin)?,
            id,
            target.into(),
            amount,
        )?)
    }

    fn transfer_keep_alive(
        &mut self,
        origin: Origin,
        id: T::AssetId,
        target: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), Error<T>> {
        Ok((pallet_dao_assets::Pallet::<T>::transfer_keep_alive(
            self.select_origin(origin)?,
            id,
            target.into(),
            amount,
        ))?)
    }

    fn approve_transfer(
        &mut self,
        origin: Origin,
        id: T::AssetId,
        delegate: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), Error<T>> {
        Ok(pallet_dao_assets::Pallet::<T>::approve_transfer(
            self.select_origin(origin)?,
            id,
            delegate.into(),
            amount,
        )?)
    }

    fn cancel_approval(&mut self, origin: Origin, id: T::AssetId, delegate: T::AccountId) -> Result<(), Error<T>> {
        Ok(pallet_dao_assets::Pallet::<T>::cancel_approval(
            self.select_origin(origin)?,
            id,
            delegate.into(),
        )?)
    }

    fn transfer_approved(
        &mut self,
        origin: Origin,
        id: T::AssetId,
        owner: T::AccountId,
        destination: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), Error<T>> {
        Ok(pallet_dao_assets::Pallet::<T>::transfer_approved(
            self.select_origin(origin)?,
            id,
            owner.into(),
            destination.into(),
            amount,
        )?)
    }
}

/// Trait with additional helpers functions.
pub trait Internal<T: DaoAssetsEnvironment + SysConfig> {
    /// Returns the `AccountId` of the contract as signed origin.
    fn origin(&mut self) -> T::RuntimeOrigin;

    /// Returns the `AccountId` of the contract as signed origin based on the permission.
    fn select_origin(&mut self, origin: Origin) -> Result<T::RuntimeOrigin, Error<T>>;
}

impl<'a, 'b, E, T> Internal<T> for ExtensionContext<'a, 'b, E, T, DaoAssetsExtension>
where
    T: SysConfig + DaoAssetConfig + ContractConfig,
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
    E: Ext<T = T>,
    <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
{
    fn origin(&mut self) -> T::RuntimeOrigin {
        RawOrigin::Signed(self.env.ext().address().clone()).into()
    }

    fn select_origin(&mut self, origin: Origin) -> Result<T::RuntimeOrigin, Error<T>> {
        let origin = RawOrigin::Signed(match origin {
            Origin::Caller => {
                // TODO: Add check that the contract is admin. Right now `asset-pallet` doesn't have getter for admin.
                // TODO: Return `Error::<T>::ContractIsNotAdmin`
                // let a = pallet_assets::Pallet::<T>::asset();
                self.env.ext().caller().clone()
            }
            Origin::Address => self.env.ext().address().clone(),
        });

        Ok(origin.into())
    }
}
