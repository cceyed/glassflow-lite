// T044: FileScheduler - determine generation order based on dependencies
use crate::models::FileTemplate;
use std::collections::{HashMap, HashSet};

pub struct FileScheduler;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenerationStrategy {
    Sequential,      // Generate one at a time
    Parallel,        // Generate all independent files at once
    DependencyFirst, // Generate in dependency order with parallelism
}

#[derive(Debug, Clone)]
pub struct GenerationPlan {
    pub strategy: GenerationStrategy,
    pub batches: Vec<Vec<usize>>, // Indices of files to generate in each batch
}

impl FileScheduler {
    pub fn create_plan(files: &[FileTemplate], max_concurrent: usize) -> GenerationPlan {
        // Build dependency graph
        let graph = Self::build_dependency_graph(files);
        
        // Determine strategy
        let strategy = Self::determine_strategy(&graph, files);
        
        // Create batches based on strategy
        let batches = match strategy {
            GenerationStrategy::Sequential => Self::create_sequential_batches(files.len()),
            GenerationStrategy::Parallel => Self::create_parallel_batches(files, max_concurrent),
            GenerationStrategy::DependencyFirst => {
                Self::create_dependency_batches(&graph, files, max_concurrent)
            }
        };
        
        GenerationPlan { strategy, batches }
    }

    fn build_dependency_graph(files: &[FileTemplate]) -> HashMap<usize, Vec<usize>> {
        let mut graph = HashMap::new();
        
        for (i, file) in files.iter().enumerate() {
            let mut deps = Vec::new();
            
            for dep in &file.dependencies {
                // Find the file index that matches this dependency
                if let Some(dep_idx) = files.iter().position(|f| {
                    dep.contains(&f.path) || f.path.contains(dep)
                }) {
                    deps.push(dep_idx);
                }
            }
            
            graph.insert(i, deps);
        }
        
        graph
    }

    fn determine_strategy(graph: &HashMap<usize, Vec<usize>>, files: &[FileTemplate]) -> GenerationStrategy {
        // Count independent files
        let independent_count = graph.values().filter(|deps| deps.is_empty()).count();
        
        // If all files are independent, use parallel
        if independent_count == files.len() {
            return GenerationStrategy::Parallel;
        }
        
        // If no files are independent, use sequential
        if independent_count == 0 {
            return GenerationStrategy::Sequential;
        }
        
        // Otherwise, use dependency-first strategy
        GenerationStrategy::DependencyFirst
    }

    fn create_sequential_batches(file_count: usize) -> Vec<Vec<usize>> {
        (0..file_count).map(|i| vec![i]).collect()
    }

    fn create_parallel_batches(files: &[FileTemplate], max_concurrent: usize) -> Vec<Vec<usize>> {
        let indices: Vec<usize> = (0..files.len()).collect();
        
        // Split into chunks of max_concurrent
        indices
            .chunks(max_concurrent)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    fn create_dependency_batches(
        graph: &HashMap<usize, Vec<usize>>,
        files: &[FileTemplate],
        max_concurrent: usize,
    ) -> Vec<Vec<usize>> {
        let mut batches = Vec::new();
        let mut completed = HashSet::new();
        let mut remaining: HashSet<usize> = (0..files.len()).collect();
        
        while !remaining.is_empty() {
            // Find all files whose dependencies are completed
            let ready: Vec<usize> = remaining
                .iter()
                .filter(|&&idx| {
                    graph
                        .get(&idx)
                        .map(|deps| deps.iter().all(|d| completed.contains(d)))
                        .unwrap_or(true)
                })
                .copied()
                .collect();
            
            if ready.is_empty() {
                // Circular dependency or error - add remaining sequentially
                batches.extend(remaining.iter().map(|&i| vec![i]));
                break;
            }
            
            // Split ready files into batches of max_concurrent
            for chunk in ready.chunks(max_concurrent) {
                batches.push(chunk.to_vec());
                for &idx in chunk {
                    completed.insert(idx);
                    remaining.remove(&idx);
                }
            }
        }
        
        batches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Language;

    fn create_test_file(path: &str, deps: Vec<String>) -> FileTemplate {
        FileTemplate {
            path: path.to_string(),
            purpose: "Test".to_string(),
            estimated_lines: 10,
            language: Language::TypeScript,
            dependencies: deps,
            generation_hints: None,
        }
    }

    #[test]
    fn test_independent_files() {
        let files = vec![
            create_test_file("a.ts", vec![]),
            create_test_file("b.ts", vec![]),
            create_test_file("c.ts", vec![]),
        ];
        
        let plan = FileScheduler::create_plan(&files, 3);
        assert_eq!(plan.strategy, GenerationStrategy::Parallel);
        assert_eq!(plan.batches.len(), 1);
        assert_eq!(plan.batches[0].len(), 3);
    }

    #[test]
    fn test_dependent_files() {
        let files = vec![
            create_test_file("a.ts", vec![]),
            create_test_file("b.ts", vec!["./a".to_string()]),
            create_test_file("c.ts", vec!["./b".to_string()]),
        ];
        
        let plan = FileScheduler::create_plan(&files, 3);
        assert_eq!(plan.strategy, GenerationStrategy::DependencyFirst);
        assert!(plan.batches.len() >= 3); // At least 3 batches for sequential deps
    }
}
