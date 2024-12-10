import { useErrorHandler } from "../context/useErrorHandler";
import { open } from "@tauri-apps/plugin-shell";

const StartLethe = () => {
  const handleError = useErrorHandler();
  async function startLethe() {
    try {
      await open("https://lethelc.site/zwei");
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
