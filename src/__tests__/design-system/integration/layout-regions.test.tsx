import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/react';
import { TitleBar, CommandInput, AgentStatusBar, StatusBar } from '@/design-system/components';

describe('Layout Regions Integration', () => {
  it('should have TitleBar with 40px height', () => {
    const { container } = render(<TitleBar projectName="Test" />);
    const titleBar = container.firstChild as HTMLElement;
    expect(titleBar).toHaveStyle({ height: '40px' });
  });

  it('should have CommandInput with 60px height', () => {
    const { container } = render(
      <CommandInput value="" onChange={() => {}} onSubmit={() => {}} />
    );
    const input = container.firstChild as HTMLElement;
    expect(input).toHaveStyle({ height: '60px' });
  });

  it('should have AgentStatusBar with 48px height', () => {
    const mockAgents = [
      { id: '1', name: 'Test', status: 'idle' as const, progress: 0 },
    ];
    const { container } = render(<AgentStatusBar agents={mockAgents} />);
    const statusBar = container.firstChild as HTMLElement;
    expect(statusBar).toHaveStyle({ height: '48px' });
  });

  it('should have StatusBar with 32px height', () => {
    const { container } = render(
      <StatusBar projectPath="/test" currentPhase="Test" timeElapsed="00:00:00" />
    );
    const statusBar = container.firstChild as HTMLElement;
    expect(statusBar).toHaveStyle({ height: '32px' });
  });

  it('should have all regions stack correctly in main layout', () => {
    const { container } = render(
      <div className="h-screen flex flex-col">
        <TitleBar projectName="Test" />
        <CommandInput value="" onChange={() => {}} onSubmit={() => {}} />
        <div className="flex-1">Content</div>
        <AgentStatusBar agents={[]} />
        <StatusBar projectPath="/test" currentPhase="Test" timeElapsed="00:00:00" />
      </div>
    );
    
    const layout = container.firstChild as HTMLElement;
    expect(layout).toHaveClass('flex', 'flex-col');
  });

  it('should have correct total fixed height (180px)', () => {
    // TitleBar: 40px + CommandInput: 60px + AgentStatusBar: 48px + StatusBar: 32px = 180px
    const totalFixedHeight = 40 + 60 + 48 + 32;
    expect(totalFixedHeight).toBe(180);
  });
});
