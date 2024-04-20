#[cfg(not(feature = "alloc"))]
use std::collections::HashMap;
#[cfg(feature = "alloc")]
use cairo_vm::without_std::collections::HashMap;

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::vec::IntoIter;
use anyhow::{bail, Context, Result};
/*
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
    pub fn new(
        path: &Path,
        starknet: bool,
        allow_warnings: bool,
        config: TestRunConfig,
    ) -> Result<Self> {
        let compiler = TestCompiler::try_new(path, starknet, allow_warnings, config.gas_enabled)?;
        Ok(Self { compiler, config })
    }

    /// Runs the tests and process the results for a summary.
    pub fn run(&self) -> Result<Option<TestsSummary>> {
        let runner = CompiledTestRunner::new(self.compiler.build()?, self.config.clone());
        runner.run(Some(&self.compiler.db))
    }
}

impl TestCompiler {
    /// Configure a new test compiler
    ///
    /// # Arguments
    ///
    /// * `path` - The path to compile and run its tests
    /// * `starknet` - Add the starknet plugin to run the tests
    pub fn try_new(
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

        let main_crate_ids = setup_project(db, Path::new(&path))?;
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

    /// Build the tests and collect metadata.
    pub fn build(&self) -> Result<TestCompilation> {
        compile_test_prepared_db(
            &self.db,
            self.starknet,
            self.main_crate_ids.clone(),
            self.test_crate_ids.clone(),
        )
    }
}
 */