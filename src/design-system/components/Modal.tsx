import React, { useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { BaseComponentProps } from '@/types/design-system';

interface ModalProps extends BaseComponentProps {
  isOpen: boolean;
  onClose: () => void;
  children: React.ReactNode;
  closeOnOverlayClick?: boolean;
  closeOnEscape?: boolean;
}

export const Modal: React.FC<ModalProps> = ({
  isOpen,
  onClose,
  children,
  closeOnOverlayClick = true,
  closeOnEscape = true,
  className = '',
  testId = 'modal',
}) => {
  useEffect(() => {
    if (!closeOnEscape) return;

    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };

    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
    }

    return () => {
      document.removeEventListener('keydown', handleEscape);
    };
  }, [isOpen, onClose, closeOnEscape]);

  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div
          data-testid={testId}
          className="fixed inset-0 z-50 flex items-center justify-center"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
        >
          {/* Overlay */}
          <motion.div
            data-testid="modal-overlay"
            className="absolute inset-0 bg-black/90 backdrop-blur-xl"
            onClick={closeOnOverlayClick ? onClose : undefined}
          />

          {/* Content */}
          <motion.div
            className={`relative z-10 glass-effect p-8 rounded-lg max-w-2xl w-full mx-4 ${className}`}
            initial={{ scale: 0.9, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            exit={{ scale: 0.9, opacity: 0 }}
            transition={{ duration: 0.2 }}
            onClick={(e) => e.stopPropagation()}
          >
            {children}
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};
