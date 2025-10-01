import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { StatusBar } from '@/design-system/components/StatusBar';

describe('StatusBar Component', () => {
  it('should render with correct height', () => {
    const { container } = render(
      <StatusBar 
        projectPath="/path/to/project"
        currentPhase="Implementation"
        timeElapsed="00:15:30"
      />
    );
    const statusBar = container.firstChild as HTMLElement;
    expect(statusBar).toHaveStyle({ height: '32px' });
  });

  it('should display project path on the left', () => {
    render(
      <StatusBar 
        projectPath="/path/to/project"
        currentPhase="Implementation"
        timeElapsed="00:15:30"
      />
    );
    expect(screen.getByText('/path/to/project')).toBeInTheDocument();
  });

  it('should display current phase in center', () => {
    render(
      <StatusBar 
        projectPath="/path/to/project"
        currentPhase="Implementation"
        timeElapsed="00:15:30"
      />
    );
    expect(screen.getByText('Implementation')).toBeInTheDocument();
  });

  it('should display time elapsed on the right', () => {
    render(
      <StatusBar 
        projectPath="/path/to/project"
        currentPhase="Implementation"
        timeElapsed="00:15:30"
      />
    );
    expect(screen.getByText('00:15:30')).toBeInTheDocument();
  });

  it('should have three sections', () => {
    const { container } = render(
      <StatusBar 
        projectPath="/path/to/project"
        currentPhase="Implementation"
        timeElapsed="00:15:30"
      />
    );
    const sections = container.querySelectorAll('[data-testid="status-section"]');
    expect(sections.length).toBe(3);
  });

  it('should have black background', () => {
    const { container } = render(
      <StatusBar 
        projectPath="/path/to/project"
        currentPhase="Implementation"
        timeElapsed="00:15:30"
      />
    );
    const statusBar = container.firstChild as HTMLElement;
    expect(statusBar).toHaveClass('bg-background');
  });

  it('should have top border', () => {
    const { container } = render(
      <StatusBar 
        projectPath="/path/to/project"
        currentPhase="Implementation"
        timeElapsed="00:15:30"
      />
    );
    const statusBar = container.firstChild as HTMLElement;
    expect(statusBar).toHaveClass('border-t');
  });
});
