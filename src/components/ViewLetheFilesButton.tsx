import { invoke } from "@tauri-apps/api/core";
import { useErrorHandler } from "../context/useErrorHandler";

const ViewLetheFiles = () => {
  const handleError = useErrorHandler();
  async function viewLetheFiles() {
    try {
      await invoke("open_game_folder");
    } catch (error) {
      handleError(error);
      console.error("Failed to open game folder:", error);
    }
  }
  return (
    <button onClick={viewLetheFiles} className="btn btn-sm btn-ghost">
      View Lethe Files
    </button>
  );
};

export default ViewLetheFiles;
