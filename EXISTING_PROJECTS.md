# Working with Existing Projects

## 🎯 Current Status: New Projects Only

**Right now**, Glassflow is designed for **creating new projects from scratch**. You describe what you want, and it generates the code.

## 🔮 How It SHOULD Work for Existing Projects

You're absolutely right - a production system needs to handle existing codebases! Here's how it would work:

### Scenario 1: Add Features to Existing Project

**What you'd do:**
```
Input: "Add user authentication to my existing todo app"
Context: Upload/point to existing codebase
```

**What Architect would do:**
1. **Analyze existing code** (scan files, understand structure)
2. **Identify integration points** (where to add auth)
3. **Detect conflicts** (existing auth? routing?)
4. **Plan modifications** (which files to modify vs create)

**What Engineer would do:**
1. **Generate new files** (auth components, API routes)
2. **Modify existing files** (add imports, update routes)
3. **Preserve existing code** (don't overwrite working features)

### Scenario 2: Refactor Existing Code

**What you'd do:**
```
Input: "Refactor my app to use TypeScript instead of JavaScript"
Context: Existing JavaScript codebase
```

**What would happen:**
1. Architect analyzes current structure
2. Plans migration strategy
3. Engineer converts files one by one
4. Quality ensures type safety
5. Debug validates nothing broke

### Scenario 3: Fix Bugs in Existing Code

**What you'd do:**
```
Input: "Fix the bug where todos don't save to localStorage"
Context: Existing codebase with bug
```

**What would happen:**
1. Debug agent analyzes the code
2. Identifies the bug (missing save call)
3. Auto-fixes the issue
4. Quality reviews the fix
5. Returns modified code

---

## 🛠️ What's Missing for Existing Projects

### 1. **Codebase Upload/Analysis**
**Current**: ❌ Not implemented  
**Needed**: 
- File upload interface
- Directory scanning
- Dependency analysis
- Understanding existing architecture

### 2. **File Modification (vs Creation)**
**Current**: ✅ Engineer can write files  
**Needed**:
- Read existing files
- Modify specific sections
- Preserve unchanged code
- Handle merge conflicts

### 3. **Context Awareness**
**Current**: ❌ Each generation is isolated  
**Needed**:
- Remember existing code structure
- Understand project conventions
- Respect existing patterns
- Maintain consistency

### 4. **Incremental Changes**
**Current**: ❌ Generates complete files  
**Needed**:
- Patch existing files
- Add functions to existing classes
- Insert imports without replacing file
- Smart merging

---

## 💡 How to Add Existing Project Support

### Phase 1: Code Analysis (Architect Enhancement)

```rust
// New capability for Architect
pub async fn analyze_existing_codebase(
    &mut self,
    project_path: PathBuf
) -> Result<CodebaseAnalysis> {
    // Scan all files
    let files = scan_directory(&project_path)?;
    
    // Analyze structure
    let structure = analyze_file_structure(&files)?;
    
    // Detect tech stack
    let tech_stack = detect_tech_stack(&files)?;
    
    // Identify patterns
    let patterns = detect_architecture_patterns(&files)?;
    
    Ok(CodebaseAnalysis {
        files,
        structure,
        tech_stack,
        patterns,
        entry_points: find_entry_points(&files)?,
    })
}
```

### Phase 2: Smart File Modification (Engineer Enhancement)

```rust
// New capability for Engineer
pub async fn modify_existing_file(
    &self,
    file_path: &str,
    modification: FileModification
) -> Result<ModifiedFile> {
    // Read existing file
    let existing_content = fs::read_to_string(file_path)?;
    
    // Parse into AST
    let ast = parse_to_ast(&existing_content)?;
    
    // Apply modification
    let modified_ast = match modification {
        FileModification::AddFunction(func) => {
            ast.add_function(func)
        }
        FileModification::ModifyFunction(name, new_impl) => {
            ast.modify_function(name, new_impl)
        }
        FileModification::AddImport(import) => {
            ast.add_import(import)
        }
    };
    
    // Generate modified code
    let new_content = ast.to_code()?;
    
    Ok(ModifiedFile {
        path: file_path.to_string(),
        original: existing_content,
        modified: new_content,
        diff: generate_diff(&existing_content, &new_content),
    })
}
```

### Phase 3: Context-Aware Generation

```rust
// Enhanced Engineer with context
pub struct Engineer {
    llm_client: Box<dyn LLMClient>,
    codebase_context: Option<CodebaseAnalysis>, // NEW!
}

impl Engineer {
    pub async fn generate_with_context(
        &self,
        request: GenerationRequest,
    ) -> Result<CodeOutput> {
        // Build prompt with existing code context
        let prompt = format!(
            "Existing codebase uses: {:?}\n\
             Existing patterns: {:?}\n\
             Generate code that fits this style:\n\
             {}",
            self.codebase_context.tech_stack,
            self.codebase_context.patterns,
            request.description
        );
        
        // Generate code that matches existing style
        self.llm_client.send_message(&prompt).await
    }
}
```

---

## 🎯 Recommended Workflow for Existing Projects

### Option 1: Manual Context (Current Workaround)

```
1. Describe your existing project in the prompt:
   "I have a React todo app using Redux. Add user authentication."

2. Architect will understand the context from your description

3. Review generated code and manually integrate it
```

### Option 2: Upload Context Files (Future)

```
1. Upload key files (package.json, main components)
2. Architect analyzes structure
3. Engineer generates code that fits
4. You get a diff/patch to apply
```

### Option 3: Full Integration (Future)

```
1. Point to your project directory
2. System scans and understands everything
3. Make requests like "add feature X"
4. System modifies files directly
5. Review changes via git diff
```

---

## 🚀 Quick Wins for Existing Projects (Now)

Even without full support, you can use Glassflow for existing projects:

### 1. **Generate New Components**
```
Input: "Create a UserProfile component for my React app"
Output: New component file you can add to your project
```

### 2. **Generate Utility Functions**
```
Input: "Create a function to validate email addresses"
Output: Standalone function you can copy into your utils
```

### 3. **Generate Types/Interfaces**
```
Input: "Create TypeScript types for a User with name, email, role"
Output: Type definitions you can add to your types file
```

### 4. **Get Architecture Advice**
```
Input: "How should I structure authentication in my React app?"
Output: Architecture plan you can follow manually
```

---

## 📝 What You Can Do Today

**For Existing Projects:**

1. **Describe your existing setup** in the prompt:
   ```
   "I have a React app with Redux and TypeScript. 
    Add a dark mode toggle feature."
   ```

2. **Generate standalone pieces**:
   - New components
   - New utilities
   - New types
   - New API routes

3. **Manually integrate** the generated code into your project

4. **Use Quality/Debug** on your existing code:
   - Copy your code into Quality agent
   - Get security and quality feedback
   - Get auto-fixes for issues

---

## 🎉 Summary

**Current Capabilities:**
- ✅ Generate new projects from scratch
- ✅ Generate standalone components/utilities
- ✅ Review existing code (Quality/Debug)
- ✅ Architecture advice

**Missing for Full Existing Project Support:**
- ❌ Codebase upload/scanning
- ❌ File modification (vs creation)
- ❌ Context-aware generation
- ❌ Smart merging/patching

**Workaround:**
- Describe your existing setup in prompts
- Generate new pieces
- Manually integrate

**Future:**
- Upload project directory
- System understands context
- Modifies files intelligently
- Shows diffs for review

---

**You're right to ask about this - it's a critical feature for real-world use!** The foundation is there (code generation, quality review, debugging), but the "existing project integration" layer needs to be built on top. 🚀
