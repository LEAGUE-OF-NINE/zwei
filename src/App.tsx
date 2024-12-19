import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Modal from "./components/Modal";
import SettingsButton from "./components/SettingsButton";
import StartLethe from "./components/StartLethe";
import UpdateLetheButton from "./components/UpdateLetheButton";
import UpdateLimbus from "./components/UpdateLimbus";
import ViewLetheFiles from "./components/ViewLetheFilesButton";
import { useErrorContext } from "./context/ErrorContext";
import { useEffect } from "react";
import { useToast } from "./context/ToastContext";

function App() {
  const { error, setError } = useErrorContext();
  const { showToast } = useToast();

  async function checkUpdate() {
    try {
      const upToDate = await invoke<boolean>("check_lethe_limbus_up_to_date");
      if (!upToDate) {
        showToast(
          "A new limbus version has been detected. Consider updating.",
          "alert-info",
          10000
        );
      }
    } catch (err) {
      console.error(err);
    }
  }

  useEffect(() => {
    checkUpdate();
  }, []);
  return (
    <main className="h-screen w-full flex flex-col items-center justify-center gap-8 p-4">
      <div className="container flex flex-col items-center">
        <img src="./Zwei.png" alt="Zwei: Your Shield" />
        <h1 className="text-3xl font-semibold">Zwei - Your Shield</h1>
        <Modal
          isOpen={error != null}
          title={"Error"}
          children={<p>{error}</p>}
          onClose={() => {
            setError(null);
          }}
        />
      </div>

      <div className="menu menu-horizontal">
        <UpdateLimbus />
        <UpdateLetheButton />
        <ViewLetheFiles />
      </div>
      <div className="flex w-full justify-center relative">
        <StartLethe />
        <SettingsButton />
      </div>
    </main>
  );
}

export default App;
