import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { motion, AnimatePresence } from 'framer-motion';
import { Button } from '../design-system/components/Button';
import { Card } from '../design-system/components/Card';

interface GeneratedFile {
  path: string;
  content: string;
  language: string;
}

export function PipelineUI() {
  const [prompt, setPrompt] = useState('');
  const [isRunning, setIsRunning] = useState(false);
  const [output, setOutput] = useState('');
  const [generatedFiles, setGeneratedFiles] = useState<GeneratedFile[]>([]);
  const [selectedFile, setSelectedFile] = useState<GeneratedFile | null>(null);
  const [currentPhase, setCurrentPhase] = useState('');
  const [lastEventTime, setLastEventTime] = useState<number>(Date.now());
  const outputRef = useRef<HTMLDivElement>(null);

  // Listen for orchestrator reasoning events
  useEffect(() => {
    const unlisten = listen('orchestrator:reasoning', (event: any) => {
      const { content } = event.payload;
      setOutput(prev => prev + content + '\n');
      setLastEventTime(Date.now());
      
      // Extract phase from content
      if (content.includes('Phase 1')) setCurrentPhase('Architecture Planning');
      else if (content.includes('Phase 2')) setCurrentPhase('Code Generation');
      else if (content.includes('Phase 3')) setCurrentPhase('Quality Review');
      else if (content.includes('Phase 4')) setCurrentPhase('Runtime Testing');
      else if (content.includes('complete')) setCurrentPhase('Complete');
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  // Heartbeat to show pipeline is still running
  useEffect(() => {
    if (!isRunning) return;
    
    const interval = setInterval(() => {
      const secondsSinceLastEvent = Math.floor((Date.now() - lastEventTime) / 1000);
      if (secondsSinceLastEvent > 5 && secondsSinceLastEvent % 10 === 0) {
        setOutput(prev => prev + `⏳ Still working... (${secondsSinceLastEvent}s since last update)\n`);
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [isRunning, lastEventTime]);

  // Auto-scroll output to bottom when new content arrives
  useEffect(() => {
    if (outputRef.current) {
      outputRef.current.scrollTop = outputRef.current.scrollHeight;
    }
  }, [output]);

  const runPipeline = async () => {
    if (!prompt.trim()) return;
    
    setIsRunning(true);
    setOutput('🎬 Starting Pipeline...\n\n');
    setGeneratedFiles([]);
    setSelectedFile(null);
    setCurrentPhase('Initializing...');
    
    // Add initial feedback
    setOutput(prev => prev + '⏳ This may take 30-60 seconds...\n');
    setOutput(prev => prev + '📡 Waiting for orchestrator events...\n\n');
    
    try {
      const response = await invoke('orchestrator_run_pipeline', {
        prompt: prompt.trim()
      });
      
      setOutput(prev => prev + '\n✅ ' + String(response) + '\n');
      setCurrentPhase('Complete');
      
      // TODO: Get generated files from the response
      // For now, mock some files
      const mockFiles: GeneratedFile[] = [
        {
          path: 'src/App.tsx',
          content: '// Generated App component\nimport React from "react";\n\nfunction App() {\n  return <div>Hello World</div>;\n}\n\nexport default App;',
          language: 'TypeScript'
        },
        {
          path: 'src/components/Header.tsx',
          content: '// Generated Header component\nimport React from "react";\n\nexport function Header() {\n  return <header>My App</header>;\n}',
          language: 'TypeScript'
        }
      ];
      setGeneratedFiles(mockFiles);
      if (mockFiles.length > 0) {
        setSelectedFile(mockFiles[0]);
      }
    } catch (error: any) {
      // Enhanced error handling
      const errorMessage = error?.message || String(error);
      setOutput(prev => prev + '━'.repeat(50) + '\n');
      setOutput(prev => prev + `Error: ${errorMessage}\n`);
      setOutput(prev => prev + '━'.repeat(50) + '\n\n');
      
      // Provide helpful suggestions
      if (errorMessage.includes('Clarification needed')) {
        setOutput(prev => prev + '💡 This error should not occur in pipeline mode:\n');
        setOutput(prev => prev + '  • The Architect detected ambiguities in your prompt\n');
        setOutput(prev => prev + '  • Pipeline mode should proceed with best-effort assumptions\n');
        setOutput(prev => prev + '  • Try rebuilding the app (cargo build in src-tauri/)\n');
        setOutput(prev => prev + '  • Or try a more specific, detailed prompt\n');
      } else if (errorMessage.includes('API') || errorMessage.includes('LLM')) {
        setOutput(prev => prev + '💡 Suggestions:\n');
        setOutput(prev => prev + '  • Check that your OPENROUTER_API_KEY is set in .env\n');
        setOutput(prev => prev + '  • Verify your API key is valid and has credits\n');
        setOutput(prev => prev + '  • Check your internet connection\n');
      } else if (errorMessage.includes('timeout')) {
        setOutput(prev => prev + '💡 Suggestions:\n');
        setOutput(prev => prev + '  • Try a simpler prompt\n');
        setOutput(prev => prev + '  • Try again in a few moments\n');
      } else if (errorMessage.includes('parse') || errorMessage.includes('JSON')) {
        setOutput(prev => prev + '💡 Suggestions:\n');
        setOutput(prev => prev + '  • The LLM returned invalid data\n');
        setOutput(prev => prev + '  • Try rephrasing your prompt to be more specific\n');
        setOutput(prev => prev + '  • This is usually a temporary issue - try again\n');
      } else {
        setOutput(prev => prev + '💡 Try:\n');
        setOutput(prev => prev + '  • Check the console for more details\n');
        setOutput(prev => prev + '  • Ensure all agents are properly configured\n');
        setOutput(prev => prev + '  • Try a different prompt\n');
      }
      
      setCurrentPhase('Error');
      console.error('Pipeline error:', error);
    } finally {
      setIsRunning(false);
    }
  };

  return (
    <div className="h-screen bg-background text-text-primary flex">
      {/* Left Panel - Pipeline Controls */}
      <div className="w-1/2 border-r border-border flex flex-col">
        {/* Header */}
        <div className="border-b border-border p-6">
          <h1 className="text-3xl font-bold mb-2">🎬 Pipeline</h1>
          <p className="text-text-secondary text-sm">
            Architect → Engineer → Quality → Debug
          </p>
        </div>

        {/* Input Section */}
        <div className="p-6 border-b border-border">
          <label className="block text-sm font-medium mb-3 text-text-secondary">
            What do you want to build?
          </label>
          <textarea
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder="e.g., Build a todo app with React and TypeScript"
            className="w-full h-32 bg-surface border border-border rounded-lg p-4 text-text-primary placeholder:text-text-tertiary focus:outline-none focus:border-glass-blue-500 resize-none"
            disabled={isRunning}
          />
          <div className="mt-4">
            <Button
              onClick={runPipeline}
              disabled={isRunning || !prompt.trim()}
              className="w-full"
            >
              {isRunning ? '⏳ Running Pipeline...' : '▶️ Run Pipeline'}
            </Button>
          </div>
        </div>

        {/* Status Section */}
        {currentPhase && (
          <div className="px-6 py-4 border-b border-border">
            <div className="flex items-center gap-3">
              <div className={`w-2 h-2 rounded-full ${
                isRunning ? 'bg-glass-blue-500 animate-pulse' : 
                currentPhase === 'Complete' ? 'bg-green-500' : 
                currentPhase === 'Error' ? 'bg-red-500' : 'bg-text-tertiary'
              }`} />
              <span className="text-sm font-medium">{currentPhase}</span>
              {isRunning && (
                <span className="text-xs text-text-tertiary ml-auto animate-pulse">
                  Processing...
                </span>
              )}
            </div>
            {isRunning && (
              <div className="mt-3 w-full bg-surface rounded-full h-1 overflow-hidden">
                <div className="h-full bg-glass-blue-500 animate-pulse" style={{ width: '100%' }} />
              </div>
            )}
          </div>
        )}

        {/* Output Log */}
        <div className="flex-1 overflow-hidden flex flex-col">
          <div className="px-6 py-3 border-b border-border">
            <h2 className="text-sm font-semibold text-text-secondary">Pipeline Log</h2>
          </div>
          <div ref={outputRef} className="flex-1 overflow-y-auto p-6">
            <pre className="text-xs font-mono text-text-secondary whitespace-pre-wrap">
              {output || 'Waiting to start...'}
            </pre>
          </div>
        </div>
      </div>

      {/* Right Panel - Generated Code */}
      <div className="w-1/2 flex flex-col">
        {/* Header */}
        <div className="border-b border-border p-6">
          <h2 className="text-xl font-bold">Generated Code</h2>
          <p className="text-text-secondary text-sm mt-1">
            {generatedFiles.length} file{generatedFiles.length !== 1 ? 's' : ''} generated
          </p>
        </div>

        {generatedFiles.length === 0 ? (
          <div className="flex-1 flex items-center justify-center text-text-tertiary">
            <div className="text-center">
              <div className="text-6xl mb-4">📄</div>
              <p>No files generated yet</p>
              <p className="text-sm mt-2">Run the pipeline to see generated code</p>
            </div>
          </div>
        ) : (
          <>
            {/* File Tabs */}
            <div className="border-b border-border overflow-x-auto">
              <div className="flex px-6 gap-2 py-2">
                {generatedFiles.map((file, index) => (
                  <button
                    key={index}
                    onClick={() => setSelectedFile(file)}
                    className={`px-4 py-2 rounded-t text-sm font-medium transition-colors whitespace-nowrap ${
                      selectedFile?.path === file.path
                        ? 'bg-surface text-text-primary border-t border-l border-r border-border'
                        : 'text-text-secondary hover:text-text-primary hover:bg-surface/50'
                    }`}
                  >
                    {file.path.split('/').pop()}
                  </button>
                ))}
              </div>
            </div>

            {/* Code Display */}
            <div className="flex-1 overflow-hidden flex flex-col">
              <AnimatePresence mode="wait">
                {selectedFile && (
                  <motion.div
                    key={selectedFile.path}
                    initial={{ opacity: 0, y: 10 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: -10 }}
                    transition={{ duration: 0.2 }}
                    className="flex-1 overflow-y-auto"
                  >
                    <div className="p-6">
                      <div className="mb-4 flex items-center justify-between">
                        <div>
                          <h3 className="font-mono text-sm text-text-secondary">{selectedFile.path}</h3>
                          <p className="text-xs text-text-tertiary mt-1">{selectedFile.language}</p>
                        </div>
                        <Button
                          variant="secondary"
                          onClick={() => {
                            navigator.clipboard.writeText(selectedFile.content);
                          }}
                          className="text-xs"
                        >
                          📋 Copy
                        </Button>
                      </div>
                      <Card className="bg-black/50">
                        <pre className="text-sm font-mono text-text-primary overflow-x-auto">
                          <code>{selectedFile.content}</code>
                        </pre>
                      </Card>
                    </div>
                  </motion.div>
                )}
              </AnimatePresence>
            </div>
          </>
        )}
      </div>
    </div>
  );
}
