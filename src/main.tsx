import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { GlobalErrorProvider } from "./context/ErrorContext";
import { ToastProvider } from "./context/ToastContext";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <GlobalErrorProvider>
      <ToastProvider>
        <App />
      </ToastProvider>
    </GlobalErrorProvider>
  </React.StrictMode>
);
