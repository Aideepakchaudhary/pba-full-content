// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

#[cfg(test)]
mod tests {
	use xcm_simulator_for_exercises::{
		relay_chain,
		parachain,
		MockNet,
		Relay, RelayChainPalletXcm,
		ParaA, ParachainPalletXcm,
		ParaB,
		TestExt,
	};

	use frame_support::assert_ok;
	use xcm::latest::prelude::*;

	#[test]
	fn test_automatic_versioning_on_runtime_upgrade_with_relay() {
		MockNet::reset();

		ParaA::execute_with(|| {
			// Set version
			parachain::XcmVersioner::set_version(1);
		});

		Relay::execute_with(|| {
			// This sets the default version, for not known destinations
			assert_ok!(RelayChainPalletXcm::force_default_xcm_version(
				relay_chain::RuntimeOrigin::root(),
				Some(2)
			));

			// Wrap version, which sets VersionedStorage
			// This is necessary because the mock router does not use wrap_version, but
			// this is not necessary in prod
			// This triggers note_unknown_version in pallet-xcm
			// And also version negotiation
			assert_ok!(<RelayChainPalletXcm as xcm::WrapVersion>::wrap_version(
				&Parachain(1).into(),
				Xcm::<()>(vec![])
			));

			// Let's advance the relay. This should trigger the subscription message
			relay_chain::relay_roll_to(2);

			// queries should have been updated
			// we received a query back from the para indicating of the version
			assert!(RelayChainPalletXcm::query(0).is_some());
		});

		// We received 1 as the supported version from paraA
		let expected_supported_version: relay_chain::RuntimeEvent =
			pallet_xcm::Event::SupportedVersionChanged(
				MultiLocation { parents: 0, interior: X1(Parachain(1)) },
				1,
			)
			.into();

		Relay::execute_with(|| {
			// Assert that the events vector contains the version change
			assert!(relay_chain::relay_events().contains(&expected_supported_version));
		});

		let expected_version_notified: parachain::RuntimeEvent =
			pallet_xcm::Event::VersionChangeNotified(
				MultiLocation { parents: 1, interior: Here },
				2,
			)
			.into();

		// ParaA changes version to 2, and calls on_runtime_upgrade. This should notify the targets
		// of the new version change
		ParaA::execute_with(|| {
			// Set version
			parachain::XcmVersioner::set_version(2);
			// Do runtime upgrade
			parachain::on_runtime_upgrade();
			// Initialize block, to call on_initialize and notify targets
			parachain::para_roll_to(2);
			// Expect the event in the parachain
			assert!(parachain::para_events().contains(&expected_version_notified));
		});

		// This event should have been seen in the relay
		let expected_supported_version_2: relay_chain::RuntimeEvent =
			pallet_xcm::Event::SupportedVersionChanged(
				MultiLocation { parents: 0, interior: X1(Parachain(1)) },
				2,
			)
			.into();

		Relay::execute_with(|| {
			// Assert that the events vector contains the new version change
			assert!(relay_chain::relay_events().contains(&expected_supported_version_2));
		});
	}
	#[test]
	fn test_automatic_versioning_on_runtime_upgrade_with_para_b() {
		MockNet::reset();

		ParaA::execute_with(|| {
			// advertised version
			parachain::XcmVersioner::set_version(2);
		});

		ParaB::execute_with(|| {
			// advertised version
			parachain::XcmVersioner::set_version(1);
		});

		ParaA::execute_with(|| {
			// This sets the default version, for not known destinations
			assert_ok!(ParachainPalletXcm::force_default_xcm_version(
				parachain::RuntimeOrigin::root(),
				Some(2)
			));
			// Wrap version, which sets VersionedStorage
			// This will trigger negotiation
			assert_ok!(<ParachainPalletXcm as xcm::WrapVersion>::wrap_version(
				&MultiLocation::new(1, X1(Parachain(2))).into(),
				Xcm::<()>(vec![])
			));

			parachain::para_roll_to(2);

			// queries should have been updated
			assert!(ParachainPalletXcm::query(0).is_some());
		});

		let expected_supported_version: parachain::RuntimeEvent =
			pallet_xcm::Event::SupportedVersionChanged(
				MultiLocation { parents: 1, interior: X1(Parachain(2)) },
				1,
			)
			.into();

		ParaA::execute_with(|| {
			// Assert that the events vector contains the version change
			assert!(parachain::para_events().contains(&expected_supported_version));
		});

		let expected_version_notified: parachain::RuntimeEvent =
			pallet_xcm::Event::VersionChangeNotified(
				MultiLocation { parents: 1, interior: X1(Parachain(1)) },
				2,
			)
			.into();

		// ParaB changes version to 2, and calls on_runtime_upgrade. This should notify the targets
		// of the new version change
		ParaB::execute_with(|| {
			// Set version
			parachain::XcmVersioner::set_version(2);
			// Do runtime upgrade
			parachain::on_runtime_upgrade();
			// Initialize block, to call on_initialize and notify targets
			parachain::para_roll_to(2);
			// Expect the event in the parachain
			assert!(parachain::para_events().contains(&expected_version_notified));
		});

		// This event should have been seen in para A
		let expected_supported_version_2: parachain::RuntimeEvent =
			pallet_xcm::Event::SupportedVersionChanged(
				MultiLocation { parents: 1, interior: X1(Parachain(2)) },
				2,
			)
			.into();

		// Para A should have received the version change
		ParaA::execute_with(|| {
			// Assert that the events vector contains the new version change
			assert!(parachain::para_events().contains(&expected_supported_version_2));
		});
	}
}