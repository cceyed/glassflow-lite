import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  Sparkles, 
  Cpu, 
  Code, 
  CheckCircle2, 
  Bug, 
  Send,
  Loader2,
  FileCode,
  Folder,
  FolderOpen,
  ChevronRight,
  FileJson,
  FileText
} from 'lucide-react';

interface Message {
  id: string;
  agent: 'architect' | 'engineer' | 'quality' | 'debug' | 'system' | 'user';
  content: string;
  timestamp: number;
}

interface GeneratedFile {
  path: string;
  content: string;
  language: string;
}

interface FileTreeNode {
  name: string;
  path: string;
  type: 'file' | 'folder';
  children?: FileTreeNode[];
  content?: string;
  language?: string;
}

const agentConfig = {
  architect: { icon: Sparkles, name: 'Architect', color: '#FFD700' },
  engineer: { icon: Code, name: 'Engineer', color: '#FFA500' },
  quality: { icon: CheckCircle2, name: 'Quality', color: '#FFB700' },
  debug: { icon: Bug, name: 'Debug', color: '#FF8C00' },
  system: { icon: Cpu, name: 'System', color: '#888888' },
  user: { icon: Send, name: 'You', color: '#FFFFFF' }
};

const loadingPhrases = [
  "Warming up the neural networks...",
  "Consulting the code oracle...",
  "Summoning the architect spirits...",
  "Brewing some fresh algorithms...",
  "Polishing the glass architecture...",
  "Channeling the flow of creativity...",
  "Awakening the agent collective...",
  "Initializing the dream sequence...",
  "Loading the multiverse of possibilities...",
  "Gathering cosmic inspiration..."
];

