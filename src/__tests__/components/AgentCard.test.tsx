// Component test for AgentCard
import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';

describe('AgentCard', () => {
  it('renders IDLE state with hollow circle indicator', () => {
    // const { container } = render(<AgentCard state="idle" confidence={0} />);
    // expect(screen.getByText('○')).toBeInTheDocument();
    // expect(screen.getByText('Ready')).toBeInTheDocument();
    throw new Error('Test not implemented - AgentCard component does not exist yet');
  });

  it('renders ANALYZING state with half circle indicator', () => {
    // const { container } = render(<AgentCard state="analyzing" confidence={0} />);
    // expect(screen.getByText('◐')).toBeInTheDocument();
    throw new Error('Test not implemented');
  });

  it('displays confidence percentage correctly', () => {
    // render(<AgentCard state="complete" confidence={94} />);
    // expect(screen.getByText('94%')).toBeInTheDocument();
    throw new Error('Test not implemented');
  });

  it('applies glass-blue-glow for active states', () => {
    // const { container } = render(<AgentCard state="designing" confidence={0} />);
    // expect(container.firstChild).toHaveClass('glass-blue-glow');
    throw new Error('Test not implemented');
  });
});
