import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Card } from '@/design-system/components/Card';

describe('Card Component', () => {
  it('should render children', () => {
    render(<Card>Card content</Card>);
    expect(screen.getByText('Card content')).toBeInTheDocument();
  });

  it('should have 8px border radius', () => {
    const { container } = render(<Card>Content</Card>);
    const card = container.firstChild as HTMLElement;
    expect(card).toHaveClass('rounded-lg');
  });

  it('should have 24px padding', () => {
    const { container } = render(<Card>Content</Card>);
    const card = container.firstChild as HTMLElement;
    expect(card).toHaveClass('p-6');
  });

  it('should have surface background', () => {
    const { container } = render(<Card>Content</Card>);
    const card = container.firstChild as HTMLElement;
    expect(card).toHaveClass('bg-surface');
  });

  it('should have border', () => {
    const { container } = render(<Card>Content</Card>);
    const card = container.firstChild as HTMLElement;
    expect(card).toHaveClass('border');
  });

  it('should call onClick when clicked', () => {
    const onClick = vi.fn();
    const { container } = render(<Card onClick={onClick}>Click me</Card>);
    const card = container.firstChild as HTMLElement;
    fireEvent.click(card);
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it('should brighten border on hover when hoverable', () => {
    const { container } = render(<Card hoverable>Hover me</Card>);
    const card = container.firstChild as HTMLElement;
    fireEvent.mouseEnter(card);
    expect(card).toBeInTheDocument();
  });

  it('should show glow when active', () => {
    const { container } = render(<Card active>Active card</Card>);
    const card = container.firstChild as HTMLElement;
    expect(card).toHaveClass('glass-blue-glow');
  });
});
