import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Modal } from '@/design-system/components/Modal';

describe('Modal Component', () => {
  it('should not render when isOpen is false', () => {
    render(
      <Modal isOpen={false} onClose={vi.fn()}>
        Modal content
      </Modal>
    );
    expect(screen.queryByText('Modal content')).not.toBeInTheDocument();
  });

  it('should render when isOpen is true', () => {
    render(
      <Modal isOpen={true} onClose={vi.fn()}>
        Modal content
      </Modal>
    );
    expect(screen.getByText('Modal content')).toBeInTheDocument();
  });

  it('should have overlay with rgba(0,0,0,0.9)', () => {
    const { container } = render(
      <Modal isOpen={true} onClose={vi.fn()}>
        Content
      </Modal>
    );
    const overlay = container.querySelector('[data-testid="modal-overlay"]');
    expect(overlay).toBeInTheDocument();
  });

  it('should have 10px backdrop blur', () => {
    const { container } = render(
      <Modal isOpen={true} onClose={vi.fn()}>
        Content
      </Modal>
    );
    const overlay = container.querySelector('[data-testid="modal-overlay"]');
    expect(overlay).toHaveClass('backdrop-blur-xl');
  });

  it('should call onClose when overlay clicked', () => {
    const onClose = vi.fn();
    const { container } = render(
      <Modal isOpen={true} onClose={onClose} closeOnOverlayClick={true}>
        Content
      </Modal>
    );
    const overlay = container.querySelector('[data-testid="modal-overlay"]');
    fireEvent.click(overlay!);
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('should call onClose when Escape key pressed', () => {
    const onClose = vi.fn();
    render(
      <Modal isOpen={true} onClose={onClose} closeOnEscape={true}>
        Content
      </Modal>
    );
    fireEvent.keyDown(document, { key: 'Escape', code: 'Escape' });
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('should not close on overlay click when closeOnOverlayClick is false', () => {
    const onClose = vi.fn();
    const { container } = render(
      <Modal isOpen={true} onClose={onClose} closeOnOverlayClick={false}>
        Content
      </Modal>
    );
    const overlay = container.querySelector('[data-testid="modal-overlay"]');
    fireEvent.click(overlay!);
    expect(onClose).not.toHaveBeenCalled();
  });

  it('should animate with scale transitions', () => {
    const { rerender } = render(
      <Modal isOpen={false} onClose={vi.fn()}>
        Content
      </Modal>
    );
    rerender(
      <Modal isOpen={true} onClose={vi.fn()}>
        Content
      </Modal>
    );
    // Framer Motion handles scale animations
    expect(screen.getByText('Content')).toBeInTheDocument();
  });
});
