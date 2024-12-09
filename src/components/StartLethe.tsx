import { invoke } from "@tauri-apps/api/core";
import { load } from "@tauri-apps/plugin-store";
import { useErrorHandler } from "../context/useErrorHandler";

const StartLethe = () => {
  const handleError = useErrorHandler();
  async function startLethe() {
    try {
      const randomNumber = Math.floor(Math.random() * (6000 - 3000 + 1)) + 3000;
      const store = await load("store.json");
      const launchArgs = await store.get<{ value: string }>("launchArgs");
      const sandbox = await store.get<{ value: boolean }>("sandbox");
      const sandboxPath = await store.get<{ value: string }>("sandboxPath");
      await invoke("start_login_server", {
        port: randomNumber,
        launchArgs: launchArgs?.value,
        useSandbox: sandbox?.value,
        sandboxPath: sandboxPath?.value,
      });
    } catch (error) {
      console.error("Failed to Start lethe:", error);
      handleError(error);
    }
  }
  return (
    <button
      className="btn btn-primary w-full max-w-sm text-xl font-semibold rounded-tr-none rounded-br-none"
      onClick={startLethe}
    >
      Launch
    </button>
  );
};

export default StartLethe;
