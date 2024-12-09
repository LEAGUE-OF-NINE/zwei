import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { GlobalErrorProvider } from "./context/ErrorContext";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <GlobalErrorProvider>
      <App />
    </GlobalErrorProvider>
  </React.StrictMode>
);
