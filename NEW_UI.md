# New Simplified Pipeline UI

## Overview
Created a clean, focused UI that displays the pipeline on the left and generated code on the right.

## Features

### Left Panel - Pipeline Controls
- **Header**: Shows pipeline name and agent flow (Architect → Engineer → Quality → Debug)
- **Input Section**: Large textarea for entering what you want to build
- **Run Button**: Starts the pipeline with visual feedback
- **Status Indicator**: Shows current phase with animated dot
  - Blue pulsing dot when running
  - Green when complete
  - Red on error
- **Pipeline Log**: Real-time output showing orchestrator reasoning events

### Right Panel - Generated Code
- **File Counter**: Shows how many files were generated
- **File Tabs**: Click to switch between generated files
- **Code Display**: Syntax-highlighted code view with:
  - File path and language info
  - Copy button for easy clipboard access
  - Smooth animations when switching files
- **Empty State**: Friendly message when no files generated yet

## Design System Integration
- Uses all design system tokens (colors, spacing, typography)
- Follows monochrome + glass blue accent pattern
- Smooth animations with Framer Motion
- Responsive button and card components

## Technical Details
- Listens to `orchestrator:reasoning` events for real-time updates
- Extracts phase information from event content
- Clean split-panel layout (50/50)
- Proper TypeScript types for generated files

## Usage
1. Enter your prompt (e.g., "Build a todo app with React and TypeScript")
2. Click "Run Pipeline"
3. Watch the pipeline log in real-time
4. View generated code on the right
5. Click file tabs to switch between files
6. Copy code with the copy button

## Next Steps
- Hook up actual generated files from the pipeline response
- Add syntax highlighting for different languages
- Add download/export functionality
- Add ability to edit and re-run
