import { invoke } from "@tauri-apps/api/core";

const StartLethe = () => {
  async function startLethe() {
    try {
      const randomNumber = Math.floor(Math.random() * (6000 - 3000 + 1)) + 3000;
      await invoke("start_login_server", { port: randomNumber });
    } catch (error) {
      console.error("Failed to clone folder:", error);
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