export function RevampedPipelineUI() {
  const [prompt, setPrompt] = useState('');
  const [messages, setMessages] = useState<Message[]>([]);
  const [isRunning, setIsRunning] = useState(false);
  const [currentAgent, setCurrentAgent] = useState<string>('');
  const [generatedFiles, setGeneratedFiles] = useState<GeneratedFile[]>([]);
  const [selectedFile, setSelectedFile] = useState<GeneratedFile | null>(null);
  const [fileTree, setFileTree] = useState<FileTreeNode[]>([]);
  const [loadingPhrase, setLoadingPhrase] = useState(loadingPhrases[0]);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  // Auto-scroll to bottom
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // Cycle loading phrases
  useEffect(() => {
    if (!isRunning) return;
    
    const interval = setInterval(() => {
      setLoadingPhrase(prev => {
        const currentIndex = loadingPhrases.indexOf(prev);
        const nextIndex = (currentIndex + 1) % loadingPhrases.length;
        return loadingPhrases[nextIndex];
      });
    }, 3000); // Change every 3 seconds

    return () => clearInterval(interval);
  }, [isRunning]);

  // Listen for orchestrator events
  useEffect(() => {
    console.log('[UI] Setting up orchestrator:reasoning listener');
    
    const unlisten = listen('orchestrator:reasoning', (event: any) => {
      console.log('[UI] Received event:', event.payload);
      const { content } = event.payload;
      
      // Determine which agent is speaking
      let agent: Message['agent'] = 'system';
      if (content.includes('Architect') || content.includes('Phase 1') || content.includes('Planning')) {
        agent = 'architect';
      } else if (content.includes('Engineer') || content.includes('Phase 2') || content.includes('Building')) {
        agent = 'engineer';
      } else if (content.includes('Quality') || content.includes('Phase 3') || content.includes('Validating')) {
        agent = 'quality';
      } else if (content.includes('Debug') || content.includes('Phase 4') || content.includes('Testing')) {
        agent = 'debug';
      }
      
      setCurrentAgent(agentConfig[agent].name);
      
      // Add message directly to avoid stale closure
      setMessages(prev => [...prev, {
        id: Date.now().toString() + Math.random(),
        agent,
        content,
        timestamp: Date.now()
      }]);
    });

    return () => {
      console.log('[UI] Cleaning up orchestrator:reasoning listener');
      unlisten.then(fn => fn());
    };
  }, []);

  const addMessage = (agent: Message['agent'], content: string) => {
    const message: Message = {
      id: Date.now().toString() + Math.random(),
      agent,
      content,
      timestamp: Date.now()
    };
    setMessages(prev => [...prev, message]);
  };

  // Build file tree from flat file list
  const buildFileTree = (files: GeneratedFile[]): FileTreeNode[] => {
    const root: { [key: string]: FileTreeNode } = {};
    
    files.forEach(file => {
      const parts = file.path.split('/');
      let current = root;
      
      parts.forEach((part, index) => {
        if (index === parts.length - 1) {
          // It's a file
          current[part] = {
            name: part,
            path: file.path,
            type: 'file',
            content: file.content,
            language: file.language
          };
        } else {
          // It's a folder
          if (!current[part]) {
            current[part] = {
              name: part,
              path: parts.slice(0, index + 1).join('/'),
              type: 'folder',
              children: []
            };
          }
          if (!current[part].children) {
            current[part].children = [];
          }
          // Navigate deeper
          const childrenObj: { [key: string]: FileTreeNode } = {};
          current[part].children!.forEach(child => {
            childrenObj[child.name] = child;
          });
          current = childrenObj;
        }
      });
    });
    
    // Convert object to array
    const convertToArray = (obj: { [key: string]: FileTreeNode }): FileTreeNode[] => {
      return Object.values(obj).map(node => {
        if (node.type === 'folder' && node.children) {
          const childrenObj: { [key: string]: FileTreeNode } = {};
          node.children.forEach(child => {
            childrenObj[child.name] = child;
          });
          node.children = convertToArray(childrenObj);
        }
        return node;
      });
    };
    
    return convertToArray(root);
  };

  const runPipeline = async () => {
    if (!prompt.trim() || isRunning) return;
    
    setIsRunning(true);
    setMessages([]);
    setGeneratedFiles([]);
    setCurrentAgent('Initializing');
    
    // Add user message
    addMessage('user', prompt);
    
    // Add system message with first loading phrase
    addMessage('system', loadingPhrases[0]);
    
    try {
      const response: any = await invoke('orchestrator_run_pipeline', {
        prompt: prompt.trim()
      });
      
      addMessage('system', response.message || 'Pipeline complete!');
      setCurrentAgent('Complete');
      
      // Use actual generated files from the response
      if (response.files && response.files.length > 0) {
        const files: GeneratedFile[] = response.files.map((f: any) => ({
          path: f.path,
          content: f.content,
          language: f.language
        }));
        setGeneratedFiles(files);
        setFileTree(buildFileTree(files));
        setSelectedFile(files[0]); // Select first file by default
        addMessage('system', `Generated ${files.length} file(s)`);
      } else {
        addMessage('system', 'No files were generated');
      }
      
    } catch (error: any) {
      const errorMsg = error?.message || String(error);
      addMessage('system', `Error: ${errorMsg}`);
      setCurrentAgent('Error');
    } finally {
      setIsRunning(false);
      setPrompt('');
      inputRef.current?.focus();
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      runPipeline();
    }
  };

  return (
    <div className="h-screen bg-black text-white flex flex-col overflow-hidden">
      {/* Header */}
      <div className="border-b border-gray-900 bg-gradient-to-r from-black via-gray-950 to-black">
        <div className="px-8 py-6">
          <div className="flex items-center gap-4">
            <motion.div
              animate={{ 
                rotate: isRunning ? 360 : 0,
                scale: isRunning ? [1, 1.1, 1] : 1
              }}
              transition={{ 
                rotate: { duration: 2, repeat: Infinity, ease: "linear" },
                scale: { duration: 1, repeat: Infinity }
              }}
            >
              <Sparkles className="w-8 h-8 text-yellow-500" />
            </motion.div>
            <div>
              <h1 className="text-2xl font-bold bg-gradient-to-r from-yellow-400 via-yellow-200 to-yellow-400 bg-clip-text text-transparent">
                Multi-Agent Pipeline
              </h1>
              <p className="text-sm text-gray-500 mt-1">
                {isRunning ? (
                  <span className="flex items-center gap-2">
                    <Loader2 className="w-3 h-3 animate-spin text-yellow-500" />
                    {currentAgent}
                  </span>
                ) : (
                  'Ready to build'
                )}
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex overflow-hidden">
        {/* Left Side - Agent Output */}
        <div className="flex-1 flex flex-col overflow-hidden border-r border-gray-900">
          {/* Current Agent Status */}
          <div className="px-8 py-4 border-b border-gray-900 bg-gradient-to-b from-gray-950 to-black">
            <AnimatePresence mode="wait">
              {currentAgent && (
                <motion.div
                  key={currentAgent}
                  initial={{ opacity: 0, y: -10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: 10 }}
                  className="flex items-center gap-3"
                >
                  <div className="w-2 h-2 rounded-full bg-yellow-500 animate-pulse shadow-lg shadow-yellow-500/50" />
                  <span className="text-sm font-medium text-gray-400">
                    Active Agent:
                  </span>
                  <span className="text-yellow-400 font-semibold">
                    {currentAgent}
                  </span>
                </motion.div>
              )}
            </AnimatePresence>
          </div>

          {/* Messages */}
          <div className="flex-1 overflow-y-auto px-8 py-6 space-y-6">
            <AnimatePresence initial={false}>
              {messages.map((message, index) => {
                const config = agentConfig[message.agent];
                const Icon = config.icon;
                
                return (
                  <motion.div
                    key={message.id}
                    initial={{ opacity: 0, y: 20, scale: 0.95 }}
                    animate={{ opacity: 1, y: 0, scale: 1 }}
                    exit={{ opacity: 0, scale: 0.95 }}
                    transition={{ 
                      duration: 0.5,
                      ease: [0.4, 0, 0.2, 1],
                      delay: Math.min(index * 0.1, 0.3)
                    }}
                    className="flex gap-4"
                  >
                    {/* Agent Icon */}
                    <motion.div
                      initial={{ scale: 0, rotate: -180 }}
                      animate={{ scale: 1, rotate: 0 }}
                      transition={{ 
                        type: "spring", 
                        stiffness: 200, 
                        damping: 15,
                        delay: Math.min(index * 0.1, 0.3) + 0.1
                      }}
                      className="flex-shrink-0"
                    >
                      <div 
                        className="w-10 h-10 rounded-full flex items-center justify-center"
                        style={{
                          background: `radial-gradient(circle, ${config.color}20, transparent)`,
                          border: `1px solid ${config.color}40`
                        }}
                      >
                        <Icon 
                          className="w-5 h-5" 
                          style={{ color: config.color }}
                        />
                      </div>
                    </motion.div>

                    {/* Message Content */}
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 mb-2">
                        <span 
                          className="text-sm font-semibold"
                          style={{ color: config.color }}
                        >
                          {config.name}
                        </span>
                        <span className="text-xs text-gray-600">
                          {new Date(message.timestamp).toLocaleTimeString()}
                        </span>
                      </div>
                      
                      {/* Animated Text */}
                      <AnimatedText 
                        text={message.content} 
                        delay={index * 0.05}
                      />
                    </div>
                  </motion.div>
                );
              })}
            </AnimatePresence>
            
            <div ref={messagesEndRef} />
          </div>
        </div>

        {/* Right Side - Input & Files */}
        <div className="w-[500px] flex flex-col bg-gradient-to-b from-gray-950 to-black">
          {/* Input Section */}
          <div className="p-8 border-b border-gray-900">
            <label className="block text-sm font-medium text-gray-400 mb-3">
              What do you want to build?
            </label>
            <div className="relative">
              <input
                ref={inputRef}
                type="text"
                value={prompt}
                onChange={(e) => setPrompt(e.target.value)}
                onKeyPress={handleKeyPress}
                placeholder="e.g., Build a todo app with React..."
                disabled={isRunning}
                className="w-full bg-gray-900 border border-gray-800 rounded-lg px-4 py-3 pr-12 text-white placeholder-gray-600 focus:outline-none focus:border-yellow-500/50 focus:ring-2 focus:ring-yellow-500/20 transition-all disabled:opacity-50"
              />
              <button
                onClick={runPipeline}
                disabled={isRunning || !prompt.trim()}
                className="absolute right-2 top-1/2 -translate-y-1/2 p-2 rounded-lg bg-gradient-to-r from-yellow-600 to-yellow-500 hover:from-yellow-500 hover:to-yellow-400 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
              >
                {isRunning ? (
                  <Loader2 className="w-4 h-4 animate-spin" />
                ) : (
                  <Send className="w-4 h-4" />
                )}
              </button>
            </div>
            
            {isRunning && (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="mt-3"
              >
                <div className="text-sm font-medium bg-gradient-to-r from-white via-yellow-200 to-white bg-clip-text text-transparent animate-shimmer bg-[length:200%_100%]">
                  {loadingPhrase}
                </div>
                <p className="text-xs text-gray-600 mt-1">This may take 30-60 seconds</p>
              </motion.div>
            )}
          </div>

          {/* File Tree & Viewer */}
          <div className="flex-1 overflow-hidden flex flex-col">
            {generatedFiles.length === 0 ? (
              <div className="flex flex-col items-center justify-center h-full text-gray-600">
                <Code className="w-12 h-12 mb-4 opacity-30" />
                <p className="text-sm">No files generated yet</p>
              </div>
            ) : (
              <>
                {/* File Tree */}
                <div className="h-1/3 border-b border-gray-900 overflow-hidden flex flex-col bg-gradient-to-b from-black to-gray-950">
                  <div className="px-8 py-4 border-b border-gray-900 flex items-center justify-between">
                    <h3 className="text-sm font-semibold text-gray-400">
                      Files ({generatedFiles.length})
                    </h3>
                    <div className="text-xs text-gray-600">Click to view</div>
                  </div>
                  <div className="flex-1 overflow-y-auto px-4 py-4 scrollbar-thin scrollbar-track-transparent scrollbar-thumb-gray-800 hover:scrollbar-thumb-gray-700">
                    <FileTree 
                      nodes={fileTree} 
                      onSelectFile={(file) => setSelectedFile(file)}
                      selectedPath={selectedFile?.path}
                    />
                  </div>
                </div>

                {/* File Viewer */}
                <div className="flex-1 overflow-hidden flex flex-col bg-black">
                  {selectedFile ? (
                    <>
                      <div className="px-8 py-4 border-b border-gray-900 bg-gradient-to-r from-gray-950 to-black">
                        <div className="flex items-center gap-3">
                          <FileCode className="w-5 h-5 text-yellow-500" />
                          <div className="flex-1 min-w-0">
                            <div className="text-sm font-mono text-gray-300 truncate">
                              {selectedFile.path}
                            </div>
                            <div className="text-xs text-gray-600 mt-0.5">
                              {selectedFile.language} • {selectedFile.content.split('\n').length} lines
                            </div>
                          </div>
                        </div>
                      </div>
                      <div className="flex-1 overflow-y-auto p-8 scrollbar-thin scrollbar-track-transparent scrollbar-thumb-gray-800 hover:scrollbar-thumb-gray-700">
                        <motion.pre 
                          initial={{ opacity: 0 }}
                          animate={{ opacity: 1 }}
                          className="text-xs font-mono text-gray-300 leading-relaxed"
                        >
                          {selectedFile.content}
                        </motion.pre>
                      </div>
                    </>
                  ) : (
                    <div className="flex flex-col items-center justify-center h-full text-gray-600">
                      <FileCode className="w-16 h-16 mb-4 opacity-20" />
                      <p className="text-sm">Select a file to view its contents</p>
                    </div>
                  )}
                </div>
              </>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

// File Tree Component
interface FileTreeProps {
  nodes: FileTreeNode[];
  onSelectFile: (file: GeneratedFile) => void;
  selectedPath?: string;
  level?: number;
}

function FileTree({ nodes, onSelectFile, selectedPath, level = 0 }: FileTreeProps) {
  const [expandedFolders, setExpandedFolders] = useState<Set<string>>(new Set());

  const toggleFolder = (path: string) => {
    setExpandedFolders(prev => {
      const next = new Set(prev);
      if (next.has(path)) {
        next.delete(path);
      } else {
        next.add(path);
      }
      return next;
    });
  };

  const getFileIcon = (name: string) => {
    if (name.endsWith('.json')) return FileJson;
    if (name.endsWith('.md') || name.endsWith('.txt')) return FileText;
    return FileCode;
  };

  return (
    <div className="space-y-0.5">
      {nodes.map((node) => {
        const isExpanded = expandedFolders.has(node.path);
        const isSelected = node.path === selectedPath;
        const Icon = node.type === 'folder' 
          ? (isExpanded ? FolderOpen : Folder)
          : getFileIcon(node.name);

        return (
          <div key={node.path}>
            <motion.div
              initial={{ opacity: 0, x: -10 }}
              animate={{ opacity: 1, x: 0 }}
              whileHover={{ x: 2 }}
              transition={{ duration: 0.15 }}
              className={`flex items-center gap-2 px-3 py-2 rounded-md cursor-pointer transition-all ${
                isSelected 
                  ? 'bg-gradient-to-r from-yellow-500/20 to-yellow-600/10 text-yellow-400 shadow-lg shadow-yellow-500/10' 
                  : 'hover:bg-gray-800/50 text-gray-400 hover:text-gray-300'
              }`}
              style={{ paddingLeft: `${level * 16 + 12}px` }}
              onClick={() => {
                if (node.type === 'folder') {
                  toggleFolder(node.path);
                } else if (node.content) {
                  onSelectFile({
                    path: node.path,
                    content: node.content,
                    language: node.language || 'text'
                  });
                }
              }}
            >
              {node.type === 'folder' && (
                <motion.div
                  animate={{ rotate: isExpanded ? 90 : 0 }}
                  transition={{ duration: 0.2, ease: "easeOut" }}
                >
                  <ChevronRight className="w-3.5 h-3.5 text-gray-500" />
                </motion.div>
              )}
              <Icon className={`w-4 h-4 flex-shrink-0 ${
                node.type === 'folder' 
                  ? 'text-yellow-600' 
                  : isSelected 
                    ? 'text-yellow-400' 
                    : 'text-yellow-500/70'
              }`} />
              <span className={`text-sm font-mono truncate ${
                isSelected ? 'font-semibold' : ''
              }`}>{node.name}</span>
            </motion.div>

            {node.type === 'folder' && isExpanded && node.children && (
              <motion.div
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: 'auto' }}
                exit={{ opacity: 0, height: 0 }}
                transition={{ duration: 0.2 }}
              >
                <FileTree
                  nodes={node.children}
                  onSelectFile={onSelectFile}
                  selectedPath={selectedPath}
                  level={level + 1}
                />
              </motion.div>
            )}
          </div>
        );
      })}
    </div>
  );
}

// Animated Text Component with gold glow effect
function AnimatedText({ text, delay = 0 }: { text: string; delay?: number }) {
  const words = text.split(' ');
  
  return (
    <div className="text-sm leading-relaxed">
      {words.map((word, index) => (
        <motion.span
          key={index}
          initial={{ opacity: 0, color: '#FFD700' }}
          animate={{ 
            opacity: 1,
            color: ['#FFD700', '#FFA500', '#FFFFFF']
          }}
          transition={{
            opacity: { duration: 0.1, delay: delay + index * 0.03 },
            color: { duration: 2, delay: delay + index * 0.03 }
          }}
          className="inline-block mr-1"
          style={{
            textShadow: '0 0 10px rgba(255, 215, 0, 0.3)'
          }}
        >
          {word}
        </motion.span>
      ))}
    </div>
  );
}
