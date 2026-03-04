use std::path::PathBuf;

use csv::StringRecord;
use rhai::{AST, Engine, EvalAltResult};

pub struct ScriptEngine {
    engine: Engine,
    ast: AST,
}

impl ScriptEngine {
    pub fn new(path: PathBuf) -> Result<Self, Box<EvalAltResult>> {
        let engine = Engine::new();
        let ast = engine.compile_file(path)?;

        Ok(Self { engine, ast })
    }

    pub fn run_filter(&mut self, data: StringRecord) -> Option<String> {
        let tmp_engine = self.engine.register_fn("data", move |idx: i64| -> String {
            let inp :Option<usize> = idx.try_into().ok();
            let d = inp.and_then(|i| data.get(i));
            d.map(|s| s.to_string()).unwrap_or_default()
        });

        if let Ok(res) = tmp_engine.eval_ast(&self.ast) {
            Some(res)
        } else {
            None
        }
    }
}
