#!/bin/bash

# Fix all subagent metrics issues
SUBAGENTS=("draftsman" "engineer" "yardmaster" "brakeman" "diffusion" "nitrogen" "omni" "conductor")

for subagent in "${SUBAGENTS[@]}"; do
    echo "Fixing $subagent..."
    
    # Update imports
    sed -i 's/BaseSubagentMetrics/MetricsStorage/g' "crates/trinity-subagents/trinity-$subagent/src/lib.rs"
    
    # Update metrics field type
    sed -i 's/metrics: Arc<RwLock<BaseSubagentMetrics>>/metrics: MetricsStorage/g' "crates/trinity-subagents/trinity-$subagent/src/lib.rs"
    
    # Update initialization
    sed -i 's/metrics: Arc::new(RwLock::new(BaseSubagentMetrics::new()))/metrics: MetricsStorage::new()/g' "crates/trinity-subagents/trinity-$subagent/src/lib.rs"
    
    # Fix metrics calls
    sed -i 's/self\.metrics\.write()\.unwrap()\.update/self.metrics.update/g' "crates/trinity-subagents/trinity-$subagent/src/lib.rs"
    
    # Fix get_metrics
    sed -i 's/fn get_metrics(&self) -> &SubagentMetrics {/fn get_metrics(&self) -> SubagentMetrics {/g' "crates/trinity-subagents/trinity-$subagent/src/lib.rs"
    sed -i 's/unimplemented!("Need to restructure to avoid lifetime issue")/self.metrics.get()/g' "crates/trinity-subagents/trinity-$subagent/src/lib.rs"
    
    # Fix update_metrics
    sed -i 's/fn update_metrics(&mut self, task_duration_ms: u64, success: bool) {/fn update_metrics(&self, task_duration_ms: u64, success: bool) {/g' "crates/trinity-subagents/trinity-$subagent/src/lib.rs"
    sed -i 's/self\.metrics\.write()\.unwrap()\.update(task_duration_ms, success)/self.metrics.update(task_duration_ms, success)/g' "crates/trinity-subagents/trinity-$subagent/src/lib.rs"
    
    echo "Fixed $subagent"
done

echo "All subagents fixed!"
