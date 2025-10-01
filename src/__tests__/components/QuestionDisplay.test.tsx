// Component test for QuestionDisplay
import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';

describe('QuestionDisplay', () => {
  it('renders single-choice question with options', () => {
    // const question = {
    //   text: 'Which state management?',
    //   type: 'single-choice',
    //   options: [{ value: 'zustand', label: 'Zustand' }]
    // };
    // render(<QuestionDisplay question={question} />);
    // expect(screen.getByText('Which state management?')).toBeInTheDocument();
    throw new Error('Test not implemented - QuestionDisplay component does not exist yet');
  });

  it('highlights default answer', () => {
    // const question = { recommendedAnswer: 'zustand', ... };
    // render(<QuestionDisplay question={question} />);
    // expect(screen.getByText(/Recommended/)).toBeInTheDocument();
    throw new Error('Test not implemented');
  });

  it('shows progress indicator', () => {
    // render(<QuestionDisplay question={q} progress={{ current: 3, total: 7 }} />);
    // expect(screen.getByText('Question 3 of 7')).toBeInTheDocument();
    throw new Error('Test not implemented');
  });
});
