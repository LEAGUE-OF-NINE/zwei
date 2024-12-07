import { invoke } from "@tauri-apps/api/core";

const StartLethe = () => {
  async function startLethe() {
    try {
      await invoke("launch_game", {
        token:"" //TODO: login logic
      });
    } catch (error) {
      console.error("Failed to clone folder:", error);
    }
  }
  return (
    <button className="btn btn-primary w-full max-w-sm text-xl font-semibold" onClick={startLethe}>
      Launch
    </button>
  );
};

export default StartLethe;
