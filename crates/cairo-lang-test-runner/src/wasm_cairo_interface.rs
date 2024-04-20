use std::path::Path;
use std::sync::Arc;

use anyhow::{bail, Result};

use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_compiler::diagnostics::DiagnosticsReporter;
use cairo_lang_compiler::wasm_cairo_interface::setup_project_with_input_string;
use cairo_lang_filesystem::cfg::{Cfg, CfgSet};
use cairo_lang_filesystem::db::FilesGroupEx;
use cairo_lang_filesystem::flag::Flag;
use cairo_lang_filesystem::ids::FlagId;

use cairo_lang_starknet::starknet_plugin_suite;
use cairo_lang_test_plugin::test_plugin_suite;

use crate::{TestCompiler, TestRunConfig, TestRunner};


impl TestRunner {
    /// Configure a new test runner
    ///
    /// # Arguments
    ///
    /// * `path` - The path to compile and run its tests
    /// * `filter` - Run only tests containing the filter string
    /// * `include_ignored` - Include ignored tests as well
    /// * `ignored` - Run ignored tests only
    /// * `starknet` - Add the starknet plugin to run the tests
    pub fn new_with_string(
        input_program_string: &String,
        path: &Path,
        starknet: bool,
        allow_warnings: bool,
        config: TestRunConfig,
    ) -> Result<Self> {
        let compiler = TestCompiler::try_new_with_string(input_program_string, path, starknet, allow_warnings, config.gas_enabled)?;
        Ok(Self { compiler, config })
    }
}

impl TestCompiler {
    /// Configure a new test compiler
    ///
    /// # Arguments
    ///
    /// * `path` - The path to compile and run its tests
    /// * `starknet` - Add the starknet plugin to run the tests
    pub fn try_new_with_string(
        input_program_string: &String,
        path: &Path,
        starknet: bool,
        allow_warnings: bool,
        gas_enabled: bool,
    ) -> Result<Self> {
        let db = &mut {
            let mut b = RootDatabase::builder();
            if !gas_enabled {
                b.skip_auto_withdraw_gas();
            }
            b.detect_corelib();
            b.with_cfg(CfgSet::from_iter([Cfg::name("test"), Cfg::kv("target", "test")]));
            b.with_plugin_suite(test_plugin_suite());
            if starknet {
                b.with_plugin_suite(starknet_plugin_suite());
            }
            b.build()?
        };
        let add_redeposit_gas_flag_id = FlagId::new(db, "add_redeposit_gas");
        db.set_flag(add_redeposit_gas_flag_id, Some(Arc::new(Flag::AddRedepositGas(true))));

        let main_crate_ids = setup_project_with_input_string(db, Path::new(&path), input_program_string)?;
        let mut reporter = DiagnosticsReporter::stderr().with_crates(&main_crate_ids);
        if allow_warnings {
            reporter = reporter.allow_warnings();
        }
        if reporter.check(db) {
            bail!("failed to compile: {}", path.display());
        }

        Ok(Self {
            db: db.snapshot(),
            test_crate_ids: main_crate_ids.clone(),
            main_crate_ids,
            starknet,
        })
    }
}
