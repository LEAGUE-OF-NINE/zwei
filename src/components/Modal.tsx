import React, { ReactNode } from "react";

interface ModalProps {
  isOpen: boolean;
  title: string;
  children: ReactNode;
  onClose: () => void;
  actions?: ReactNode;
}

const Modal: React.FC<ModalProps> = ({ isOpen, title, children, onClose, actions }) => {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
      <div className="modal-box">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-lg font-bold">{title}</h2>
          <button
            className="btn btn-sm btn-ghost"
            onClick={onClose}
            aria-label="Close"
          >
            ✕
          </button>
        </div>
        <div className="mb-4">{children}</div>
        {actions && <div className="flex justify-end space-x-2">{actions}</div>}
      </div>
    </div>
  );
};

export default Modal;