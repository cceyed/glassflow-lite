import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { AgentStatusBar } from '@/design-system/components/AgentStatusBar';

describe('AgentStatusBar Component', () => {
  const mockAgents = [
    { id: '1', name: 'Planner', status: 'active' as const, progress: 50 },
    { id: '2', name: 'Executor', status: 'idle' as const, progress: 0 },
    { id: '3', name: 'Validator', status: 'idle' as const, progress: 0 },
    { id: '4', name: 'Optimizer', status: 'complete' as const, progress: 100 },
  ];

  it('should render with correct height', () => {
    const { container } = render(
      <AgentStatusBar agents={mockAgents} />
    );
    const statusBar = container.firstChild as HTMLElement;
    expect(statusBar).toHaveStyle({ height: '48px' });
  });

  it('should display 4 agents', () => {
    render(<AgentStatusBar agents={mockAgents} />);
    expect(screen.getByText('Planner')).toBeInTheDocument();
    expect(screen.getByText('Executor')).toBeInTheDocument();
    expect(screen.getByText('Validator')).toBeInTheDocument();
    expect(screen.getByText('Optimizer')).toBeInTheDocument();
  });

  it('should show agent status', () => {
    render(<AgentStatusBar agents={mockAgents} />);
    expect(screen.getByText(/active/i)).toBeInTheDocument();
    expect(screen.getByText(/complete/i)).toBeInTheDocument();
  });

  it('should show progress for each agent', () => {
    const { container } = render(
      <AgentStatusBar agents={mockAgents} />
    );
    const progressBars = container.querySelectorAll('[role="progressbar"]');
    expect(progressBars.length).toBe(4);
  });

  it('should call onAgentClick when agent is clicked', () => {
    const onAgentClick = vi.fn();
    render(
      <AgentStatusBar agents={mockAgents} onAgentClick={onAgentClick} />
    );
    const agent = screen.getByText('Planner');
    fireEvent.click(agent);
    expect(onAgentClick).toHaveBeenCalledWith('1');
  });

  it('should expand reasoning panel on hover', () => {
    const agentsWithReasoning = [
      { ...mockAgents[0], reasoning: ['Step 1', 'Step 2'] },
      ...mockAgents.slice(1),
    ];
    const { container } = render(
      <AgentStatusBar agents={agentsWithReasoning} />
    );
    const agent = screen.getByText('Planner');
    fireEvent.mouseEnter(agent);
    expect(screen.getByText('Step 1')).toBeInTheDocument();
    expect(screen.getByText('Step 2')).toBeInTheDocument();
  });

  it('should show status LED for each agent', () => {
    const { container } = render(
      <AgentStatusBar agents={mockAgents} />
    );
    const statusLEDs = container.querySelectorAll('[data-testid="status-led"]');
    expect(statusLEDs.length).toBe(4);
  });
});
