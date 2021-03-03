use arrow::util::pretty;
use std::time::{Duration, Instant};
use datafusion::datasource::csv::CsvReadOptions;
use datafusion::error::Result;
use datafusion::execution::context::ExecutionContext;


fn main() -> Result<()>{
    let start = Instant::now();

    let mut ctx = ExecutionContext::new();
    
    ctx.register_csv(
        // ファイルの名前
    "",
    // ファイルのパス
    "",
    CsvReadOptions::new(),
    );

    // let sql = "SELECT FROM";

    // create the query plan
    let plan = ctx.create_logical_plan(sql)?;
    let plan = ctx.optimize(&plan)?;
    let plan = ctx.create_physical_plan(&plan, 1024 * 1024)?;

    let results = ctx.collect(plan.as_ref())?;

    let duration = start.elapsed();
    println!("Time elapsed in SQL() is: {:?}", diration);

    pretty::print_batches(&results)?;

    Ok(())


}

