import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/react';
import { Button, Card, TitleBar } from '@/design-system/components';

describe('Component-Token Integration', () => {
  it('should use design system colors in components', () => {
    const { container } = render(<Card>Test</Card>);
    const card = container.firstChild as HTMLElement;
    expect(card).toHaveClass('bg-surface');
    expect(card).toHaveClass('border-border');
  });

  it('should use design system spacing in components', () => {
    const { container } = render(<Card>Test</Card>);
    const card = container.firstChild as HTMLElement;
    expect(card).toHaveClass('p-6'); // 24px padding
  });

  it('should use glass blue for branding in TitleBar', () => {
    const { container } = render(
      <TitleBar projectName="Test" />
    );
    const logo = container.querySelector('[data-testid="titlebar-logo"]');
    expect(logo).toBeInTheDocument();
  });

  it('should apply correct typography classes', () => {
    const { container } = render(<Button>Click</Button>);
    const button = container.querySelector('button');
    expect(button).toHaveClass('font-semibold');
  });

  it('should use monochrome colors (not glass blue) for most UI', () => {
    const { container } = render(<Card>Content</Card>);
    const card = container.firstChild as HTMLElement;
    // Should use surface color, not glass blue
    expect(card.className).not.toContain('glass-blue');
  });
});
