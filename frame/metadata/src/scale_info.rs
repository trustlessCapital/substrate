// This file is part of Substrate.

// Copyright (C) 2018-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Decodable variant of the RuntimeMetadata.
//!
//! This really doesn't belong here, but is necessary for the moment. In the future
//! it should be removed entirely to an external module for shimming on to the
//! codec-encoded metadata.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use serde::Serialize;
#[cfg(feature = "std")]
use codec::{Encode, Output};
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;

use scale_info::{
	form::{
		CompactForm,
		Form,
		MetaForm,
	},
	meta_type,
	IntoCompact,
	Registry,
	TypeInfo,
};

/// The metadata of a runtime.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize), serde(bound = "F::TypeId: Serialize"))]
pub struct RuntimeMetadataV11<F: Form = MetaForm> {
	/// Metadata of all the modules.
	pub modules: Vec<ModuleMetadata<F>>,
	/// Metadata of the extrinsic.
	pub extrinsic: ExtrinsicMetadata<F>,
}

impl IntoCompact for RuntimeMetadataV11 {
	type Output = RuntimeMetadataV11<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		RuntimeMetadataV11 {
			modules: registry.map_into_compact(self.modules),
			extrinsic: self.extrinsic.into_compact(registry),
		}
	}
}

/// Metadata of the extrinsic used by the runtime.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize), serde(bound = "F::TypeId: Serialize"))]
pub struct ExtrinsicMetadata<F: Form = MetaForm> {
	/// Extrinsic version.
	pub version: u8,
	/// The signed extensions in the order they appear in the extrinsic.
	pub signed_extensions: Vec<F::TypeId>,
}

impl IntoCompact for ExtrinsicMetadata {
	type Output = ExtrinsicMetadata<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		ExtrinsicMetadata {
			version: self.version,
			signed_extensions: registry.register_types(self.signed_extensions),
		}
	}
}

/// All metadata about an runtime module.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize), serde(bound = "F::TypeId: Serialize"))]
pub struct ModuleMetadata<F: Form = MetaForm> {
	pub name: &'static str,
	// pub storage: Option<DecodeDifferent<FnEncode<StorageMetadata>, StorageMetadata>>,
	pub calls: Option<FunctionMetadata<F>>,
	// pub event: ODFnA<EventMetadata>,
	// pub constants: DFnA<ModuleConstantMetadata>,
	// pub errors: DFnA<ErrorMetadata>,
}

impl IntoCompact for ModuleMetadata {
	type Output = ModuleMetadata<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		ModuleMetadata {
			name: self.name,
			calls: self.calls.map(|calls| calls.into_compact(registry)),
		}
	}
}

/// All the metadata about a function.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize), serde(bound = "F::TypeId: Serialize"))]
pub struct FunctionMetadata<F: Form = MetaForm> {
	pub name: &'static str,
	pub arguments: Vec<FunctionArgumentMetadata<F>>,
	pub documentation: &'static [&'static str],
}

impl IntoCompact for FunctionMetadata {
	type Output = FunctionMetadata<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		FunctionMetadata {
			name: self.name,
			arguments: registry.map_into_compact(self.arguments),
			documentation: self.documentation,
		}
	}
}

/// All the metadata about a function argument.
#[derive(Clone, PartialEq, Eq, Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize), serde(bound = "F::TypeId: Serialize"))]
pub struct FunctionArgumentMetadata<F: Form = MetaForm> {
	pub name: &'static str,
	pub ty: F::TypeId,
}

impl IntoCompact for FunctionArgumentMetadata {
	type Output = FunctionArgumentMetadata<CompactForm>;

	fn into_compact(self, registry: &mut Registry) -> Self::Output {
		FunctionArgumentMetadata {
			name: self.name,
			ty: registry.register_type(&self.ty),
		}
	}
}
