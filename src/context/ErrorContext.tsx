import React, { createContext, useContext, useState, ReactNode } from "react";

interface ErrorContextType {
  error: string | null;
  setError: (error: string | null) => void;
}

const ErrorContext = createContext<ErrorContextType | undefined>(undefined);

interface GlobalErrorProviderProps {
  children: ReactNode;
}

export const GlobalErrorProvider: React.FC<GlobalErrorProviderProps> = ({
  children,
}) => {
  const [error, setError] = useState<string | null>(null);

  return (
    <ErrorContext.Provider value={{ error, setError }}>
      {children}
    </ErrorContext.Provider>
  );
};

export const useErrorContext = (): ErrorContextType => {
  const context = useContext(ErrorContext);
  if (!context) {
    throw new Error(
      "useErrorContext must be used within a GlobalErrorProvider"
    );
  }
  return context;
};

