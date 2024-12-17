import React, {
  createContext,
  useContext,
  useState,
  ReactNode,
  useRef,
} from "react";

interface ToastContextType {
  showToast: (message: string, type?: ToastType, duration?: number) => void;
}

type ToastType = "alert-info" | "alert-error" | "alert-warn" | "alert-success";

const ToastContext = createContext<ToastContextType | undefined>(undefined);

interface ToastProviderProps {
  children: ReactNode;
}

interface ToastState {
  message: string;
  type: ToastType;
}

export const ToastProvider: React.FC<ToastProviderProps> = ({ children }) => {
  const [toast, setToast] = useState<ToastState | null>(null);
  const timeoutRef = useRef<number | null>(null);

  const showToast = (
    message: string,
    type: ToastType = "alert-info",
    duration: number = 3000
  ) => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }

    setToast({ message, type });

    timeoutRef.current = setTimeout(() => {
      setToast(null);
      timeoutRef.current = null;
    }, duration);
  };
  return (
    <ToastContext.Provider value={{ showToast }}>
      {children}
      {toast && (
        <div className="toast toast-top toast-center z-50">
          <div className={`alert ${toast.type}`}>
            <span>{toast.message}</span>
          </div>
        </div>
      )}
    </ToastContext.Provider>
  );
};

// Custom hook to use the Toast Context
export const useToast = (): ToastContextType => {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error("useToast must be used within a ToastProvider");
  }
  return context;
};
