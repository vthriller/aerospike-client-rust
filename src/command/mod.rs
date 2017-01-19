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

pub mod info_command;
pub mod buffer;
pub mod single_command;
pub mod read_command;
pub mod write_command;
pub mod delete_command;
pub mod touch_command;
pub mod exists_command;
pub mod read_header_command;
pub mod operate_command;
pub mod execute_udf_command;
pub mod stream_command;
pub mod scan_command;
pub mod query_command;
pub mod admin_command;

use std::sync::Arc;
use std::time::Duration;

use net::Connection;
use errors::*;

use cluster::Node;

// Command interface describes all commands available
pub trait Command {
    fn write_timeout(&mut self,
                     conn: &mut Connection,
                     timeout: Option<Duration>)
                     -> Result<()>;
    fn prepare_buffer(&mut self, conn: &mut Connection) -> Result<()>;
    fn get_node(&self) -> Result<Arc<Node>>;
    fn parse_result(&mut self, conn: &mut Connection) -> Result<()>;
    fn write_buffer(&mut self, conn: &mut Connection) -> Result<()>;
}
