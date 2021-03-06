// Copyright 2015-2017 Aerospike, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;
use std::time::Duration;

use errors::*;
use Bins;
use Key;
use cluster::{Node, Cluster};
use commands::{Command, SingleCommand, ReadCommand};
use net::Connection;
use operations::Operation;
use policy::WritePolicy;

pub struct OperateCommand<'a> {
    pub read_command: ReadCommand<'a>,
    policy: &'a WritePolicy,
    operations: &'a [Operation<'a>],
}

impl<'a> OperateCommand<'a> {
    pub fn new(policy: &'a WritePolicy,
               cluster: Arc<Cluster>,
               key: &'a Key,
               operations: &'a [Operation<'a>])
               -> Self {
        OperateCommand {
            read_command: ReadCommand::new(&policy.base_policy, cluster, key, Bins::All),
            policy: policy,
            operations: operations,
        }
    }

    pub fn execute(&mut self) -> Result<()> {
        SingleCommand::execute(self.policy, self)
    }
}

impl<'a> Command for OperateCommand<'a> {
    fn write_timeout(&mut self, conn: &mut Connection, timeout: Option<Duration>) -> Result<()> {
        conn.buffer.write_timeout(timeout);
        Ok(())
    }

    fn write_buffer(&mut self, conn: &mut Connection) -> Result<()> {
        conn.flush()
    }

    fn prepare_buffer(&mut self, conn: &mut Connection) -> Result<()> {
        conn.buffer.set_operate(self.policy,
                                self.read_command.single_command.key,
                                self.operations)
    }

    fn get_node(&self) -> Result<Arc<Node>> {
        self.read_command.get_node()
    }

    fn parse_result(&mut self, conn: &mut Connection) -> Result<()> {
        self.read_command.parse_result(conn)
    }
}
