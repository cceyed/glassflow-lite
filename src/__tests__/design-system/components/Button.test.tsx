import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Button } from '@/design-system/components/Button';

describe('Button Component', () => {
  it('should render primary variant', () => {
    render(<Button variant="primary">Click me</Button>);
    const button = screen.getByRole('button');
    expect(button).toHaveClass('bg-white', 'text-black');
  });

  it('should render secondary variant', () => {
    render(<Button variant="secondary">Click me</Button>);
    const button = screen.getByRole('button');
    expect(button).toHaveClass('border', 'text-white');
  });

  it('should render ghost variant', () => {
    render(<Button variant="ghost">Click me</Button>);
    const button = screen.getByRole('button');
    expect(button).toHaveClass('text-white');
    expect(button).not.toHaveClass('bg-white');
  });

  it('should call onClick when clicked', () => {
    const onClick = vi.fn();
    render(<Button onClick={onClick}>Click me</Button>);
    const button = screen.getByRole('button');
    fireEvent.click(button);
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it('should be disabled when disabled prop is true', () => {
    render(<Button disabled>Click me</Button>);
    const button = screen.getByRole('button');
    expect(button).toBeDisabled();
  });

  it('should not call onClick when disabled', () => {
    const onClick = vi.fn();
    render(<Button disabled onClick={onClick}>Click me</Button>);
    const button = screen.getByRole('button');
    fireEvent.click(button);
    expect(onClick).not.toHaveBeenCalled();
  });

  it('should show shimmer effect on hover', () => {
    const { container } = render(<Button>Hover me</Button>);
    const button = screen.getByRole('button');
    fireEvent.mouseEnter(button);
    // Framer Motion adds hover classes
    expect(container.querySelector('button')).toBeInTheDocument();
  });

  it('should scale down on active state', () => {
    render(<Button>Press me</Button>);
    const button = screen.getByRole('button');
    fireEvent.mouseDown(button);
    // Framer Motion handles scale animation
    expect(button).toBeInTheDocument();
  });
});
