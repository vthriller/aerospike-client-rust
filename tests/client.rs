// Copyright 2015-2017 Aerospike, Inc.
//
// Portions may be licensed to Aerospike, Inc. under one or more contributor
// license agreements.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

extern crate aerospike;
extern crate env_logger;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use aerospike::Client;

mod common;

#[test]
#[should_panic(expected = "Failed to connect to host(s).")]
fn cluster_name() {
    let mut policy = &mut common::client_policy().clone();
    policy.cluster_name = Some(String::from("notTheRealClusterName"));
    Client::new(policy, &common::hosts()).unwrap();
}
