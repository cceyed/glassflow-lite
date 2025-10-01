import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { CommandInput } from '@/design-system/components/CommandInput';

describe('CommandInput Component', () => {
  it('should render with correct height', () => {
    const { container } = render(
      <CommandInput 
        value=""
        onChange={vi.fn()}
        onSubmit={vi.fn()}
      />
    );
    const input = container.querySelector('input');
    expect(input?.parentElement?.parentElement).toHaveStyle({ height: '60px' });
  });

  it('should display placeholder text', () => {
    render(
      <CommandInput 
        value=""
        onChange={vi.fn()}
        onSubmit={vi.fn()}
        placeholder="Enter command..."
      />
    );
    expect(screen.getByPlaceholderText('Enter command...')).toBeInTheDocument();
  });

  it('should call onChange when typing', () => {
    const onChange = vi.fn();
    render(
      <CommandInput 
        value=""
        onChange={onChange}
        onSubmit={vi.fn()}
      />
    );
    const input = screen.getByRole('textbox');
    fireEvent.change(input, { target: { value: 'test' } });
    expect(onChange).toHaveBeenCalled();
  });

  it('should call onSubmit when Enter key pressed', () => {
    const onSubmit = vi.fn();
    render(
      <CommandInput 
        value="test command"
        onChange={vi.fn()}
        onSubmit={onSubmit}
      />
    );
    const input = screen.getByRole('textbox');
    fireEvent.keyDown(input, { key: 'Enter', code: 'Enter' });
    expect(onSubmit).toHaveBeenCalledWith('test command');
  });

  it('should have 18px font size', () => {
    render(
      <CommandInput 
        value=""
        onChange={vi.fn()}
        onSubmit={vi.fn()}
      />
    );
    const input = screen.getByRole('textbox');
    expect(input).toHaveClass('text-large');
  });

  it('should show shimmer effect on focus', () => {
    const { container } = render(
      <CommandInput 
        value=""
        onChange={vi.fn()}
        onSubmit={vi.fn()}
      />
    );
    const input = screen.getByRole('textbox');
    fireEvent.focus(input);
    const wrapper = container.querySelector('.border-shimmer');
    expect(wrapper).toBeInTheDocument();
  });

  it('should show blue glow on focus', () => {
    const { container } = render(
      <CommandInput 
        value=""
        onChange={vi.fn()}
        onSubmit={vi.fn()}
      />
    );
    const input = screen.getByRole('textbox');
    fireEvent.focus(input);
    const wrapper = container.querySelector('.glass-blue-glow');
    expect(wrapper).toBeInTheDocument();
  });
});
