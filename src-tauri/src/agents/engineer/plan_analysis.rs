// T031: PlanAnalyzer - parse ArchitecturePlan, validate completeness, build dependency graph
use crate::models::{EngineerArchitecturePlan, FileTemplate};
use std::collections::{HashMap, HashSet};

pub struct PlanAnalyzer {
    plan: EngineerArchitecturePlan,
}

#[derive(Debug, Clone)]
pub struct DependencyGraph {
    pub nodes: Vec<String>,
    pub edges: HashMap<String, Vec<String>>,
    pub levels: Vec<Vec<String>>, // Topologically sorted levels
}

impl PlanAnalyzer {
    pub fn new(plan: EngineerArchitecturePlan) -> Self {
        Self { plan }
    }

    pub fn validate(&self) -> Result<(), String> {
        self.plan.validate()?;
        
        // Check for circular dependencies
        let graph = self.build_dependency_graph()?;
        self.detect_circular_dependencies(&graph)?;
        
        Ok(())
    }

    pub fn build_dependency_graph(&self) -> Result<DependencyGraph, String> {
        let mut nodes = Vec::new();
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        
        // Build graph from file templates
        for file in &self.plan.file_structure.files {
            nodes.push(file.path.clone());
            
            // Add dependencies (filter to only include generated files)
            let file_deps: Vec<String> = file
                .dependencies
                .iter()
                .filter(|dep| {
                    // Check if dependency is a relative path (generated file)
                    dep.starts_with("./") || dep.starts_with("../")
                })
                .map(|dep| self.resolve_relative_path(&file.path, dep))
                .collect();
            
            edges.insert(file.path.clone(), file_deps);
        }
        
        // Topological sort to determine generation order
        let levels = self.topological_sort(&nodes, &edges)?;
        
        Ok(DependencyGraph {
            nodes,
            edges,
            levels,
        })
    }

    fn topological_sort(
        &self,
        nodes: &[String],
        edges: &HashMap<String, Vec<String>>,
    ) -> Result<Vec<Vec<String>>, String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut levels = Vec::new();
        
        // Calculate in-degrees
        for node in nodes {
            in_degree.insert(node.clone(), 0);
        }
        
        for deps in edges.values() {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }
        
        // Process nodes level by level
        let mut remaining: HashSet<String> = nodes.iter().cloned().collect();
        
        while !remaining.is_empty() {
            // Find all nodes with in-degree 0
            let current_level: Vec<String> = remaining
                .iter()
                .filter(|node| *in_degree.get(*node).unwrap_or(&0) == 0)
                .cloned()
                .collect();
            
            if current_level.is_empty() {
                return Err("Circular dependency detected".to_string());
            }
            
            // Remove processed nodes and update in-degrees
            for node in &current_level {
                remaining.remove(node);
                
                if let Some(deps) = edges.get(node) {
                    for dep in deps {
                        if let Some(degree) = in_degree.get_mut(dep) {
                            *degree = degree.saturating_sub(1);
                        }
                    }
                }
            }
            
            levels.push(current_level);
        }
        
        Ok(levels)
    }

    fn detect_circular_dependencies(&self, graph: &DependencyGraph) -> Result<(), String> {
        // If topological sort succeeded, no circular dependencies exist
        // This is a double-check
        for (node, deps) in &graph.edges {
            for dep in deps {
                if self.has_path_to(graph, dep, node) {
                    return Err(format!("Circular dependency: {} ↔ {}", node, dep));
                }
            }
        }
        Ok(())
    }

    fn has_path_to(&self, graph: &DependencyGraph, from: &str, to: &str) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![from];
        
        while let Some(current) = stack.pop() {
            if current == to {
                return true;
            }
            
            if visited.contains(current) {
                continue;
            }
            
            visited.insert(current);
            
            if let Some(deps) = graph.edges.get(current) {
                for dep in deps {
                    stack.push(dep);
                }
            }
        }
        
        false
    }

    fn resolve_relative_path(&self, from: &str, to: &str) -> String {
        // Simple relative path resolution
        // TODO: Implement proper path resolution
        if to.starts_with("./") {
            let base = std::path::Path::new(from)
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or("");
            format!("{}/{}", base, &to[2..])
        } else {
            to.to_string()
        }
    }

    pub fn get_independent_files(&self) -> Vec<&FileTemplate> {
        self.plan
            .file_structure
            .files
            .iter()
            .filter(|f| f.is_independent())
            .collect()
    }

    pub fn get_generation_order(&self) -> Result<Vec<Vec<&FileTemplate>>, String> {
        let graph = self.build_dependency_graph()?;
        
        let mut ordered = Vec::new();
        for level in &graph.levels {
            let level_files: Vec<&FileTemplate> = level
                .iter()
                .filter_map(|path| {
                    self.plan
                        .file_structure
                        .files
                        .iter()
                        .find(|f| &f.path == path)
                })
                .collect();
            ordered.push(level_files);
        }
        
        Ok(ordered)
    }

    pub fn estimate_total_lines(&self) -> usize {
        self.plan
            .file_structure
            .files
            .iter()
            .map(|f| f.estimated_lines)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{FileStructure, Language, ProjectIntent, TechStack, ArchitecturePattern};

    fn create_test_plan() -> EngineerArchitecturePlan {
        EngineerArchitecturePlan {
            project_name: "test".to_string(),
            project_intent: ProjectIntent::WebApp,
            tech_stack: TechStack {
                language: "TypeScript".to_string(),
                runtime: "Node.js".to_string(),
                framework: Some("React".to_string()),
                styling: None,
                state_management: None,
                testing: None,
            },
            architecture_pattern: ArchitecturePattern::Atomic,
            file_structure: FileStructure {
                directories: Vec::new(),
                files: vec![
                    FileTemplate {
                        path: "src/types.ts".to_string(),
                        purpose: "Types".to_string(),
                        estimated_lines: 10,
                        language: Language::TypeScript,
                        dependencies: vec![],
                        generation_hints: None,
                    },
                    FileTemplate {
                        path: "src/utils.ts".to_string(),
                        purpose: "Utils".to_string(),
                        estimated_lines: 20,
                        language: Language::TypeScript,
                        dependencies: vec!["./types".to_string()],
                        generation_hints: None,
                    },
                ],
            },
            component_hierarchy: Vec::new(),
            dependencies: Vec::new(),
            architecture_decisions: Vec::new(),
            confidence_breakdown: None,
        }
    }

    #[test]
    fn test_validate_plan() {
        let analyzer = PlanAnalyzer::new(create_test_plan());
        assert!(analyzer.validate().is_ok());
    }

    #[test]
    fn test_dependency_graph() {
        let analyzer = PlanAnalyzer::new(create_test_plan());
        let graph = analyzer.build_dependency_graph().unwrap();
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.levels.len(), 2);
    }

    #[test]
    fn test_independent_files() {
        let analyzer = PlanAnalyzer::new(create_test_plan());
        let independent = analyzer.get_independent_files();
        assert_eq!(independent.len(), 1);
        assert_eq!(independent[0].path, "src/types.ts");
    }
}
