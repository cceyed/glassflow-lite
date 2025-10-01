import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { TitleBar } from '@/design-system/components/TitleBar';

describe('TitleBar Component', () => {
  it('should render with correct height', () => {
    const { container } = render(
      <TitleBar 
        projectName="Test Project"
        onMinimize={vi.fn()}
        onMaximize={vi.fn()}
        onClose={vi.fn()}
      />
    );
    const titleBar = container.firstChild as HTMLElement;
    expect(titleBar).toHaveStyle({ height: '40px' });
  });

  it('should display project name in center', () => {
    render(
      <TitleBar 
        projectName="Test Project"
        onMinimize={vi.fn()}
        onMaximize={vi.fn()}
        onClose={vi.fn()}
      />
    );
    expect(screen.getByText('Test Project')).toBeInTheDocument();
  });

  it('should have logo on the left', () => {
    const { container } = render(
      <TitleBar 
        projectName="Test Project"
        onMinimize={vi.fn()}
        onMaximize={vi.fn()}
        onClose={vi.fn()}
      />
    );
    const logo = container.querySelector('[data-testid="titlebar-logo"]');
    expect(logo).toBeInTheDocument();
  });

  it('should have window controls on the right', () => {
    const { container } = render(
      <TitleBar 
        projectName="Test Project"
        onMinimize={vi.fn()}
        onMaximize={vi.fn()}
        onClose={vi.fn()}
      />
    );
    const controls = container.querySelector('[data-testid="window-controls"]');
    expect(controls).toBeInTheDocument();
  });

  it('should call onMinimize when minimize button clicked', () => {
    const onMinimize = vi.fn();
    render(
      <TitleBar 
        projectName="Test Project"
        onMinimize={onMinimize}
        onMaximize={vi.fn()}
        onClose={vi.fn()}
      />
    );
    const minimizeBtn = screen.getByRole('button', { name: /minimize/i });
    minimizeBtn.click();
    expect(onMinimize).toHaveBeenCalledTimes(1);
  });

  it('should call onMaximize when maximize button clicked', () => {
    const onMaximize = vi.fn();
    render(
      <TitleBar 
        projectName="Test Project"
        onMinimize={vi.fn()}
        onMaximize={onMaximize}
        onClose={vi.fn()}
      />
    );
    const maximizeBtn = screen.getByRole('button', { name: /maximize/i });
    maximizeBtn.click();
    expect(onMaximize).toHaveBeenCalledTimes(1);
  });

  it('should call onClose when close button clicked', () => {
    const onClose = vi.fn();
    render(
      <TitleBar 
        projectName="Test Project"
        onMinimize={vi.fn()}
        onMaximize={vi.fn()}
        onClose={onClose}
      />
    );
    const closeBtn = screen.getByRole('button', { name: /close/i });
    closeBtn.click();
    expect(onClose).toHaveBeenCalledTimes(1);
  });
});
